#!/bin/bash
TESTKIT_ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)/tools/testkit"
source "${TESTKIT_ROOT:?}/lib/core/common.sh"

echo "--- CanonRS UI Governance ---"

RESULT=$(python3 "$(dirname $0)/check_ui.py" 2>&1)
ERRORS=$(echo "$RESULT" | grep "^\[FAIL\]" | grep -oP '\d+')
OK=$(echo "$RESULT" | grep "^\[OK\].*UI files clean" | grep -oP '\d+')

if [ -z "$ERRORS" ] || [ "$ERRORS" = "0" ]; then
    ok "all UI files canonical ($OK clean)"
else
    echo "$RESULT" | grep "^\[ERRO\]" | while read line; do
        fail "UI violation: $line"
    done
    fail "UI governance: $ERRORS violations found"
fi

summary
