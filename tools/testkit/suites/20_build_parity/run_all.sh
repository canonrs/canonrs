#!/bin/bash
# 20_build_parity
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PASSED=0; FAILED=0

echo ""
echo "[SUITE] check_build_parity.py"
python3 "$DIR/check_build_parity.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "=============================="
echo "[20_build_parity] $PASSED passed, $FAILED failed"
exit $FAILED
