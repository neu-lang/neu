# Soundness Report: M0032-015

Decision: pass.

Only a process exit matching the accepted bootstrap mapping can pass the
smoke. Out-of-range language results use the target-pack failure status, and
the runner never treats signal termination or a mismatched status as success.
