# ADR-0050: Bootstrap Straight-Line Return Diagnostics

Status: Accepted

## Question

How are bootstrap `missing_return` and `unreachable_return` determined and
located before branch analysis exists?

## Competing Designs

1. Count every nested return as a function return.
2. Analyze only returns directly contained by the function body block.
3. Defer all return diagnostics until general control-flow analysis.

## Trade-offs

Nested returns cannot prove an unconditional path. General control flow is
deferred. Direct block order is deterministic and sufficient for the first
executable subset.

## Recommended Choice

For an `Int` function in the bootstrap executable subset, only explicit return
statements directly contained by its function body block participate in
straight-line return analysis. No such return reports `missing_return` at the
function declaration. The first direct return completes the straight-line path;
each later direct return reports `unreachable_return` at that later return.

Returns inside nested blocks, including `if` branches, neither satisfy nor
produce straight-line return diagnostics. They remain deferred executable forms
until branch analysis is accepted. Recovery records no successful return-path
fact for `missing_return` and ignores unreachable returns for later success
facts. Safe suggestions are to add a direct `return <Int expression>;` or
remove/restructure a later direct return.

## Dependencies

- ADR-0015
- ADR-0040
- ADR-0041
- ADR-0042
