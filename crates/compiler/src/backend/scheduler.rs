//! Cooperative FIFO scheduler for structured tasks (ADR-0101).
//!
//! Every task runs on its own stack and control moves between tasks only at
//! accepted suspension points: `await`, scope exit, and channel operations that
//! must wait for a full or empty channel. Runnable tasks are selected from one
//! FIFO ready queue. Channel waiters sit in per-channel FIFO queues and move to
//! the ready queue when the state they wait for changes. Nothing here creates
//! OS threads.
//!
//! The task, scope, and queue layouts are private to generated code. Each task
//! switches context in one of two ways, chosen by the target OS. On Linux and
//! macOS, tasks use `ucontext` (`getcontext`, `makecontext`, `swapcontext`,
//! `setcontext`) with the struct layout taken from the libc crate. On Windows,
//! tasks are fibers (`CreateFiber`, `SwitchToFiber`, `ConvertThreadToFiber`).
//! Other targets have no scheduler, so spawn and scope fail to compile there.

use cranelift_codegen::{
    ir::{
        AbiParam, ExtFuncData, ExternalName, FuncRef, Function, InstBuilder, MemFlagsData,
        Signature, TrapCode, UserExternalName, UserFuncName, Value, condcodes::IntCC, types,
    },
    isa::CallConv,
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{DataDescription, DataId, FuncId, Linkage, Module};
use cranelift_object::ObjectModule;
use target_lexicon::{OperatingSystem, Triple};

use crate::backend::CraneliftLoweringError;

pub(super) const TASK_DONE: i32 = 0;
pub(super) const TASK_CANCELLED: i32 = 8;
pub(super) const TASK_RESULT: i32 = 16;
const TASK_FRAME: i32 = 24;
const TASK_WAITER: i32 = 32;
const TASK_PARENT_SCOPE: i32 = 40;
const TASK_NEXT: i32 = 48;
const TASK_INNERMOST_SCOPE: i32 = 56;
const TASK_STACK: i32 = 64;
const TASK_CONTEXT: i32 = 72;
const TASK_STACK_BYTES: i64 = 256 * 1024;

/// Where a task's switchable context lives inside its record, and the offsets
/// needed to point a `ucontext_t` at the task's stack.
#[derive(Clone, Copy, Debug)]
struct ContextLayout {
    /// Size of the context stored at `TASK_CONTEXT`.
    bytes: i64,
    /// Offset of `uc_link` inside a `ucontext_t`.
    link: i32,
    /// Offset of the stack pointer inside a `ucontext_t`.
    stack_sp: i32,
    /// Offset of the stack size inside a `ucontext_t`.
    stack_size: i32,
}

/// A fiber context is a single handle, so it needs no stack offsets.
const FIBER_LAYOUT: ContextLayout = ContextLayout {
    bytes: 8,
    link: 0,
    stack_sp: 0,
    stack_size: 0,
};

/// Layout of the host's `ucontext_t`. The compiler emits code only for the host
/// (the driver rejects other targets), so the host's libc struct is the right one.
#[cfg(unix)]
const UCONTEXT_LAYOUT: ContextLayout = ContextLayout {
    bytes: std::mem::size_of::<libc::ucontext_t>() as i64,
    link: std::mem::offset_of!(libc::ucontext_t, uc_link) as i32,
    stack_sp: std::mem::offset_of!(libc::ucontext_t, uc_stack.ss_sp) as i32,
    stack_size: std::mem::offset_of!(libc::ucontext_t, uc_stack.ss_size) as i32,
};

/// Size of a task record for the given context layout.
fn task_bytes(layout: ContextLayout) -> i64 {
    TASK_CONTEXT as i64 + layout.bytes
}

const SCOPE_LIVE: i32 = 0;
const SCOPE_WAITER: i32 = 8;
const SCOPE_PARENT: i32 = 16;
const SCOPE_BYTES: i64 = 24;

const STATE_CURRENT: i32 = 0;
/// The ready queue is stored at this offset in the scheduler state.
const STATE_READY: i32 = 8;
const STATE_BYTES: i64 = STATE_READY as i64 + QUEUE_BYTES;

const QUEUE_HEAD: i32 = 0;
const QUEUE_TAIL: i32 = 8;
/// Size of a FIFO queue header: head pointer then tail pointer.
pub(super) const QUEUE_BYTES: i64 = 16;

pub(super) const TRAP_DEADLOCK: u8 = 40;
pub(super) const TRAP_SPAWN_OUTSIDE_SCOPE: u8 = 41;
pub(super) const TRAP_ALLOCATION: u8 = 42;
const TRAP_UNREACHABLE: u8 = 43;

/// Imported functions that create and switch between task contexts.
#[derive(Clone, Copy, Debug)]
enum ContextFunctions {
    /// Only constructed on Unix hosts, which have libc's `ucontext_t`.
    #[cfg_attr(not(unix), allow(dead_code))]
    Ucontext {
        makecontext: FuncId,
        getcontext: FuncId,
        swapcontext: FuncId,
        setcontext: FuncId,
    },
    Fiber {
        create_fiber: FuncId,
        switch_to_fiber: FuncId,
        convert_thread: FuncId,
    },
}

#[derive(Clone, Copy, Debug)]
pub(super) struct SchedulerFunctions {
    state: DataId,
    malloc: FuncId,
    current: FuncId,
    enqueue: FuncId,
    dequeue: FuncId,
    block: FuncId,
    spawn: FuncId,
    await_task: FuncId,
    finish: FuncId,
    scope_enter: FuncId,
    scope_exit: FuncId,
    wait_on_queue: FuncId,
    wake_one: FuncId,
    wake_all: FuncId,
    context: ContextFunctions,
    layout: ContextLayout,
}

fn signature(call_conv: CallConv, parameters: usize, returns: usize) -> Signature {
    let mut signature = Signature::new(call_conv);
    for _ in 0..parameters {
        signature.params.push(AbiParam::new(types::I64));
    }
    for _ in 0..returns {
        signature.returns.push(AbiParam::new(types::I64));
    }
    signature
}

/// Declares the context functions for `target`, or `None` when the target has no
/// scheduler.
fn declare_context_functions(
    module: &mut ObjectModule,
    call_conv: CallConv,
    target: &Triple,
) -> Result<Option<(ContextFunctions, ContextLayout)>, CraneliftLoweringError> {
    let definition = |_| CraneliftLoweringError::ObjectDefinitionFailed;
    match target.operating_system {
        OperatingSystem::Windows => {
            let create_fiber = module
                .declare_function("CreateFiber", Linkage::Import, &signature(call_conv, 3, 1))
                .map_err(definition)?;
            let switch_to_fiber = module
                .declare_function(
                    "SwitchToFiber",
                    Linkage::Import,
                    &signature(call_conv, 1, 0),
                )
                .map_err(definition)?;
            let convert_thread = module
                .declare_function(
                    "ConvertThreadToFiber",
                    Linkage::Import,
                    &signature(call_conv, 1, 1),
                )
                .map_err(definition)?;
            Ok(Some((
                ContextFunctions::Fiber {
                    create_fiber,
                    switch_to_fiber,
                    convert_thread,
                },
                FIBER_LAYOUT,
            )))
        }
        _ if target.operating_system.is_like_darwin()
            || matches!(target.operating_system, OperatingSystem::Linux) =>
        {
            #[cfg(unix)]
            {
                let makecontext = module
                    .declare_function("makecontext", Linkage::Import, &signature(call_conv, 3, 0))
                    .map_err(definition)?;
                let getcontext = module
                    .declare_function("getcontext", Linkage::Import, &signature(call_conv, 1, 1))
                    .map_err(definition)?;
                let swapcontext = module
                    .declare_function("swapcontext", Linkage::Import, &signature(call_conv, 2, 1))
                    .map_err(definition)?;
                let setcontext = module
                    .declare_function("setcontext", Linkage::Import, &signature(call_conv, 1, 1))
                    .map_err(definition)?;
                Ok(Some((
                    ContextFunctions::Ucontext {
                        makecontext,
                        getcontext,
                        swapcontext,
                        setcontext,
                    },
                    UCONTEXT_LAYOUT,
                )))
            }
            #[cfg(not(unix))]
            {
                let _ = (module, call_conv);
                Ok(None)
            }
        }
        _ => Ok(None),
    }
}

/// Declares and defines the scheduler functions and their state for one object,
/// or returns `None` when `target` has no scheduler.
pub(super) fn declare_scheduler(
    module: &mut ObjectModule,
    malloc: FuncId,
    call_conv: CallConv,
    target: &Triple,
) -> Result<Option<SchedulerFunctions>, CraneliftLoweringError> {
    let definition = |_| CraneliftLoweringError::ObjectDefinitionFailed;
    let Some((context, layout)) = declare_context_functions(module, call_conv, target)? else {
        return Ok(None);
    };
    let state = module
        .declare_data("neu_sched_state", Linkage::Local, true, false)
        .map_err(definition)?;
    let mut state_data = DataDescription::new();
    state_data.define_zeroinit(STATE_BYTES as usize);
    module.define_data(state, &state_data).map_err(definition)?;

    let mut declare = |name: &str, parameters: usize| {
        module
            .declare_function(name, Linkage::Local, &signature(call_conv, parameters, 1))
            .map_err(definition)
    };
    let scheduler = SchedulerFunctions {
        state,
        malloc,
        current: declare("neu_sched_current", 0)?,
        enqueue: declare("neu_sched_enqueue", 2)?,
        dequeue: declare("neu_sched_dequeue", 1)?,
        block: declare("neu_sched_block", 0)?,
        spawn: declare("neu_sched_spawn", 2)?,
        await_task: declare("neu_sched_await", 1)?,
        finish: declare("neu_sched_finish", 0)?,
        scope_enter: declare("neu_sched_scope_enter", 0)?,
        scope_exit: declare("neu_sched_scope_exit", 0)?,
        wait_on_queue: declare("neu_sched_wait_on_queue", 1)?,
        wake_one: declare("neu_sched_wake_one", 1)?,
        wake_all: declare("neu_sched_wake_all", 1)?,
        context,
        layout,
    };
    define_scheduler_bodies(module, &scheduler, call_conv)?;
    Ok(Some(scheduler))
}

fn define_function(
    module: &mut ObjectModule,
    id: FuncId,
    signature: Signature,
    body: impl FnOnce(
        &mut FunctionBuilder<'_>,
        &mut ObjectModule,
        &[Value],
    ) -> Result<(), CraneliftLoweringError>,
) -> Result<(), CraneliftLoweringError> {
    let mut function = Function::with_name_signature(UserFuncName::user(0, id.as_u32()), signature);
    let mut builder_context = FunctionBuilderContext::new();
    {
        let mut builder = FunctionBuilder::new(&mut function, &mut builder_context);
        let entry = builder.create_block();
        builder.append_block_params_for_function_params(entry);
        builder.switch_to_block(entry);
        let parameters = builder.block_params(entry).to_vec();
        body(&mut builder, module, &parameters)?;
        builder.seal_all_blocks();
        builder.finalize();
    }
    let mut context = cranelift_codegen::Context::for_function(function);
    module
        .define_function(id, &mut context)
        .map_err(|_| CraneliftLoweringError::ObjectDefinitionFailed)
}

/// References `function` as a colocated callee, so calls become PC-relative
/// (or PLT) relocations. Absolute references to imported symbols would make the
/// linker emit text relocations for every program that contains the scheduler.
fn callee_ref(
    builder: &mut FunctionBuilder<'_>,
    module: &ObjectModule,
    function: FuncId,
) -> FuncRef {
    let signature = module
        .declarations()
        .get_function_decl(function)
        .signature
        .clone();
    let signature = builder.func.import_signature(signature);
    let user_name = builder
        .func
        .declare_imported_user_function(UserExternalName {
            namespace: 0,
            index: function.as_u32(),
        });
    builder.func.import_function(ExtFuncData {
        name: ExternalName::user(user_name),
        signature,
        colocated: true,
        patchable: false,
    })
}

fn call(
    builder: &mut FunctionBuilder<'_>,
    module: &mut ObjectModule,
    function: FuncId,
    arguments: &[Value],
) -> Result<Option<Value>, CraneliftLoweringError> {
    let function_ref = callee_ref(builder, module, function);
    let instruction = builder.ins().call(function_ref, arguments);
    Ok(builder.inst_results(instruction).first().copied())
}

fn call_value(
    builder: &mut FunctionBuilder<'_>,
    module: &mut ObjectModule,
    function: FuncId,
    arguments: &[Value],
) -> Result<Value, CraneliftLoweringError> {
    call(builder, module, function, arguments)?.ok_or(CraneliftLoweringError::MissingValue)
}

fn load_word(builder: &mut FunctionBuilder<'_>, base: Value, offset: i32) -> Value {
    builder
        .ins()
        .load(types::I64, MemFlagsData::new(), base, offset)
}

fn store_word(builder: &mut FunctionBuilder<'_>, value: Value, base: Value, offset: i32) {
    builder
        .ins()
        .store(MemFlagsData::new(), value, base, offset);
}

fn state_base(
    builder: &mut FunctionBuilder<'_>,
    module: &mut ObjectModule,
    state: DataId,
) -> Value {
    let global = module.declare_data_in_func(state, builder.func);
    builder.ins().global_value(types::I64, global)
}

fn trap_if_zero(builder: &mut FunctionBuilder<'_>, value: Value, code: u8) {
    builder.ins().trapz(value, TrapCode::unwrap_user(code));
}

fn define_scheduler_bodies(
    module: &mut ObjectModule,
    s: &SchedulerFunctions,
    call_conv: CallConv,
) -> Result<(), CraneliftLoweringError> {
    let s = *s;
    let ready = (s.state, STATE_READY);

    // neu_sched_current(): the running task, creating the root task on first use.
    define_function(
        module,
        s.current,
        signature(call_conv, 0, 1),
        move |b, m, _| {
            let st = state_base(b, m, s.state);
            let current = load_word(b, st, STATE_CURRENT);
            let found = b.create_block();
            let create = b.create_block();
            b.ins().brif(current, found, &[], create, &[]);

            b.switch_to_block(found);
            b.ins().return_(&[current]);

            b.switch_to_block(create);
            let size = b.ins().iconst(types::I64, task_bytes(s.layout));
            let task = call_value(b, m, s.malloc, &[size])?;
            trap_if_zero(b, task, TRAP_ALLOCATION);
            let zero = b.ins().iconst(types::I64, 0);
            init_task_fields(b, task, zero, zero, zero);
            emit_root_context(b, m, &s, task)?;
            store_word(b, task, st, STATE_CURRENT);
            b.ins().return_(&[task]);
            Ok(())
        },
    )?;

    // neu_sched_enqueue(queue, task): append task to queue; null tasks are ignored.
    define_function(
        module,
        s.enqueue,
        signature(call_conv, 2, 1),
        move |b, _m, p| {
            let queue = p[0];
            let task = p[1];
            let done = b.create_block();
            let insert = b.create_block();
            let set_head = b.create_block();
            let link = b.create_block();
            let set_tail = b.create_block();
            let null = b.ins().icmp_imm(IntCC::Equal, task, 0);
            b.ins().brif(null, done, &[], insert, &[]);

            b.switch_to_block(insert);
            let zero = b.ins().iconst(types::I64, 0);
            store_word(b, zero, task, TASK_NEXT);
            let tail = load_word(b, queue, QUEUE_TAIL);
            let empty = b.ins().icmp_imm(IntCC::Equal, tail, 0);
            b.ins().brif(empty, set_head, &[], link, &[]);

            b.switch_to_block(set_head);
            store_word(b, task, queue, QUEUE_HEAD);
            b.ins().jump(set_tail, &[]);

            b.switch_to_block(link);
            store_word(b, task, tail, TASK_NEXT);
            b.ins().jump(set_tail, &[]);

            b.switch_to_block(set_tail);
            store_word(b, task, queue, QUEUE_TAIL);
            let zero = b.ins().iconst(types::I64, 0);
            b.ins().return_(&[zero]);

            b.switch_to_block(done);
            let zero = b.ins().iconst(types::I64, 0);
            b.ins().return_(&[zero]);
            Ok(())
        },
    )?;

    // neu_sched_dequeue(queue): remove and return the head task, or 0 when empty.
    define_function(
        module,
        s.dequeue,
        signature(call_conv, 1, 1),
        move |b, _m, p| {
            let queue = p[0];
            let head = load_word(b, queue, QUEUE_HEAD);
            let none = b.create_block();
            let take = b.create_block();
            let empty = b.ins().icmp_imm(IntCC::Equal, head, 0);
            b.ins().brif(empty, none, &[], take, &[]);

            b.switch_to_block(none);
            let zero = b.ins().iconst(types::I64, 0);
            b.ins().return_(&[zero]);

            b.switch_to_block(take);
            let next = load_word(b, head, TASK_NEXT);
            store_word(b, next, queue, QUEUE_HEAD);
            let zero = b.ins().iconst(types::I64, 0);
            store_word(b, zero, head, TASK_NEXT);
            let tail = load_word(b, queue, QUEUE_TAIL);
            let last = b.ins().icmp_imm(IntCC::Equal, next, 0);
            let new_tail = b.ins().select(last, zero, tail);
            store_word(b, new_tail, queue, QUEUE_TAIL);
            b.ins().return_(&[head]);
            Ok(())
        },
    )?;

    // neu_sched_block(): suspend the current task and run the next ready task.
    // The caller has already recorded where it will be woken from.
    define_function(
        module,
        s.block,
        signature(call_conv, 0, 1),
        move |b, m, _| {
            let me = call_value(b, m, s.current, &[])?;
            let ready_queue_addr = ready_queue(b, m, ready);
            let next = call_value(b, m, s.dequeue, &[ready_queue_addr])?;
            let deadlock = b.create_block();
            let switch = b.create_block();
            let none = b.ins().icmp_imm(IntCC::Equal, next, 0);
            b.ins().brif(none, deadlock, &[], switch, &[]);

            b.switch_to_block(deadlock);
            b.ins().trap(TrapCode::unwrap_user(TRAP_DEADLOCK));

            b.switch_to_block(switch);
            let st = state_base(b, m, s.state);
            store_word(b, next, st, STATE_CURRENT);
            emit_switch(b, m, &s, me, next)?;
            let zero = b.ins().iconst(types::I64, 0);
            b.ins().return_(&[zero]);
            Ok(())
        },
    )?;

    // neu_sched_spawn(entry, frame): create a queued child in the innermost scope.
    define_function(
        module,
        s.spawn,
        signature(call_conv, 2, 1),
        move |b, m, p| {
            let entry = p[0];
            let frame = p[1];
            let me = call_value(b, m, s.current, &[])?;
            let scope = load_word(b, me, TASK_INNERMOST_SCOPE);
            let outside = b.create_block();
            let create = b.create_block();
            b.ins().brif(scope, create, &[], outside, &[]);

            b.switch_to_block(outside);
            b.ins()
                .trap(TrapCode::unwrap_user(TRAP_SPAWN_OUTSIDE_SCOPE));

            b.switch_to_block(create);
            let size = b.ins().iconst(types::I64, task_bytes(s.layout));
            let task = call_value(b, m, s.malloc, &[size])?;
            trap_if_zero(b, task, TRAP_ALLOCATION);
            let zero = b.ins().iconst(types::I64, 0);
            init_task_fields(b, task, frame, scope, zero);
            let live = load_word(b, scope, SCOPE_LIVE);
            let live = b.ins().iadd_imm(live, 1);
            store_word(b, live, scope, SCOPE_LIVE);
            emit_child_context(b, m, &s, task, entry, zero)?;
            let ready_queue_addr = ready_queue(b, m, ready);
            call(b, m, s.enqueue, &[ready_queue_addr, task])?;
            b.ins().return_(&[task]);
            Ok(())
        },
    )?;

    // neu_sched_await(task): block until the task completes.
    define_function(
        module,
        s.await_task,
        signature(call_conv, 1, 1),
        move |b, m, p| {
            let task = p[0];
            let me = call_value(b, m, s.current, &[])?;
            let check = b.create_block();
            let wait = b.create_block();
            let exit = b.create_block();
            b.ins().jump(check, &[]);

            b.switch_to_block(check);
            let done = load_word(b, task, TASK_DONE);
            b.ins().brif(done, exit, &[], wait, &[]);

            b.switch_to_block(wait);
            store_word(b, me, task, TASK_WAITER);
            call(b, m, s.block, &[])?;
            b.ins().jump(check, &[]);

            b.switch_to_block(exit);
            let zero = b.ins().iconst(types::I64, 0);
            b.ins().return_(&[zero]);
            Ok(())
        },
    )?;

    // neu_sched_finish(): complete the current task and switch away for good.
    define_function(
        module,
        s.finish,
        signature(call_conv, 0, 1),
        move |b, m, _| {
            let me = call_value(b, m, s.current, &[])?;
            let one = b.ins().iconst(types::I64, 1);
            store_word(b, one, me, TASK_DONE);

            let scope = load_word(b, me, TASK_PARENT_SCOPE);
            let live = load_word(b, scope, SCOPE_LIVE);
            let live = b.ins().iadd_imm(live, -1);
            store_word(b, live, scope, SCOPE_LIVE);
            let waiter = load_word(b, scope, SCOPE_WAITER);
            let zero = b.ins().iconst(types::I64, 0);
            let joined = b.ins().icmp_imm(IntCC::Equal, live, 0);
            let wake = b.ins().select(joined, waiter, zero);
            let kept = b.ins().select(joined, zero, waiter);
            store_word(b, kept, scope, SCOPE_WAITER);
            let ready_queue_addr = ready_queue(b, m, ready);
            let ready_queue_ref = ready_queue_addr;
            call(b, m, s.enqueue, &[ready_queue_ref, wake])?;

            let own_waiter = load_word(b, me, TASK_WAITER);
            call(b, m, s.enqueue, &[ready_queue_ref, own_waiter])?;

            let next = call_value(b, m, s.dequeue, &[ready_queue_ref])?;
            let deadlock = b.create_block();
            let switch = b.create_block();
            let none = b.ins().icmp_imm(IntCC::Equal, next, 0);
            b.ins().brif(none, deadlock, &[], switch, &[]);

            b.switch_to_block(deadlock);
            b.ins().trap(TrapCode::unwrap_user(TRAP_DEADLOCK));

            b.switch_to_block(switch);
            let st = state_base(b, m, s.state);
            store_word(b, next, st, STATE_CURRENT);
            emit_jump(b, m, &s, next)?;
            b.ins().trap(TrapCode::unwrap_user(TRAP_UNREACHABLE));
            Ok(())
        },
    )?;

    // neu_sched_scope_enter(): push a scope owned by the current task.
    define_function(
        module,
        s.scope_enter,
        signature(call_conv, 0, 1),
        move |b, m, _| {
            let me = call_value(b, m, s.current, &[])?;
            let size = b.ins().iconst(types::I64, SCOPE_BYTES);
            let scope = call_value(b, m, s.malloc, &[size])?;
            trap_if_zero(b, scope, TRAP_ALLOCATION);
            let zero = b.ins().iconst(types::I64, 0);
            store_word(b, zero, scope, SCOPE_LIVE);
            store_word(b, zero, scope, SCOPE_WAITER);
            let parent = load_word(b, me, TASK_INNERMOST_SCOPE);
            store_word(b, parent, scope, SCOPE_PARENT);
            store_word(b, scope, me, TASK_INNERMOST_SCOPE);
            b.ins().return_(&[zero]);
            Ok(())
        },
    )?;

    // neu_sched_scope_exit(): join every child registered in the innermost scope.
    define_function(
        module,
        s.scope_exit,
        signature(call_conv, 0, 1),
        move |b, m, _| {
            let me = call_value(b, m, s.current, &[])?;
            let scope = load_word(b, me, TASK_INNERMOST_SCOPE);
            let check = b.create_block();
            let wait = b.create_block();
            let leave = b.create_block();
            b.ins().jump(check, &[]);

            b.switch_to_block(check);
            let live = load_word(b, scope, SCOPE_LIVE);
            b.ins().brif(live, wait, &[], leave, &[]);

            b.switch_to_block(wait);
            store_word(b, me, scope, SCOPE_WAITER);
            call(b, m, s.block, &[])?;
            b.ins().jump(check, &[]);

            b.switch_to_block(leave);
            let zero = b.ins().iconst(types::I64, 0);
            store_word(b, zero, scope, SCOPE_WAITER);
            let parent = load_word(b, scope, SCOPE_PARENT);
            store_word(b, parent, me, TASK_INNERMOST_SCOPE);
            b.ins().return_(&[zero]);
            Ok(())
        },
    )?;

    // neu_sched_wait_on_queue(queue): park the current task on a FIFO wait queue.
    define_function(
        module,
        s.wait_on_queue,
        signature(call_conv, 1, 1),
        move |b, m, p| {
            let me = call_value(b, m, s.current, &[])?;
            call(b, m, s.enqueue, &[p[0], me])?;
            call(b, m, s.block, &[])?;
            let zero = b.ins().iconst(types::I64, 0);
            b.ins().return_(&[zero]);
            Ok(())
        },
    )?;

    // neu_sched_wake_one(queue): move the head waiter to the ready queue.
    define_function(
        module,
        s.wake_one,
        signature(call_conv, 1, 1),
        move |b, m, p| {
            let waiter = call_value(b, m, s.dequeue, &[p[0]])?;
            let ready_queue_addr = ready_queue(b, m, ready);
            call(b, m, s.enqueue, &[ready_queue_addr, waiter])?;
            let zero = b.ins().iconst(types::I64, 0);
            b.ins().return_(&[zero]);
            Ok(())
        },
    )?;

    // neu_sched_wake_all(queue): move every waiter to the ready queue in FIFO order.
    define_function(
        module,
        s.wake_all,
        signature(call_conv, 1, 1),
        move |b, m, p| {
            let queue = p[0];
            let check = b.create_block();
            let move_one = b.create_block();
            let exit = b.create_block();
            b.ins().jump(check, &[]);

            b.switch_to_block(check);
            let waiter = call_value(b, m, s.dequeue, &[queue])?;
            b.ins().brif(waiter, move_one, &[], exit, &[]);

            b.switch_to_block(move_one);
            let ready_queue_addr = ready_queue(b, m, ready);
            call(b, m, s.enqueue, &[ready_queue_addr, waiter])?;
            b.ins().jump(check, &[]);

            b.switch_to_block(exit);
            let zero = b.ins().iconst(types::I64, 0);
            b.ins().return_(&[zero]);
            Ok(())
        },
    )?;

    Ok(())
}

/// Address of the ready queue inside the scheduler state.
fn ready_queue(
    builder: &mut FunctionBuilder<'_>,
    module: &mut ObjectModule,
    (state, offset): (DataId, i32),
) -> Value {
    let base = state_base(builder, module, state);
    builder.ins().iadd_imm(base, offset as i64)
}

fn init_task_fields(
    builder: &mut FunctionBuilder<'_>,
    task: Value,
    frame: Value,
    parent_scope: Value,
    innermost_scope: Value,
) {
    let zero = builder.ins().iconst(types::I64, 0);
    store_word(builder, zero, task, TASK_DONE);
    store_word(builder, zero, task, TASK_CANCELLED);
    store_word(builder, zero, task, TASK_RESULT);
    store_word(builder, frame, task, TASK_FRAME);
    store_word(builder, zero, task, TASK_WAITER);
    store_word(builder, parent_scope, task, TASK_PARENT_SCOPE);
    store_word(builder, zero, task, TASK_NEXT);
    store_word(builder, innermost_scope, task, TASK_INNERMOST_SCOPE);
    store_word(builder, zero, task, TASK_STACK);
}

/// Makes the root task's context refer to the thread that runs the program.
/// Fibers need the thread converted once before any switch; ucontext saves the
/// running context into the root record on the first switch.
fn emit_root_context(
    builder: &mut FunctionBuilder<'_>,
    module: &mut ObjectModule,
    scheduler: &SchedulerFunctions,
    root: Value,
) -> Result<(), CraneliftLoweringError> {
    if let ContextFunctions::Fiber { convert_thread, .. } = scheduler.context {
        let zero = builder.ins().iconst(types::I64, 0);
        let fiber = call_value(builder, module, convert_thread, &[zero])?;
        trap_if_zero(builder, fiber, TRAP_ALLOCATION);
        store_word(builder, fiber, root, TASK_CONTEXT);
    }
    Ok(())
}

/// Gives a new child task its context, so the first switch to it runs `entry`
/// on the child's own stack.
fn emit_child_context(
    builder: &mut FunctionBuilder<'_>,
    module: &mut ObjectModule,
    scheduler: &SchedulerFunctions,
    task: Value,
    entry: Value,
    zero: Value,
) -> Result<(), CraneliftLoweringError> {
    let stack_bytes = builder.ins().iconst(types::I64, TASK_STACK_BYTES);
    match scheduler.context {
        ContextFunctions::Ucontext {
            makecontext,
            getcontext,
            ..
        } => {
            let stack = call_value(builder, module, scheduler.malloc, &[stack_bytes])?;
            trap_if_zero(builder, stack, TRAP_ALLOCATION);
            store_word(builder, stack, task, TASK_STACK);
            let context = builder.ins().iadd_imm(task, TASK_CONTEXT as i64);
            call(builder, module, getcontext, &[context])?;
            let layout = scheduler.layout;
            store_word(builder, stack, context, layout.stack_sp);
            store_word(builder, stack_bytes, context, layout.stack_size);
            store_word(builder, zero, context, layout.link);
            call(builder, module, makecontext, &[context, entry, zero])?;
        }
        ContextFunctions::Fiber { create_fiber, .. } => {
            let fiber = call_value(builder, module, create_fiber, &[stack_bytes, entry, zero])?;
            trap_if_zero(builder, fiber, TRAP_ALLOCATION);
            store_word(builder, fiber, task, TASK_CONTEXT);
        }
    }
    Ok(())
}

/// Saves the running task and resumes `to`. Returns when something switches back.
fn emit_switch(
    builder: &mut FunctionBuilder<'_>,
    module: &mut ObjectModule,
    scheduler: &SchedulerFunctions,
    from: Value,
    to: Value,
) -> Result<(), CraneliftLoweringError> {
    match scheduler.context {
        ContextFunctions::Ucontext { swapcontext, .. } => {
            let from_context = builder.ins().iadd_imm(from, TASK_CONTEXT as i64);
            let to_context = builder.ins().iadd_imm(to, TASK_CONTEXT as i64);
            call(builder, module, swapcontext, &[from_context, to_context])?;
        }
        ContextFunctions::Fiber {
            switch_to_fiber, ..
        } => {
            let fiber = load_word(builder, to, TASK_CONTEXT);
            call(builder, module, switch_to_fiber, &[fiber])?;
        }
    }
    Ok(())
}

/// Resumes `to` without saving the running task. Never returns.
fn emit_jump(
    builder: &mut FunctionBuilder<'_>,
    module: &mut ObjectModule,
    scheduler: &SchedulerFunctions,
    to: Value,
) -> Result<(), CraneliftLoweringError> {
    match scheduler.context {
        ContextFunctions::Ucontext { setcontext, .. } => {
            let to_context = builder.ins().iadd_imm(to, TASK_CONTEXT as i64);
            call(builder, module, setcontext, &[to_context])?;
        }
        ContextFunctions::Fiber {
            switch_to_fiber, ..
        } => {
            let fiber = load_word(builder, to, TASK_CONTEXT);
            call(builder, module, switch_to_fiber, &[fiber])?;
        }
    }
    Ok(())
}

/// Emits a spawn: allocates the child, queues it in the innermost scope, and
/// returns its handle. `entry` is the per-site trampoline for the child body.
pub(super) fn emit_spawn(
    builder: &mut FunctionBuilder<'_>,
    module: &mut ObjectModule,
    scheduler: &SchedulerFunctions,
    entry: FuncId,
    frame: Value,
) -> Result<Value, CraneliftLoweringError> {
    let entry_ref = module.declare_func_in_func(entry, builder.func);
    let entry_address = builder.ins().func_addr(types::I64, entry_ref);
    call_value(builder, module, scheduler.spawn, &[entry_address, frame])
}

pub(super) fn emit_await(
    builder: &mut FunctionBuilder<'_>,
    module: &mut ObjectModule,
    scheduler: &SchedulerFunctions,
    task: Value,
) -> Result<(), CraneliftLoweringError> {
    call(builder, module, scheduler.await_task, &[task])?;
    Ok(())
}

pub(super) fn emit_scope_enter(
    builder: &mut FunctionBuilder<'_>,
    module: &mut ObjectModule,
    scheduler: &SchedulerFunctions,
) -> Result<(), CraneliftLoweringError> {
    call(builder, module, scheduler.scope_enter, &[])?;
    Ok(())
}

pub(super) fn emit_scope_exit(
    builder: &mut FunctionBuilder<'_>,
    module: &mut ObjectModule,
    scheduler: &SchedulerFunctions,
) -> Result<(), CraneliftLoweringError> {
    call(builder, module, scheduler.scope_exit, &[])?;
    Ok(())
}

/// Parks the current task on `queue` until a wake call moves it back to ready.
pub(super) fn emit_wait_on_queue(
    builder: &mut FunctionBuilder<'_>,
    module: &mut ObjectModule,
    scheduler: &SchedulerFunctions,
    queue: Value,
) -> Result<(), CraneliftLoweringError> {
    call(builder, module, scheduler.wait_on_queue, &[queue])?;
    Ok(())
}

pub(super) fn emit_wake_one(
    builder: &mut FunctionBuilder<'_>,
    module: &mut ObjectModule,
    scheduler: &SchedulerFunctions,
    queue: Value,
) -> Result<(), CraneliftLoweringError> {
    call(builder, module, scheduler.wake_one, &[queue])?;
    Ok(())
}

pub(super) fn emit_wake_all(
    builder: &mut FunctionBuilder<'_>,
    module: &mut ObjectModule,
    scheduler: &SchedulerFunctions,
    queue: Value,
) -> Result<(), CraneliftLoweringError> {
    call(builder, module, scheduler.wake_all, &[queue])?;
    Ok(())
}

/// Defines the trampoline a new task starts in. It loads the captures from the
/// task frame, calls the child body, stores the result, and finishes the task.
/// A task cancelled before it starts skips the body.
pub(super) fn define_task_entry(
    module: &mut ObjectModule,
    scheduler: &SchedulerFunctions,
    name: &str,
    body: FuncId,
    parameters: &[types::Type],
    result: Option<types::Type>,
    call_conv: CallConv,
) -> Result<FuncId, CraneliftLoweringError> {
    let entry_signature = signature(call_conv, 0, 0);
    let entry = module
        .declare_function(name, Linkage::Local, &entry_signature)
        .map_err(|_| CraneliftLoweringError::ObjectDefinitionFailed)?;
    let scheduler = *scheduler;
    let parameters = parameters.to_vec();
    define_function(module, entry, entry_signature, move |b, m, _| {
        let me = call_value(b, m, scheduler.current, &[])?;
        let cancelled = load_word(b, me, TASK_CANCELLED);
        let run = b.create_block();
        let finish = b.create_block();
        b.ins().brif(cancelled, finish, &[], run, &[]);

        b.switch_to_block(run);
        let frame = load_word(b, me, TASK_FRAME);
        let mut arguments = Vec::with_capacity(parameters.len());
        for (index, ty) in parameters.iter().enumerate() {
            arguments.push(
                b.ins()
                    .load(*ty, MemFlagsData::new(), frame, (index * 8) as i32),
            );
        }
        let body_ref = callee_ref(b, m, body);
        let instruction = b.ins().call(body_ref, &arguments);
        if result.is_some() {
            let value = b
                .inst_results(instruction)
                .first()
                .copied()
                .ok_or(CraneliftLoweringError::MissingValue)?;
            b.ins().store(MemFlagsData::new(), value, me, TASK_RESULT);
        }
        b.ins().jump(finish, &[]);

        b.switch_to_block(finish);
        call(b, m, scheduler.finish, &[])?;
        b.ins().trap(TrapCode::unwrap_user(TRAP_UNREACHABLE));
        Ok(())
    })?;
    Ok(entry)
}
