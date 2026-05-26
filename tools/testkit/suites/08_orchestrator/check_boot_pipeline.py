#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""check_boot_pipeline.py — Boot pipeline order governance (static)"""
import re, sys
from pathlib import Path

MAIN = Path(_CANONRS_ROOT + "/tools/canonrs-orchestrator/src/main.rs")

def run():
    src = MAIN.read_text()
    errors = []

    # Extract step order from main()
    steps = re.findall(r'// (\d+)\. (\w+)', src)
    order = [s[1].lower() for s in steps]

    EXPECTED_ORDER = ["tokens", "loaders", "wasm", "watchers", "ws", "css", "leptos"]
    for i, step in enumerate(EXPECTED_ORDER):
        if i >= len(order):
            errors.append(f"[CR-BOOT-{i+1:03d}] missing step: {step}")
        elif order[i] != step:
            errors.append(f"[CR-BOOT-{i+1:03d}] wrong order: expected {step} at pos {i+1}, got {order[i]}")

    # tokens before wasm
    if "tokens" in order and "wasm" in order:
        if order.index("tokens") > order.index("wasm"):
            errors.append("[CR-BOOT-010] tokens must start before wasm")

    # wasm before watchers
    if "wasm" in order and "watchers" in order:
        if order.index("wasm") > order.index("watchers"):
            errors.append("[CR-BOOT-011] wasm must build before watchers start")

    # ws before leptos
    if "ws" in order and "leptos" in order:
        if order.index("ws") > order.index("leptos"):
            errors.append("[CR-BOOT-012] ws server must start before leptos")

    print("\n" + "="*50)
    if errors:
        for e in errors: print(f"[ERRO] {e}")
        print(f"[FAIL] {len(errors)} boot pipeline violations")
        return 1

    print(f"[OK] boot order: {' → '.join(order)}")
    print("[OK] Boot pipeline governance compliant")
    return 0

if __name__ == "__main__":
    sys.exit(run())