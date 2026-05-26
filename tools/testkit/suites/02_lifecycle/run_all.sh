#!/bin/bash
# 02_lifecycle
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PASSED=0; FAILED=0

echo ""
echo "[SUITE] check_init_governance.py"
python3 "$DIR/check_init_governance.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_capability_chain.py"
python3 "$DIR/check_capability_chain.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_ssr_runtime_parity.py"
python3 "$DIR/check_ssr_runtime_parity.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_mutation_governance.py"
python3 "$DIR/check_mutation_governance.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_data_contracts.py"
python3 "$DIR/check_data_contracts.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "=============================="
echo "[02_lifecycle] $PASSED passed, $FAILED failed"
exit $FAILED
