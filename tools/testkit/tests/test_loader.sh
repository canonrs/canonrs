#!/bin/bash
TESTKIT_ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)/tools/testkit"
source "${TESTKIT_ROOT:?}/lib/core/common.sh"

echo "--- CanonRS Bootstrap Loader Governance ---"

RESULT=$(python3 "$(dirname $0)/check_loader.py" 2>&1)
ERRORS=$(echo "$RESULT" | grep "^\[FAIL\]" | grep -oP '\d+')
OK=$(echo "$RESULT" | grep "^\[OK\] Bootstrap" | wc -l)

if [ -z "$ERRORS" ] || [ "$ERRORS" = "0" ]; then
    ok "bootstrap architecture canonical"
else
    echo "$RESULT" | grep "^\[ERRO\]" | while read line; do
        fail "loader violation: $line"
    done
    fail "loader governance: $ERRORS violations found"
fi

summary
