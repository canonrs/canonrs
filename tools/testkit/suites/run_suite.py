#!/usr/bin/env python3
"""
run_suite.py — Run testkit suites

Usage:
  python3 run_suite.py                 # all suites
  python3 run_suite.py 01_architecture # single suite
  python3 run_suite.py 01 02 03        # multiple suites
  python3 run_suite.py --no-browser    # suites 01-03 only
"""
import sys, os, subprocess, glob

SUITES_DIR = os.path.dirname(os.path.abspath(__file__))
BROWSER_SUITES = {"04_browser", "05_behavior", "06_contracts", "07_stability"}

def get_suites(args):
    all_suites = sorted([
        os.path.basename(d.rstrip("/"))
        for d in glob.glob(f"{SUITES_DIR}/*/")
        if os.path.isdir(d)
    ])
    if "--no-browser" in args:
        return [s for s in all_suites if s not in BROWSER_SUITES]
    components = [a for a in args if not a.startswith("--")]
    if not components:
        return all_suites
    result = []
    for a in components:
        result.extend([s for s in all_suites if s.startswith(a)])
    return result

def run():
    args = sys.argv[1:]
    suites = get_suites(args)
    if not suites:
        print("[FAIL] no suites found"); return 1
    passed = failed = 0
    for suite in suites:
        run_all = os.path.join(SUITES_DIR, suite, "run_all.sh")
        if not os.path.exists(run_all):
            print(f"[SKIP] {suite}"); continue
        print(f"\n{'='*50}\n[SUITE] {suite}\n{'='*50}")
        result = subprocess.run(["bash", run_all], capture_output=False)
        if result.returncode == 0: passed += 1
        else: failed += 1
    print(f"\n{'='*50}")
    print(f"Suites: {passed} passed, {failed} failed")
    return failed

if __name__ == "__main__":
    sys.exit(run())
