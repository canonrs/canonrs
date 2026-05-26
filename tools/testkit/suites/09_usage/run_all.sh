#!/bin/bash
# 09_usage
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PASSED=0; FAILED=0

echo ""
echo "[SUITE] check_input_usage.py"
python3 "$DIR/check_input_usage.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_dialog_usage.py"
python3 "$DIR/check_dialog_usage.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_confirm_dialog_usage.py"
python3 "$DIR/check_confirm_dialog_usage.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_canvas_bridge_usage.py"
python3 "$DIR/check_canvas_bridge_usage.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "=============================="
echo "[09_usage] $PASSED passed, $FAILED failed"
exit $FAILED
