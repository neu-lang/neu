#!/usr/bin/env sh
set -eu

set -- examples/current/*.neu
[ "$#" -eq 1 ]
example=$1
rg -q '^package examples\.current$' "$example"
rg -q '^public fun main\(\): Int \{$' "$example"
rg -q 'return 1 [+] 2 [*] 3;' "$example"
! rg -n '(^|[[:space:]])(struct|interface|enum|when|if|import|const|var|String|Bool|Unit|\?|<<|>>|&|\||\^|\*\*)' "$example"
