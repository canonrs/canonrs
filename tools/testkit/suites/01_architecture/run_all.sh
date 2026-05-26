#!/bin/bash
# 01_architecture
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PASSED=0; FAILED=0

echo ""
echo "[SUITE] check_primitives.py"
python3 "$DIR/check_primitives.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_ui.py"
python3 "$DIR/check_ui.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_boundary.py"
python3 "$DIR/check_boundary.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_css.py"
python3 "$DIR/check_css.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_preview.py"
python3 "$DIR/check_preview.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_bundle.py"
python3 "$DIR/check_bundle.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_runtime.py"
python3 "$DIR/check_runtime.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "=============================="
echo "[01_architecture] $PASSED passed, $FAILED failed"
exit $FAILED
