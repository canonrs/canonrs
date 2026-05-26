#!/usr/bin/env python3
"""check_runtime_artifacts.py"""
import subprocess, sys, os
DIR = os.path.dirname(os.path.abspath(__file__))
def run():
    env = dict(os.environ)
    env["NODE_PATH"] = "/usr/lib/node_modules"
    try:
        result = subprocess.run(["node", os.path.join(DIR, "runtime_artifacts_tests.cjs")], env=env, timeout=180)
        return result.returncode
    except subprocess.TimeoutExpired:
        print("[FAIL] timeout"); return 1
    except Exception as e:
        print(f"[FAIL] {e}"); return 1
if __name__ == "__main__": sys.exit(run())
