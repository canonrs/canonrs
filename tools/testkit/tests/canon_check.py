#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""
canon_check.py — Component-scoped development pipeline v2

Usage:
  python3 canon_check.py chart
  python3 canon_check.py data_table --full
  python3 canon_check.py dialog --verbose
  python3 canon_check.py --ci
  python3 canon_check.py --from preview chart
"""
import sys, os, subprocess, glob, re

TESTS_DIR     = os.path.dirname(os.path.abspath(__file__))
_SUITES_DIR   = os.path.join(TESTS_DIR, "..", "suites")

def _find_checker(checker):
    for suite in sorted(os.listdir(_SUITES_DIR)):
        c = os.path.join(_SUITES_DIR, suite, checker)
        if os.path.exists(c): return os.path.dirname(c)
    return TESTS_DIR
PRIMITIVES_DIR = _CANONRS_ROOT + "/canonrs-core/src/primitives"
INT_BASE      = _CANONRS_ROOT + ""

# interaction group name -> checker file
IX_GROUP_MAP = {
    "data":      "check_interactions_data.py",
    "overlay":   "check_interactions_overlay.py",
    "nav":       "check_interactions_core.py",
    "gesture":   "check_interactions_core.py",
    "selection": "check_interactions_core.py",
    "content":   "check_interactions_core.py",
    "init":      "check_interactions_core.py",
}
DEFAULT_INTERACTION = "check_interactions_core.py"

# timeout per checker (seconds)
CHECKER_TIMEOUTS = {
    "check_playwright.py": 120,
    "check_replay.py":      180,
    "check_behavior.py":    180,
    "check_canvas.py":       60,
    "check_runtime_alive.py": 120,
    "check_interaction_connectivity.py": 120,
    "check_chaos.py":       180,
    "check_observer_graph.py": 120,
    "check_contracts_runtime.py": 300,
    "check_hydration.py":   60,
    "check_ssr.py":         60,
    "check_runtime.py":     30,
    "default":              20,
}

PIPELINE = [
    ("primitive",   "check_primitives.py",  True),
    ("ui",          "check_ui.py",          True),
    ("boundary",    "check_boundary.py",    True),
    ("css",         "check_css.py",         True),
    ("preview",     "check_preview.py",     True),
    ("interaction", None,                   True),
    ("cap-chain",  "check_capability_chain.py",    True),
    ("gov",         "check_init_governance.py",    True),
    ("ssr-parity",  "check_ssr_runtime_parity.py", True),
    ("contracts",   "check_data_contracts.py",     True),
    ("mutation",    "check_mutation_governance.py",      False),
    ("runtime",     "check_runtime.py",                  True),
    ("replay",      "check_replay.py",                   False),
    ("alive",       "check_runtime_alive.py",            False),
    ("behavior",    "check_behavior.py",                 False),
    ("connectivity","check_interaction_connectivity.py", False),
    ("canvas",      "check_canvas.py",                   False),
    ("memory",      "check_memory_leak.py",              False),
    ("perf",        "check_performance.py",              False),
    ("contracts",   "check_contracts_runtime.py",        False),
    ("chaos",       "check_chaos.py",                    False),
    ("obs-graph",   "check_observer_graph.py",           False),
]
FULL_STEPS = [
    ("hydration",   "check_hydration.py",   True),
    ("playwright",  "check_playwright.py",  False),
]

def scan_primitive_interaction(component):
    """Scan primitive .rs file for data-rs-interaction value."""
    # try exact match first
    candidates = glob.glob(f"{PRIMITIVES_DIR}/{component}.rs")
    if not candidates:
        # fuzzy: component name anywhere in filename
        candidates = [p for p in glob.glob(f"{PRIMITIVES_DIR}/*.rs")
                      if component in os.path.basename(p)]
    if not candidates:
        return None
    try:
        src = open(candidates[0]).read()
        m = re.search(r'data-rs-interaction="([^"]+)"', src)
        return m.group(1) if m else None
    except Exception:
        return None

def resolve_interaction(component):
    """Infer interaction checker by scanning primitive — no hardcode."""
    group = scan_primitive_interaction(component)
    if group:
        return IX_GROUP_MAP.get(group, DEFAULT_INTERACTION), group
    return DEFAULT_INTERACTION, "unknown"

def get_timeout(checker):
    return CHECKER_TIMEOUTS.get(checker, CHECKER_TIMEOUTS["default"])

def run_checker(checker, target=None, label=None, verbose=False):
    """Run checker safely — never explode traceback."""
    path = os.path.join(_find_checker(checker), checker)
    if not os.path.exists(path):
        print(f"  [CR-CHK-001] {label or checker} — checker not found")
        return False
    cmd = [sys.executable, path]
    if target: cmd.append(target)
    timeout = get_timeout(checker)
    try:
        result = subprocess.run(cmd, timeout=timeout)
        return result.returncode == 0
    except subprocess.TimeoutExpired:
        print(f"  [CR-CHK-002] {label or checker} — timeout ({timeout}s)")
        return False
    except Exception as e:
        print(f"  [CR-CHK-003] {label or checker} — runner error: {type(e).__name__}: {e}")
        return False

def run_component(component, full=False, start_from=None, verbose=False):
    """Run pipeline for a single component."""
    interaction_checker, ix_group = resolve_interaction(component)
    print(f"\n{"="*54}")
    print(f"canon check: {component}  [ix:{ix_group}]")
    print(f"{"="*54}")
    steps = list(PIPELINE)
    if full: steps += FULL_STEPS
    started = start_from is None
    passed = failed = skipped = 0
    for label, checker, supports_target in steps:
        if not started:
            if label == start_from: started = True
            else: skipped += 1; continue
        if checker is None: checker = interaction_checker
        target = component if supports_target else None
        print(f"\n  [{label.upper()}] {checker}{f' {target}' if target else ''}")
        ok = run_checker(checker, target=target, label=label, verbose=verbose)
        print(f"  -> {'PASS' if ok else 'FAIL'}")
        if ok: passed += 1
        else:  failed += 1
    print(f"\n{"="*54}")
    print(f"Result: {passed} passed, {failed} failed, {skipped} skipped")
    if failed:
        print(f"[CR-CHK-010] {component} — pipeline failed at {failed} step(s)")
    return failed == 0

def run_ci(verbose=False):
    """Full CI — all checkers, no scope."""
    print("\n[CI] Full pipeline — all components")
    ci_checkers = [
        "check_primitives.py", "check_ui.py", "check_boundary.py",
        "check_css.py", "check_preview.py",
        "check_interactions_core.py", "check_interactions_data.py",
        "check_interactions_overlay.py",
        "check_runtime.py", "check_hydration.py", "check_playwright.py",
    ]
    passed = failed = 0
    for checker in ci_checkers:
        print(f"\n  [CI] {checker}")
        ok = run_checker(checker, verbose=verbose)
        print(f"  -> {'PASS' if ok else 'FAIL'}")
        if ok: passed += 1
        else:  failed += 1
    print(f"\n{"="*54}")
    print(f"CI: {passed}/{passed+failed} checkers passed")
    return failed == 0

def main():
    args = sys.argv[1:]
    verbose   = "--verbose" in args or "-v" in args
    full      = "--full" in args
    ci        = "--ci" in args or not args
    from_step = None
    if "--from" in args:
        i = args.index("--from")
        if i + 1 < len(args): from_step = args[i + 1]
    components = [a for a in args
                  if not a.startswith("--") and a != from_step]
    if ci and not components:
        return 0 if run_ci(verbose=verbose) else 1
    if not components:
        print("[CR-CHK-004] No component specified")
        print("Usage: python3 canon_check.py <component> [--full] [--verbose] [--from <step>]")
        return 1
    ok = True
    for component in components:
        ok = run_component(component, full=full, start_from=from_step, verbose=verbose) and ok
    return 0 if ok else 1

if __name__ == "__main__":
    sys.exit(main())