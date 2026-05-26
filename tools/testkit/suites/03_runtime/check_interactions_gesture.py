#!/usr/bin/env python3
"""check_interactions_gesture.py — canonrs-interactions-gesture governance only"""
import subprocess, sys, os

TESTS_DIR = os.path.dirname(os.path.abspath(__file__))
CRATE = "canonrs-interactions-gesture"

def run():
    checker = os.path.join(TESTS_DIR, "check_interactions.py")
    try:
        result = subprocess.run(
            [sys.executable, checker],
            capture_output=True, text=True, timeout=120
        )
        output = result.stdout + result.stderr
        lines = output.splitlines()
        errors = 0
        in_crate = False
        for line in lines:
            if "(" + CRATE + ")" in line:
                in_crate = True
                print(line)
                if "[ERRO]" in line:
                    errors += 1
            elif in_crate and line.startswith("   "):
                print(line)
            else:
                in_crate = False

        print("\n" + "="*50)
        if errors > 0:
            print(f"[FAIL] {errors} violations in canonrs-interactions-gesture")
            return 1
        print(f"[OK] canonrs-interactions-gesture — clean")
        return 0
    except subprocess.TimeoutExpired:
        print("[FAIL] timeout"); return 1
    except Exception as e:
        print(f"[FAIL] {e}"); return 1

if __name__ == "__main__":
    sys.exit(run())
