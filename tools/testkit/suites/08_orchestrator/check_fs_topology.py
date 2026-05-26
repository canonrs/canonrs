#!/usr/bin/env python3
"""check_fs_topology.py — Filesystem topology governance (static)"""
import sys, os
from pathlib import Path

ROOT     = Path("/opt/docker/monorepo")
ORCH_SRC = ROOT / "packages-rust/rs-canonrs/tools/canonrs-orchestrator/src"
CONFIG   = ORCH_SRC / "config.rs"

def run():
    config_src = CONFIG.read_text()
    errors = []

    import re

    # Validate WASM_CRATES paths exist
    crates = re.findall(r'"(canonrs-interactions[^"]*)"', config_src)
    for crate in crates:
        src_dir = ROOT / "packages-rust/rs-canonrs" / crate / "src"
        if not src_dir.exists():
            errors.append(f"[CR-FS-001] WASM_CRATE path missing: {src_dir}")

    # Validate CORE_WATCH_DIRS exist
    dirs = re.findall(r'"(packages-rust/rs-canonrs/[^"]+)"', config_src)
    for d in dirs:
        full = ROOT / d
        if not full.exists():
            errors.append(f"[CR-FS-002] CORE_WATCH_DIR missing: {full}")

    # Validate loader src/dest dirs exist
    loader_src  = ROOT / "packages-rust/rs-canonrs/canonrs-client/src/loader"
    loader_dest = ROOT / "packages-rust/rs-canonrs/canonrs-client/assets/js"
    if not loader_src.exists():
        errors.append(f"[CR-FS-003] loader src missing: {loader_src}")
    if not loader_dest.exists():
        errors.append(f"[CR-FS-004] loader dest missing: {loader_dest}")

    # Validate required loader files exist
    for f in ["canon-loader.js", "canonrs.bundle.js"]:
        if not (loader_src / f).exists():
            errors.append(f"[CR-FS-005] loader file missing: {f}")

    # Validate wasm output dir
    wasm_dest = ROOT / "packages-rust/rs-canonrs/canonrs-client/assets/wasm"
    if not wasm_dest.exists():
        errors.append(f"[CR-FS-006] wasm dest dir missing: {wasm_dest}")

    print("\n" + "="*50)
    if errors:
        for e in errors: print(f"[ERRO] {e}")
        print(f"[FAIL] {len(errors)} filesystem topology violations")
        return 1

    print(f"[OK] {len(crates)} wasm crate paths valid")
    print(f"[OK] loader src/dest dirs valid")
    print(f"[OK] wasm output dir valid")
    print("[OK] Filesystem topology compliant")
    return 0

if __name__ == "__main__":
    sys.exit(run())
