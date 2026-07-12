# Current Language Examples

Only source that fits the current end-to-end Cranelift bootstrap boundary is
kept here. Every example has one top-level `main` with no parameters and an
`Int` return, and requires no runtime library, printing, or linker runtime
features. The examples cover integer arithmetic, structured control flow,
lexical scopes, classes, class methods, interfaces, and the accepted `Bool`, `Unit`, `Float`,
and `Byte` primitive runtime forms, plus mutable `var` initialization and
reassignment. `cancellation.neu` demonstrates requesting cancellation of a
child task. `spawn.neu`, `await.neu`, `task_results.neu`, and
`cancel_task.neu` exercise the accepted task operations.
`runtime_entry.neu` exercises the suspendable executable entry boundary.
`concurrency_diagnostics.neu` keeps the accepted cancellation path runnable
for diagnostic and adversarial regression coverage.
`channels.neu` exercises bounded FIFO message transfer and typed closure
matching.
`channel_receive.neu` exercises the receive suspension boundary and message
payload extraction.
`producer_consumer.neu` exercises a spawned producer and parent consumer sharing
an opaque channel handle.
`concurrency_interfaces.neu` exercises public interface dispatch over a private
channel-backed implementation.
`concurrency_binary.neu` exercises the host linker path for a compiler-owned
channel runtime operation.

Parser-only, type-checker-only, ownership, pattern, nullable, and unsupported
syntax examples are intentionally omitted until those forms are accepted by
the full backend and executable pipeline.

The `packages/` directory is a virtual project example: `main.neu` imports the
sibling `math` directory with an explicit alias, and every direct `.neu` file
in that directory belongs to the package.
