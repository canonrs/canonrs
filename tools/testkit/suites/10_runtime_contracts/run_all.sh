#!/bin/bash
# 10_runtime_contracts
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PASSED=0; FAILED=0

echo ""
echo "[SUITE] check_runtime_listener_usage.py"
python3 "$DIR/check_runtime_listener_usage.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "=============================="
echo "[10_runtime_contracts] $PASSED passed, $FAILED failed"
exit $FAILED
