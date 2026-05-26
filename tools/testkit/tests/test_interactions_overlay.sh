#!/bin/bash
TESTKIT_ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)/tools/testkit"
source "${TESTKIT_ROOT:?}/lib/core/common.sh"

echo "--- CanonRS Overlay Runtime Guarantees ---"

RESULT=$(python3 "$(dirname $0)/check_interactions_overlay.py" 2>&1)
ERRORS=$(echo "$RESULT" | grep "^\[FAIL\]" | grep -oP '\d+')

if [ -z "$ERRORS" ] || [ "$ERRORS" = "0" ]; then
    ok "$RESULT"
else
    echo "$RESULT" | grep "^\[ERRO\]" | while read line; do
        fail "violation: $line"
    done
    fail "governance: $ERRORS violations found"
fi

summary
