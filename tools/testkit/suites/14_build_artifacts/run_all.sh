#!/bin/bash
# 16_build_artifacts
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PASSED=0; FAILED=0

echo ""
echo "[SUITE] check_build_artifacts.py"
python3 "$DIR/check_build_artifacts.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "=============================="
echo "[16_build_artifacts] $PASSED passed, $FAILED failed"
exit $FAILED
