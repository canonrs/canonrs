#!/bin/bash
TESTKIT_ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)/tools/testkit"
source "${TESTKIT_ROOT:?}/lib/core/common.sh"

echo "--- CanonRS Bundle Architecture ---"

RESULT=$(python3 "$(dirname $0)/check_bundle.py" 2>&1)
ERRORS=$(echo "$RESULT" | grep "^\[FAIL\]" | grep -oP '\d+')
OK=$(echo "$RESULT" | grep "^\[OK\].*Bundle architecture canonical" | wc -l)

if [ -z "$ERRORS" ] || [ "$ERRORS" = "0" ]; then
    ok "$RESULT"
else
    echo "$RESULT" | grep "^\[ERRO\]" | while read line; do
        fail "bundle violation: $line"
    done
    fail "bundle governance: $ERRORS violations found"
fi

summary
