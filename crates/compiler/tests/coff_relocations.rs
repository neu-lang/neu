//! Windows arm64 objects must reference data through COFF page relocations.
//! Upstream cranelift-object has no mapping for them, so the local patch in
//! `third_party/cranelift-object` supplies it. This test emits a small object
//! for `aarch64-pc-windows-msvc` on any host and checks the relocation types.

use std::str::FromStr;

use cranelift_codegen::{
    ir::{InstBuilder, MemFlagsData, Signature, UserFuncName, types},
    settings,
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{DataDescription, Linkage, Module, default_libcall_names};
use cranelift_object::{ObjectBuilder, ObjectModule};
use object::{Object, ObjectSection, RelocationFlags};
use target_lexicon::Triple;

#[test]
fn windows_arm64_data_references_use_coff_page_relocations() {
    let triple = Triple::from_str("aarch64-pc-windows-msvc").expect("valid triple");
    let isa = cranelift_codegen::isa::lookup(triple)
        .expect("aarch64 windows isa")
        .finish(settings::Flags::new(settings::builder()))
        .expect("isa flags");
    let mut module = ObjectModule::new(
        ObjectBuilder::new(isa, "coff_probe", default_libcall_names()).expect("object builder"),
    );

    let data_id = module
        .declare_data("probe_data", Linkage::Local, false, false)
        .expect("declare data");
    let mut data = DataDescription::new();
    data.define(vec![0u8; 8].into_boxed_slice());
    module.define_data(data_id, &data).expect("define data");

    let call_conv = module.target_config().default_call_conv;
    let mut signature = Signature::new(call_conv);
    signature
        .returns
        .push(cranelift_codegen::ir::AbiParam::new(types::I64));
    let func_id = module
        .declare_function("probe", Linkage::Export, &signature)
        .expect("declare function");

    let mut context = module.make_context();
    context.func.signature = signature;
    context.func.name = UserFuncName::testcase("probe");
    let mut builder_context = FunctionBuilderContext::new();
    {
        let mut builder = FunctionBuilder::new(&mut context.func, &mut builder_context);
        let block = builder.create_block();
        builder.switch_to_block(block);
        let global = module.declare_data_in_func(data_id, builder.func);
        let address = builder.ins().global_value(types::I64, global);
        let value = builder
            .ins()
            .load(types::I64, MemFlagsData::new(), address, 0);
        builder.ins().return_(&[value]);
        builder.seal_all_blocks();
        builder.finalize();
    }
    module
        .define_function(func_id, &mut context)
        .expect("define function");
    module.clear_context(&mut context);

    let bytes = module.finish().emit().expect("emit object");
    let file = object::File::parse(&bytes[..]).expect("parse object");
    let kinds: Vec<RelocationFlags> = file
        .sections()
        .flat_map(|section| {
            section
                .relocations()
                .map(|(_, relocation)| relocation.flags())
                .collect::<Vec<_>>()
        })
        .collect();

    assert!(
        kinds.contains(&RelocationFlags::Coff {
            typ: object::pe::IMAGE_REL_ARM64_PAGEBASE_REL21,
        }),
        "expected a PAGEBASE_REL21 relocation, found {kinds:?}"
    );
    assert!(
        kinds.contains(&RelocationFlags::Coff {
            typ: object::pe::IMAGE_REL_ARM64_PAGEOFFSET_12A,
        }),
        "expected a PAGEOFFSET_12A relocation, found {kinds:?}"
    );
}
