#!/bin/bash
# 04_browser
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PASSED=0; FAILED=0

echo ""
echo "[SUITE] check_playwright.py"
python3 "$DIR/check_playwright.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_hydration.py"
python3 "$DIR/check_hydration.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_ssr.py"
python3 "$DIR/check_ssr.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_portal.py"
python3 "$DIR/check_portal.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_overlay_topology.py"
python3 "$DIR/check_overlay_topology.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_loader.py"
python3 "$DIR/check_loader.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "=============================="
echo "[04_browser] $PASSED passed, $FAILED failed"
exit $FAILED
