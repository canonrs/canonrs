#!/usr/bin/env python3
"""
check_replay.py — Runtime Replay Stability

Valida que init_count e replay_count sao estaveis apos bootstrap.
Detecta replay loops via window.__canonRuntime.snapshot().

Requer: servidor rodando em localhost:3000
Requer: playwright instalado

Usage:
  python3 check_replay.py
"""
import subprocess, sys, os, shutil

TESTS_DIR = os.path.dirname(os.path.abspath(__file__))

def run():
    cjs = os.path.join(TESTS_DIR, "replay_tests.cjs")
    if not os.path.exists(cjs):
        print(f"[FAIL] replay_tests.cjs not found: {cjs}")
        return 1
    try:
        env = dict(os.environ)
        env["NODE_PATH"] = "/usr/lib/node_modules"
        result = subprocess.run(
            ["node", cjs],
            env=env, capture_output=False, timeout=120
        )
        return result.returncode
    except subprocess.TimeoutExpired:
        print("[FAIL] check_replay.py — timeout (120s)")
        return 1
    except FileNotFoundError:
        print("[FAIL] node not found — install Node.js")
        return 1
    except Exception as e:
        print(f"[FAIL] check_replay.py — {type(e).__name__}: {e}")
        return 1

if __name__ == "__main__":
    sys.exit(run())
