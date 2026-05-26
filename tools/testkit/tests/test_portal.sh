#!/bin/bash
TESTKIT_ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)/tools/testkit"
source "${TESTKIT_ROOT:?}/lib/core/common.sh"

echo "--- CanonRS Portal SSR Safety ---"

RESULT=$(python3 "$(dirname $0)/check_portal.py" 2>&1)
ERRORS=$(echo "$RESULT" | grep "^\[FAIL\]" | grep -oP '\d+')
OK=$(echo "$RESULT" | grep "^\[OK\].*Portal architecture canonical" | wc -l)

if [ -z "$ERRORS" ] || [ "$ERRORS" = "0" ]; then
    ok "$RESULT"
else
    echo "$RESULT" | grep "^\[ERRO\]" | while read line; do
        fail "portal violation: $line"
    done
    fail "portal governance: $ERRORS violations found"
fi

summary
