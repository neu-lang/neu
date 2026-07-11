# Soundness Report: M0032-014

Decision: pass.

An object that merely defines the platform entry cannot satisfy the startup
contract unless a relocation targets the manifest language symbol. Missing and
mismatched targets are rejected before linking, and the tests use a real
Cranelift-generated relocation.
