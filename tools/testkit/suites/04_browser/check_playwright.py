#!/usr/bin/env python3
"""
check_playwright.py — Browser runtime governance via Playwright
Gera playwright_tests.cjs a partir do codigo fonte e executa.
"""
import sys, subprocess, shutil, os

DIR    = os.path.dirname(os.path.abspath(__file__))
GEN    = os.path.join(DIR, "gen_playwright_tests.py")
SCRIPT = os.path.join(DIR, "playwright_tests.cjs")

def run():
    if not shutil.which("playwright"):
        print("[SKIP] playwright nao encontrado no PATH")
        return 0
    # gera o script de testes a partir do codigo fonte
    gen = subprocess.run(["python3", GEN], capture_output=True, text=True)
    if gen.returncode != 0:
        print("[FAIL] gen_playwright_tests.py falhou:\n" + gen.stderr)
        return 1
    env = dict(os.environ)
    env["NODE_PATH"] = "/usr/lib/node_modules"
    result = subprocess.run(["node", SCRIPT], capture_output=False, env=env)
    return result.returncode

if __name__ == "__main__":
    sys.exit(run())
