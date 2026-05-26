#!/usr/bin/env python3
"""check_interaction_connectivity.py — Dispatch chain validation"""
import subprocess, sys, os

TESTS_DIR = os.path.dirname(os.path.abspath(__file__))

def run():
    cjs = os.path.join(TESTS_DIR, "connectivity_tests.cjs")
    env = dict(os.environ)
    env["NODE_PATH"] = "/usr/lib/node_modules"
    try:
        result = subprocess.run(["node", cjs], env=env, capture_output=False, timeout=120)
        return result.returncode
    except subprocess.TimeoutExpired:
        print("[FAIL] check_interaction_connectivity.py — timeout"); return 1
    except Exception as e:
        print(f"[FAIL] check_interaction_connectivity.py — {e}"); return 1

if __name__ == "__main__":
    sys.exit(run())
