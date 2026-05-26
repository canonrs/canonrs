#!/bin/bash
TESTKIT_ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)/tools/testkit"
source "${TESTKIT_ROOT:?}/lib/core/common.sh"

echo "--- CanonRS CSS Governance ---"

RESULT=$(python3 "$(dirname $0)/check_css.py" 2>&1)
ERRORS=$(echo "$RESULT" | grep "^\[FAIL\]" | grep -oP '\d+')
OK=$(echo "$RESULT" | grep "^\[OK\].*CSS files clean" | grep -oP '\d+')

if [ -z "$ERRORS" ] || [ "$ERRORS" = "0" ]; then
    ok "all CSS files canonical ($OK clean)"
else
    echo "$RESULT" | grep "^\[ERRO\]" | while read line; do
        fail "CSS violation: $line"
    done
    fail "CSS governance: $ERRORS violations found"
fi

summary
