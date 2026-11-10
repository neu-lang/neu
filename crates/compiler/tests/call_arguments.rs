use std::{fs, process::Command};

use compiler::{
    driver::{SourceDriverOptions, compile_source_to_executable},
    module::{ModuleName, PackageNamespace},
    source::SourceFileId,
};

fn compile_and_run(source: &str, name: &str) -> i32 {
    let workspace =
        std::env::temp_dir().join(format!("neu-call-args-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(14000),
            ModuleName::parse(name).unwrap(),
            PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    let status = Command::new(&executable).status().unwrap();
    let code = status.code().unwrap_or(-1);
    let _ = fs::remove_dir_all(workspace);
    code
}

#[test]
fn call_arguments_survive_inside_a_scope_body() {
    let source = r#"
        func take(value: Int): Int {
            return value;
        }
        suspend func main(): Int {
            scope {
                val done: Int = take(4);
                return done;
            }
            return 0;
        }
    "#;
    assert_eq!(compile_and_run(source, "scope_call"), 4);
}

#[test]
fn call_arguments_survive_before_a_channel_result_match() {
    let source = r#"
        func take(value: Int): Int {
            return value;
        }
        suspend func main(): Int {
            val queue = channel<Int>(1);
            send(queue, 3);
            val done: Int = take(4);
            val r1 = receive(queue);
            val a: Int = when (r1) {
                ChannelResult.Message(value) -> value;
                ChannelResult.Closed -> 9;
            };
            return done * 10 + a;
        }
    "#;
    assert_eq!(compile_and_run(source, "channel_match_call"), 43);
}
