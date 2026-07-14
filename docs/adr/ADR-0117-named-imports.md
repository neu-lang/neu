# ADR-0117: Named imports

Status: Accepted

## Decision

Neu supports file-local named imports in addition to qualified directory
imports:

```neu
import {add, sub} from "./math"
```

The imported declarations are available unqualified in the importing source
file. Named imports do not create a package qualifier, do not re-export names,
and may not use `as` aliases. The compiler resolves the path using the existing
directory-package and dependency rules, then requires every requested name to
be a public top-level declaration in that package.

Names must be unique within a declaration and across named imports in one
source file. A named import may not collide with a local top-level declaration.
Missing or private names are reported as inaccessible imports. Qualified
imports retain their existing alias and collision semantics.
