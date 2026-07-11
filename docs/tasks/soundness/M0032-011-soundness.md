# Soundness Report: M0032-011

Decision: pass.

A zero linker status is no longer treated as executable production unless the
requested output is a regular file. The adversarial no-output case prevents a
false success from crossing the link boundary.
