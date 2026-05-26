#!/bin/bash
TESTKIT_ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)/tools/testkit"
source "${TESTKIT_ROOT:?}/lib/core/common.sh"

echo "--- CanonRS Browser Runtime ---"

RESULT=$(python3 "$(dirname $0)/check_playwright.py" 2>&1)
ERRORS=$(echo "$RESULT" | grep "^\[FAIL\]" | grep -oP '\d+')
OK=$(echo "$RESULT" | grep "^\[OK\].*Browser runtime canonical" | wc -l)

if [ -z "$ERRORS" ] || [ "$ERRORS" = "0" ]; then
    ok "$RESULT"
else
    echo "$RESULT" | grep "^\[ERRO\]" | while read line; do
        fail "browser violation: $line"
    done
    fail "browser governance: $ERRORS violations found"
fi

summary
