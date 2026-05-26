#!/bin/bash
TESTKIT_ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)/tools/testkit"
source "${TESTKIT_ROOT:?}/lib/core/common.sh"

echo "--- CanonRS Primitive Governance ---"

RESULT=$(python3 "$(dirname $0)/check_primitives.py" 2>&1)
ERRORS=$(echo "$RESULT" | grep "^\[FAIL\]" | grep -oP '\d+')
OK=$(echo "$RESULT" | grep "^\[OK\].*primitives clean" | grep -oP '\d+')

if [ -z "$ERRORS" ] || [ "$ERRORS" = "0" ]; then
    ok "all primitives canonical ($OK clean)"
else
    echo "$RESULT" | grep "^\[ERRO\]" | while read line; do
        fail "primitive violation: $line"
    done
    fail "primitive governance: $ERRORS violations found"
fi

summary
