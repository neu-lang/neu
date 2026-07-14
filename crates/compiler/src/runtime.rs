use std::{
    future::Future,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
};

use target_lexicon::Triple;
use tokio::{sync::Notify, task::JoinHandle};

pub const RUNTIME_ABI_VERSION: &str = "neu-runtime-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeBoundaryError {
    UnsupportedTarget(Triple),
    InitializationFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeTaskError {
    Cancelled,
    Failed,
}

pub struct Task<T> {
    join: Option<JoinHandle<Result<T, RuntimeTaskError>>>,
    cancellation: Arc<Notify>,
    state: Arc<TaskState>,
}

impl<T> Task<T> {
    pub fn cancel(&self) {
        self.cancellation.notify_one();
    }
}

impl<T> Drop for Task<T> {
    fn drop(&mut self) {
        self.cancel();
    }
}

struct TaskState {
    completed: AtomicBool,
    completion: Notify,
    cancellation: Arc<Notify>,
    failure: Mutex<Option<RuntimeTaskError>>,
    scope: Option<Arc<ScopeState>>,
}

struct ScopeState {
    progress: Notify,
    failure: Notify,
}

pub struct RuntimeScope<'runtime> {
    runtime: &'runtime RuntimeBoundary,
    children: Vec<Arc<TaskState>>,
    state: Arc<ScopeState>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeSymbol {
    name: &'static str,
    signature: &'static str,
}

impl RuntimeSymbol {
    pub const fn new(name: &'static str, signature: &'static str) -> Self {
        Self { name, signature }
    }

    pub const fn name(self) -> &'static str {
        self.name
    }

    pub const fn signature(self) -> &'static str {
        self.signature
    }
}

pub const RUNTIME_SYMBOLS: &[RuntimeSymbol] = &[
    RuntimeSymbol::new("neu_runtime_init", "() -> Void"),
    RuntimeSymbol::new("neu_runtime_shutdown", "() -> Void"),
    RuntimeSymbol::new("neu_runtime_task_spawn", "(opaque) -> opaque"),
    RuntimeSymbol::new("neu_runtime_task_await", "(opaque) -> opaque"),
    RuntimeSymbol::new("neu_runtime_task_cancel", "(opaque) -> Void"),
];

pub struct RuntimeBoundary {
    runtime: tokio::runtime::Runtime,
}

impl RuntimeBoundary {
    pub fn new(target: Triple) -> Result<Self, RuntimeBoundaryError> {
        if target != Triple::host() {
            return Err(RuntimeBoundaryError::UnsupportedTarget(target));
        }
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|_| RuntimeBoundaryError::InitializationFailed)?;
        Ok(Self { runtime })
    }

    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future,
    {
        self.runtime.block_on(future)
    }

    pub fn shutdown(self) {
        self.runtime.shutdown_background();
    }

    pub fn scope(&self) -> RuntimeScope<'_> {
        RuntimeScope {
            runtime: self,
            children: Vec::new(),
            state: Arc::new(ScopeState {
                progress: Notify::new(),
                failure: Notify::new(),
            }),
        }
    }

    pub fn spawn<F, T>(&self, future: F) -> Task<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        self.spawn_with_scope(future, None)
    }

    fn spawn_with_scope<F, T>(&self, future: F, scope: Option<Arc<ScopeState>>) -> Task<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let cancellation = Arc::new(Notify::new());
        let task_cancellation = Arc::clone(&cancellation);
        let state = Arc::new(TaskState {
            completed: AtomicBool::new(false),
            completion: Notify::new(),
            cancellation: Arc::clone(&cancellation),
            failure: Mutex::new(None),
            scope,
        });
        let worker = self.runtime.spawn(async move {
            tokio::select! {
                value = future => Ok(value),
                _ = task_cancellation.notified() => Err(RuntimeTaskError::Cancelled),
            }
        });
        let task_state = Arc::clone(&state);
        let join = self.runtime.spawn(async move {
            let result = worker.await.unwrap_or(Err(RuntimeTaskError::Failed));
            if let Err(error) = &result {
                *task_state.failure.lock().unwrap() = Some(*error);
                if let Some(scope) = &task_state.scope {
                    scope.failure.notify_waiters();
                }
            }
            task_state.completed.store(true, Ordering::Release);
            task_state.completion.notify_waiters();
            if let Some(scope) = &task_state.scope {
                scope.progress.notify_waiters();
            }
            result
        });
        Task {
            join: Some(join),
            cancellation,
            state,
        }
    }

    pub fn await_task<T>(&self, mut task: Task<T>) -> Result<T, RuntimeTaskError>
    where
        T: Send + 'static,
    {
        let join = task.join.take().expect("task handle has not been awaited");
        self.block_on(async move { join.await.unwrap_or(Err(RuntimeTaskError::Failed)) })
    }

    pub fn cancel_task<T>(&self, task: &Task<T>) {
        task.cancel();
    }
}

impl RuntimeScope<'_> {
    pub fn spawn<F, T>(&mut self, future: F) -> Task<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let task = self
            .runtime
            .spawn_with_scope(future, Some(Arc::clone(&self.state)));
        self.children.push(Arc::clone(&task.state));
        task
    }

    pub fn await_task<T>(&self, task: Task<T>) -> Result<T, RuntimeTaskError>
    where
        T: Send + 'static,
    {
        self.runtime.await_task(task)
    }

    pub fn cancel_children(&self) {
        for child in &self.children {
            if !child.completed.load(Ordering::Acquire) {
                child.cancellation.notify_one();
            }
        }
    }

    pub fn active_children(&self) -> usize {
        self.children
            .iter()
            .filter(|child| !child.completed.load(Ordering::Acquire))
            .count()
    }

    pub fn finish(&self) -> Result<(), RuntimeTaskError> {
        let children = self.children.clone();
        let scope = Arc::clone(&self.state);
        self.runtime.block_on(async move {
            let mut first_failure = None;
            loop {
                if first_failure.is_none() {
                    first_failure = children
                        .iter()
                        .find_map(|child| *child.failure.lock().unwrap());
                    if first_failure.is_some() {
                        for child in &children {
                            if !child.completed.load(Ordering::Acquire) {
                                child.cancellation.notify_one();
                            }
                        }
                    }
                }
                if children
                    .iter()
                    .all(|child| child.completed.load(Ordering::Acquire))
                {
                    return first_failure.map_or(Ok(()), Err);
                }
                let progress = scope.progress.notified();
                let failure = scope.failure.notified();
                tokio::select! {
                    _ = progress => {},
                    _ = failure => {},
                }
            }
        })
    }
}
