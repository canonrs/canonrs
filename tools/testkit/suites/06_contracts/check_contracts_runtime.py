#!/usr/bin/env python3
"""check_contracts_runtime.py — Declarative Runtime Contract Validation"""
import subprocess, sys, os

TESTS_DIR = os.path.dirname(os.path.abspath(__file__))

def run():
    cjs = os.path.join(TESTS_DIR, "run_contracts.cjs")
    env = dict(os.environ)
    env["NODE_PATH"] = "/usr/lib/node_modules:/home/cristiano/.npm-global/lib/node_modules"
    try:
        result = subprocess.run(["node", cjs], env=env, capture_output=False, timeout=300)
        return result.returncode
    except subprocess.TimeoutExpired:
        print("[FAIL] check_contracts_runtime.py — timeout (300s)"); return 1
    except Exception as e:
        print(f"[FAIL] check_contracts_runtime.py — {e}"); return 1

if __name__ == "__main__":
    sys.exit(run())
