#!/bin/bash
TESTKIT_ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)/tools/testkit"
source "${TESTKIT_ROOT:?}/lib/core/common.sh"

echo "--- CanonRS Preview Governance ---"

RESULT=$(python3 "$(dirname $0)/check_preview.py" 2>&1)
ERRORS=$(echo "$RESULT" | grep "^\[FAIL\]" | grep -oP '\d+')
OK=$(echo "$RESULT" | grep "^\[OK\].*previews clean" | grep -oP '\d+')

if [ -z "$ERRORS" ] || [ "$ERRORS" = "0" ]; then
    ok "all previews canonical ($OK clean)"
else
    echo "$RESULT" | grep "^\[ERRO\]" | while read line; do
        fail "preview violation: $line"
    done
    fail "preview governance: $ERRORS violations found"
fi

summary
