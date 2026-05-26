#!/bin/bash
# 05_behavior
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PASSED=0; FAILED=0

echo ""
echo "[SUITE] check_behavior.py"
python3 "$DIR/check_behavior.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_canvas.py"
python3 "$DIR/check_canvas.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_runtime_alive.py"
python3 "$DIR/check_runtime_alive.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_replay.py"
python3 "$DIR/check_replay.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_interaction_connectivity.py"
python3 "$DIR/check_interaction_connectivity.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_runtime_bootstrap.py"
python3 "$DIR/check_runtime_bootstrap.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_capability_loading.py"
python3 "$DIR/check_capability_loading.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_runtime_replay_safety.py"
python3 "$DIR/check_runtime_replay_safety.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_runtime_artifacts.py"
python3 "$DIR/check_runtime_artifacts.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_hot_reload.py"
python3 "$DIR/check_hot_reload.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_listener_topology.py"
python3 "$DIR/check_listener_topology.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_ownership_graph.py"
python3 "$DIR/check_ownership_graph.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "[SUITE] check_mutation_storm.py"
python3 "$DIR/check_mutation_storm.py" 2>&1
if [ $? -eq 0 ]; then PASSED=$((PASSED+1)); else FAILED=$((FAILED+1)); fi

echo ""
echo "=============================="
echo "[05_behavior] $PASSED passed, $FAILED failed"
exit $FAILED
