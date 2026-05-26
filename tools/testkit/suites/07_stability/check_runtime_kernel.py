#!/usr/bin/env python3
"""check_runtime_kernel.py — Runtime Kernel Certification
Validates: ownership graph, lifecycle states, cleanup cascade, dispatch determinism
"""
import subprocess, sys, os

TESTS_DIR = os.path.dirname(os.path.abspath(__file__))

def run():
    cjs = os.path.join(TESTS_DIR, "runtime_kernel_tests.cjs")
    env = dict(os.environ)
    env["NODE_PATH"] = "/usr/lib/node_modules"
    try:
        result = subprocess.run(["node", cjs], env=env, timeout=180)
        return result.returncode
    except subprocess.TimeoutExpired:
        print("[FAIL] check_runtime_kernel.py — timeout"); return 1
    except Exception as e:
        print(f"[FAIL] check_runtime_kernel.py — {e}"); return 1

if __name__ == "__main__":
    sys.exit(run())
