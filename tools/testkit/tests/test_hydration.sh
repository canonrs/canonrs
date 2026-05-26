#!/bin/bash
TESTKIT_ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)/tools/testkit"
source "${TESTKIT_ROOT:?}/lib/core/common.sh"

echo "--- CanonRS Hydration Governance ---"

RESULT=$(python3 "$(dirname $0)/check_hydration.py" 2>&1)
ERRORS=$(echo "$RESULT" | grep "^\[FAIL\]" | grep -oP '\d+')

if [ -z "$ERRORS" ] || [ "$ERRORS" = "0" ]; then
    ok "$RESULT"
else
    echo "$RESULT" | grep "^\[FAIL\]" | while read line; do
        fail "hydration violation: $line"
    done
    fail "hydration governance: $ERRORS violations found"
fi

summary
