#!/usr/bin/env bash
TESTKIT_ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)/tools/testkit"
DIR="$(dirname "$0")/tests"

total_pass=0
total_fail=0

run_test() {
    local name="$1" script="$2"
    echo ""
    echo "━━━ $name ━━━"
    if ( bash "$script" ); then
        total_pass=$((total_pass+1))
    else
        total_fail=$((total_fail+1))
    fi
}

run_test "primitives"   "$DIR/test_primitives.sh"
run_test "ui"           "$DIR/test_ui.sh"
run_test "boundary"     "$DIR/test_boundary.sh"
run_test "interactions" "$DIR/test_interactions.sh"
run_test "preview"      "$DIR/test_preview.sh"
run_test "css"          "$DIR/test_css.sh"
run_test "loader"       "$DIR/test_loader.sh"
run_test "portal"       "$DIR/test_portal.sh"
run_test "bundle"       "$DIR/test_bundle.sh"
run_test "runtime"      "$DIR/test_runtime.sh"
run_test "ssr"          "$DIR/test_ssr.sh"
run_test "playwright"        "$DIR/test_playwright.sh"
run_test "interactions_core" "$DIR/test_interactions_core.sh"
run_test "interactions_ovl"  "$DIR/test_interactions_overlay.sh"
run_test "hydration"         "$DIR/test_hydration.sh"

# ── Suites completas ──────────────────────────────────────────────────────────
run_test "suites" "python3 $TESTKIT_ROOT/canonrs/suites/run_suite.py"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  suites passed: $total_pass"
echo "  suites failed: $total_fail"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
[ $total_fail -eq 0 ] && exit 0 || exit 1
