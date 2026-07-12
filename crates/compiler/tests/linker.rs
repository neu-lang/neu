use std::{
    fs,
    path::{Path, PathBuf},
};

use compiler::{linker::LinkInvocation, target_pack::TargetPackManifest};
use cranelift_codegen::{
    ir::{AbiParam, Function, InstBuilder, UserFuncName, types},
    settings,
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{Linkage, Module, default_libcall_names};
use cranelift_object::{ObjectBuilder, ObjectModule};
use target_lexicon::Triple;

fn startup_shim_fixture() -> Vec<u8> {
    let triple = Triple::host();
    let isa_builder = cranelift_codegen::isa::lookup_by_name(&triple.to_string()).unwrap();
    let isa = isa_builder
        .finish(settings::Flags::new(settings::builder()))
        .unwrap();
    let mut module =
        ObjectModule::new(ObjectBuilder::new(isa, "startup", default_libcall_names()).unwrap());
    let mut signature = module.make_signature();
    signature.returns.push(AbiParam::new(types::I64));
    let start = module
        .declare_function("start", Linkage::Export, &signature)
        .unwrap();
    let language_main = module
        .declare_function("neu_lang_main", Linkage::Import, &signature)
        .unwrap();
    let mut context = cranelift_codegen::Context::new();
    context.func = Function::with_name_signature(UserFuncName::user(0, start.as_u32()), signature);
    let mut builder_context = FunctionBuilderContext::new();
    {
        let mut builder = FunctionBuilder::new(&mut context.func, &mut builder_context);
        let block = builder.create_block();
        builder.switch_to_block(block);
        let callee = module.declare_func_in_func(language_main, builder.func);
        let call = builder.ins().call(callee, &[]);
        let result = builder.inst_results(call)[0];
        builder.ins().return_(&[result]);
        builder.seal_block(block);
        builder.finalize();
    }
    module.define_function(start, &mut context).unwrap();
    module.finish().emit().unwrap()
}

fn fixture_root(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("neu-link-plan-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("bin")).unwrap();
    fs::create_dir_all(root.join("runtime")).unwrap();
    fs::write(root.join("bin/lld"), b"lld").unwrap();
    fs::write(root.join("runtime/startup.o"), startup_shim_fixture()).unwrap();
    fs::write(root.join("program.o"), b"object").unwrap();
    root
}

#[cfg(unix)]
fn executable_linker(root: &Path, exit_code: i32) {
    use std::os::unix::fs::PermissionsExt;

    fs::write(
        root.join("bin/lld"),
        format!("#!/bin/sh\nexit {exit_code}\n"),
    )
    .unwrap();
    let mut permissions = fs::metadata(root.join("bin/lld")).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(root.join("bin/lld"), permissions).unwrap();
}

#[cfg(unix)]
fn output_producing_linker(root: &Path) {
    use std::os::unix::fs::PermissionsExt;

    fs::write(
        root.join("bin/lld"),
        "#!/bin/sh\nwhile [ \"$#\" -gt 0 ]; do\n  if [ \"$1\" = \"-o\" ]; then\n    shift\n    touch \"$1\"\n    exit 0\n  fi\n  shift\ndone\nexit 2\n",
    )
    .unwrap();
    let mut permissions = fs::metadata(root.join("bin/lld")).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(root.join("bin/lld"), permissions).unwrap();
}

#[cfg(unix)]
fn executable_program(root: &Path, exit_code: i32) {
    use std::os::unix::fs::PermissionsExt;

    fs::write(
        root.join("program"),
        format!("#!/bin/sh\ntest \"$#\" -eq 0 || exit 99\nexit {exit_code}\n"),
    )
    .unwrap();
    let mut permissions = fs::metadata(root.join("program")).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(root.join("program"), permissions).unwrap();
}

#[test]
fn builds_deterministic_link_invocation_plan() {
    let root = fixture_root("valid");
    let pack = compiler::target_pack::TargetPack::resolve(
        &root,
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/lld",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        )
        .unwrap(),
        Triple::host(),
    )
    .unwrap();

    let plan = LinkInvocation::new(&pack, root.join("program.o"), root.join("program")).unwrap();
    let canonical_root = root.canonicalize().unwrap();

    assert_eq!(plan.program(), pack.linker_path());
    assert_eq!(plan.arguments()[0], "-arch");
    assert_eq!(plan.arguments()[1], "arm64");
    assert_eq!(plan.arguments()[2], "-platform_version");
    assert_eq!(plan.arguments()[3], "macos");
    assert_eq!(plan.arguments()[4], "15.0");
    assert_eq!(plan.arguments()[5], "15.0");
    assert_eq!(plan.arguments()[6], "-o");
    assert_eq!(plan.arguments()[8], "-e");
    assert_eq!(plan.arguments()[9], "_start");
    assert_eq!(
        PathBuf::from(&plan.arguments()[10]),
        canonical_root.join("runtime/startup.o")
    );
    assert_eq!(PathBuf::from(&plan.arguments()[11]), root.join("program.o"));
    assert_eq!(plan.language_entry_symbol(), "neu_lang_main");
    let _ = fs::remove_dir_all(root);
}

#[test]
fn link_plan_rejects_missing_object() {
    let root = fixture_root("missing-object");
    let pack = compiler::target_pack::TargetPack::resolve(
        &root,
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/lld",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        )
        .unwrap(),
        Triple::host(),
    )
    .unwrap();

    assert_eq!(
        LinkInvocation::new(&pack, root.join("missing.o"), root.join("program")),
        Err(compiler::linker::LinkInvocationError::MissingObject)
    );
    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
#[test]
fn executes_resolved_linker_successfully() {
    let root = fixture_root("execute-success");
    output_producing_linker(&root);
    let pack = compiler::target_pack::TargetPack::resolve(
        &root,
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/lld",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        )
        .unwrap(),
        Triple::host(),
    )
    .unwrap();
    let plan = LinkInvocation::new(&pack, root.join("program.o"), root.join("program")).unwrap();

    assert_eq!(plan.execute(), Ok(()));
    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
#[test]
fn reports_linker_non_success() {
    let root = fixture_root("execute-failure");
    executable_linker(&root, 23);
    let pack = compiler::target_pack::TargetPack::resolve(
        &root,
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/lld",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        )
        .unwrap(),
        Triple::host(),
    )
    .unwrap();
    let plan = LinkInvocation::new(&pack, root.join("program.o"), root.join("program")).unwrap();

    assert_eq!(
        plan.execute(),
        Err(compiler::linker::LinkInvocationError::LinkerFailed(Some(
            23
        )))
    );
    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
#[test]
fn reports_linker_launch_failure() {
    let root = fixture_root("execute-unavailable");
    let pack = compiler::target_pack::TargetPack::resolve(
        &root,
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/lld",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        )
        .unwrap(),
        Triple::host(),
    )
    .unwrap();
    let plan = LinkInvocation::new(&pack, root.join("program.o"), root.join("program")).unwrap();
    fs::remove_file(root.join("bin/lld")).unwrap();

    assert_eq!(
        plan.execute(),
        Err(compiler::linker::LinkInvocationError::LinkerUnavailable)
    );
    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
#[test]
fn accepts_linker_output_file() {
    let root = fixture_root("output-present");
    output_producing_linker(&root);
    let pack = compiler::target_pack::TargetPack::resolve(
        &root,
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/lld",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        )
        .unwrap(),
        Triple::host(),
    )
    .unwrap();
    let output = root.join("program");
    let plan = LinkInvocation::new(&pack, root.join("program.o"), &output).unwrap();

    assert_eq!(plan.execute(), Ok(()));
    assert!(output.is_file());
    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
#[test]
fn rejects_linker_success_without_output() {
    let root = fixture_root("output-missing");
    executable_linker(&root, 0);
    let pack = compiler::target_pack::TargetPack::resolve(
        &root,
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/lld",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        )
        .unwrap(),
        Triple::host(),
    )
    .unwrap();
    let plan = LinkInvocation::new(&pack, root.join("program.o"), root.join("program")).unwrap();

    assert_eq!(
        plan.execute(),
        Err(compiler::linker::LinkInvocationError::MissingOutput)
    );
    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
#[test]
fn runs_linked_output_without_arguments() {
    let root = fixture_root("run-success");
    executable_program(&root, 17);
    let pack = compiler::target_pack::TargetPack::resolve(
        &root,
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/lld",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        )
        .unwrap(),
        Triple::host(),
    )
    .unwrap();
    let plan = LinkInvocation::new(&pack, root.join("program.o"), root.join("program")).unwrap();

    assert_eq!(
        plan.run(),
        Ok(compiler::linker::ExecutableRunOutcome::Exited(17))
    );
    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
#[test]
fn reports_unavailable_executable() {
    let root = fixture_root("run-unavailable");
    let pack = compiler::target_pack::TargetPack::resolve(
        &root,
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/lld",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        )
        .unwrap(),
        Triple::host(),
    )
    .unwrap();
    let plan = LinkInvocation::new(&pack, root.join("program.o"), root.join("program")).unwrap();

    assert_eq!(
        plan.run(),
        Err(compiler::linker::ExecutableRunError::Unavailable)
    );
    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
#[test]
fn verifies_valid_bootstrap_exit() {
    let root = fixture_root("verify-valid");
    executable_program(&root, 17);
    let pack = compiler::target_pack::TargetPack::resolve(
        &root,
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/lld",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        )
        .unwrap(),
        Triple::host(),
    )
    .unwrap();
    let plan = LinkInvocation::new(&pack, root.join("program.o"), root.join("program")).unwrap();

    assert_eq!(plan.verify_main_result(17, pack.trap_exit_code()), Ok(()));
    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
#[test]
fn verifies_unsupported_bootstrap_exit() {
    let root = fixture_root("verify-unsupported");
    executable_program(&root, 1);
    let pack = compiler::target_pack::TargetPack::resolve(
        &root,
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/lld",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        )
        .unwrap(),
        Triple::host(),
    )
    .unwrap();
    let plan = LinkInvocation::new(&pack, root.join("program.o"), root.join("program")).unwrap();

    assert_eq!(plan.verify_main_result(300, pack.trap_exit_code()), Ok(()));
    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
#[test]
fn rejects_unexpected_bootstrap_exit() {
    let root = fixture_root("verify-mismatch");
    executable_program(&root, 16);
    let pack = compiler::target_pack::TargetPack::resolve(
        &root,
        TargetPackManifest::new(
            Triple::host(),
            "macho",
            "macho",
            "bin/lld",
            "runtime/startup.o",
            "_start",
            "neu_lang_main",
            1,
        )
        .unwrap(),
        Triple::host(),
    )
    .unwrap();
    let plan = LinkInvocation::new(&pack, root.join("program.o"), root.join("program")).unwrap();

    assert_eq!(
        plan.verify_main_result(17, pack.trap_exit_code()),
        Err(compiler::linker::ExecutableSmokeError::UnexpectedExit {
            expected: 17,
            actual: 16,
        })
    );
    let _ = fs::remove_dir_all(root);
}
