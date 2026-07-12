# ADR-0100: Host-Only System Linking

Status: Accepted

## Question

How does the initial Neu driver turn Cranelift object output into an executable,
and what target portability is guaranteed?

## Competing Designs

- Bundle target-specific linkers and startup objects in the repository.
- Invoke a host system linker for host builds and reject non-host targets.
- Require an externally configured linker for every target.
- Emit only object files and defer executable linking.

## Trade-offs

Bundled artifacts would freeze target ABI and runtime decisions prematurely.
Requiring a linker for every target complicates the initial host workflow.
Object-only output would not satisfy the first executable contract.

Host-only system linking keeps the first executable path explicit and small. It
depends on a host C toolchain, while portability is added later as a deliberate
architectural decision.

## Recommended Choice

Neu emits a Cranelift object for the current host triple and invokes the host
system C linker, `cc`, to produce an executable. `NEU_LINKER` may override the
linker command. The driver rejects every non-host target before frontend or
backend work with an explicit diagnostic.

The initial executable has no bundled linker, startup object, target-pack
registry, or foreign-target test contract. The compiler-owned runtime remains
an internal lowering boundary; no allocator, object layout, startup ABI, or FFI
surface becomes public. Language `main(): Int` remains the executable contract.

## Downstream Consequences

- Cranelift object generation remains supported for the host target.
- Host `cc` availability is a build prerequisite, with `NEU_LINKER` as an
  explicit override.
- Non-host requests fail deterministically before object emission.
- Bundled linker/startup artifacts and target-pack registry APIs are removed.
- Cross compilation requires a future accepted ABI, object, runtime, and
  distribution decision.
- CI validates host linking and explicit non-host rejection.

## Dependencies

- Supersedes the linker and target portability portions of ADR-0020,
  ADR-0046, ADR-0047, ADR-0057, and ADR-0058.
- Reconciles target assumptions referenced by ADR-0042, ADR-0064, ADR-0066,
  and ADR-0097.
- Preserves compiler-private runtime boundaries from ADR-0041 and ADR-0059.
