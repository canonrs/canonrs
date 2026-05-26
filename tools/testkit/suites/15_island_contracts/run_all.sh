#!/bin/bash
# 15_island_contracts
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PASSED=0; FAILED=0

echo ""
echo "[SUITE] check_island_contracts.py"
python3 "$DIR/check_island_contracts.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "=============================="
echo "[15_island_contracts] $PASSED passed, $FAILED failed"
exit $FAILED
