#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""check_capability_groups.py — Capability group wasm artifact certification"""
import sys
from pathlib import Path

ROOT   = Path(_CANONRS_ROOT + "")
ASSETS = ROOT / "canonrs-client/assets/wasm"
GROUPS = ["init", "nav", "data", "gesture", "overlay", "selection", "content"]

def run():
    errors = []
    warnings = []

    for group in GROUPS:
        group_dir  = ASSETS / group
        wasm_file  = group_dir / f"canonrs_interactions_{group}_bg.wasm"
        js_file    = group_dir / f"canonrs_interactions_{group}.js"

        if not group_dir.exists():
            errors.append(f"[CR-CAP-001] {group}: directory missing — build_all_groups() not run")
            continue

        if not wasm_file.exists():
            errors.append(f"[CR-CAP-002] {group}: wasm missing — {wasm_file.name}")
            continue

        if not js_file.exists():
            errors.append(f"[CR-CAP-003] {group}: JS missing — {js_file.name}")
            continue

        # Validate wasm magic bytes
        magic = wasm_file.read_bytes()[:4]
        if magic != b"\x00asm":
            errors.append(f"[CR-CAP-004] {group}: invalid wasm magic: {magic.hex()}")
            continue

        size = wasm_file.stat().st_size
        if size < 10000:
            errors.append(f"[CR-CAP-005] {group}: wasm too small: {size} bytes")
            continue

        # Validate JS has init_<group>_all export
        js_src = js_file.read_text()
        if f"init_{group}_all" not in js_src:
            errors.append(f"[CR-CAP-006] {group}: missing export init_{group}_all")
            continue

        print(f"  [OK] {group}: wasm={size:,}b magic=valid export=init_{group}_all")

    print("\n" + "="*50)
    if errors:
        for e in errors: print(f"[ERRO] {e}")
        print(f"[FAIL] {len(errors)} capability group violations")
        return 1

    print(f"[OK] All {len(GROUPS)} capability groups certified")
    return 0

if __name__ == "__main__":
    sys.exit(run())