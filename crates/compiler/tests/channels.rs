use std::{fs, process::Command};

use compiler::{
    driver::{SourceDriverOptions, compile_source_to_executable},
    module::{ModuleName, PackageNamespace},
    source::SourceFileId,
};

fn compile_and_run(source: &str, name: &str) -> i32 {
    let workspace = std::env::temp_dir().join(format!("neu-channel-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&workspace);
    fs::create_dir_all(&workspace).unwrap();
    let executable = workspace.join("program");
    let output = compile_source_to_executable(
        source,
        SourceDriverOptions::new(
            SourceFileId::from_raw(12000),
            ModuleName::parse(name).unwrap(),
            PackageNamespace::root(),
            &executable,
        ),
    )
    .unwrap();
    let status = Command::new(output).status().unwrap();
    let code = status.code().unwrap_or(-1);
    let _ = fs::remove_dir_all(workspace);
    code
}

#[test]
fn bounded_channel_preserves_fifo_and_message_payloads() {
    let source = r#"
        public func main(): Int {
            val queue = channel<Int>(2);
            send(queue, 3);
            send(queue, 4);
            val first = receive(queue);
            val second = receive(queue);
            close(queue);
            val first_value: Int = when (first) {
                ChannelResult.Message(value) -> value;
                ChannelResult.Closed -> 0;
            };
            val second_value: Int = when (second) {
                ChannelResult.Message(value) -> value;
                ChannelResult.Closed -> 0;
            };
            return first_value + second_value;
        }
    "#;
    assert_eq!(compile_and_run(source, "channel_fifo"), 7);
}

#[test]
fn closed_channel_returns_closed_result_after_draining() {
    let source = r#"
        public func main(): Int {
            val queue = channel<Int>(1);
            close(queue);
            val result = receive(queue);
            return when (result) {
                ChannelResult.Message(value) -> value;
                ChannelResult.Closed -> 9;
            };
        }
    "#;
    assert_eq!(compile_and_run(source, "channel_closed"), 9);
}

#[test]
fn sending_a_move_only_value_consumes_the_source_binding() {
    let output = std::env::temp_dir().join(format!("neu-channel-send-move-{}", std::process::id()));
    let error = compiler::driver::compile_source_to_executable(
        r#"
            public func main(): Int {
                val queue = channel<String>(1);
                val message: String = "payload";
                send(queue, message);
                return message.length;
            }
        "#,
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(12001),
            ModuleName::parse("channel_send_move").unwrap(),
            PackageNamespace::root(),
            &output,
        ),
    )
    .unwrap_err();
    assert!(format!("{error:?}").contains("UseAfterMove"), "{error:?}");
}

#[test]
fn sending_a_string_uses_the_opaque_channel_storage() {
    let source = r#"
        public func main(): Int {
            val queue = channel<String>(1);
            val message: String = "payload";
            send(queue, message);
            return 7;
        }
    "#;
    assert_eq!(compile_and_run(source, "channel_string"), 7);
}

#[test]
fn sending_a_copyable_value_leaves_the_source_usable() {
    let source = r#"
        public func main(): Int {
            val queue = channel<Int>(1);
            val value = 7;
            send(queue, value);
            return value;
        }
    "#;
    assert_eq!(compile_and_run(source, "channel_copy"), 7);
}

#[test]
fn channel_rejects_non_integer_capacity() {
    let output = std::env::temp_dir().join(format!("neu-channel-capacity-{}", std::process::id()));
    let error = compiler::driver::compile_source_to_executable(
        "public func main(): Int { val queue = channel<Int>(true); return 0; }",
        compiler::driver::SourceDriverOptions::new(
            SourceFileId::from_raw(12002),
            ModuleName::parse("channel_capacity").unwrap(),
            PackageNamespace::root(),
            &output,
        ),
    )
    .unwrap_err();
    assert!(
        format!("{error:?}").contains("InvalidChannelOperation"),
        "{error:?}"
    );
}

#[test]
fn closed_channel_can_be_received_repeatedly_without_messages() {
    let source = r#"
        public func main(): Int {
            val queue = channel<Int>(1);
            close(queue);
            val first = receive(queue);
            val second = receive(queue);
            val first_value: Int = when (first) {
                ChannelResult.Message(value) -> value;
                ChannelResult.Closed -> 2;
            };
            val second_value: Int = when (second) {
                ChannelResult.Message(value) -> value;
                ChannelResult.Closed -> 3;
            };
            return first_value + second_value;
        }
    "#;
    assert_eq!(compile_and_run(source, "channel_repeated_closed"), 5);
}
