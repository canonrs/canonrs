#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""check_wasm_artifacts.py — WASM artifact integrity certification"""
import sys, re
from pathlib import Path

ROOT      = Path(_CANONRS_ROOT + "")
ASSETS_JS = ROOT / "canonrs-client/assets/js"
ASSETS_WM = ROOT / "canonrs-client/assets/wasm"
SERVER    = ROOT / "canonrs-server/src"

REQUIRED_BUNDLE = ["__canonRuntime", "__canonGroups", "init_all", "canon-reload", "__CANON_WASM_HASH__"]

def run():
    errors = []

    # 1. wasm bundle exists and valid
    wasm_file = ASSETS_WM / "canonrs_interactions_bg.wasm"
    if not wasm_file.exists():
        errors.append("[CR-ART-001] wasm bundle missing")
    else:
        size  = wasm_file.stat().st_size
        magic = wasm_file.read_bytes()[:4]
        if magic != b"\x00asm":
            errors.append(f"[CR-ART-002] invalid wasm magic: {magic.hex()}")
        elif size < 10000:
            errors.append(f"[CR-ART-003] wasm too small: {size} bytes")
        else:
            print(f"  [OK] wasm: {size:,} bytes, magic valid")

    # 2. wasm JS exists with required exports
    js_file = ASSETS_WM / "canonrs_interactions.js"
    if not js_file.exists():
        errors.append("[CR-ART-004] canonrs_interactions.js missing")
    else:
        js_src = js_file.read_text()
        for export in ["init_all", "init_subtree", "gc"]:
            if "function " + export not in js_src:
                errors.append(f"[CR-ART-005] missing wasm export: {export}")
        if "__wbindgen" not in js_src:
            errors.append("[CR-ART-006] wasm-bindgen markers missing")
        else:
            print("  [OK] wasm JS: exports valid")

    # 3. wasm_hash.js exists and valid
    hash_file = ASSETS_JS / "wasm_hash.js"
    if not hash_file.exists():
        errors.append("[CR-ART-007] wasm_hash.js missing")
    else:
        hash_src = hash_file.read_text()
        m = re.search(r"__CANON_WASM_HASH__", hash_src)
        if not m:
            errors.append("[CR-ART-008] wasm_hash.js invalid format")
        else:
            print(f"  [OK] wasm hash: present")

    # 4. bundle integrity
    bundle = ASSETS_JS / "canonrs.bundle.js"
    if not bundle.exists():
        errors.append("[CR-ART-009] canonrs.bundle.js missing")
    else:
        src = bundle.read_text()
        missing = [r for r in REQUIRED_BUNDLE if r not in src]
        if missing:
            errors.append(f"[CR-ART-010] bundle missing: {missing}")
        else:
            print("  [OK] bundle: all runtime components present")

    # 5. src/loader is source of truth
    loader_src = ROOT / "canonrs-client/src/loader/canonrs.bundle.js"
    if loader_src.exists():
        src2 = loader_src.read_text()
        missing2 = [r for r in REQUIRED_BUNDLE if r not in src2]
        if missing2:
            errors.append(f"[CR-ART-011] src/loader/canonrs.bundle.js missing: {missing2}")
        else:
            print("  [OK] src/loader: source of truth valid")

    # 6. canon-init-loader.js not served
    init_loader = ROOT / "canonrs-client/src/loader/canon-init-loader.js"
    if init_loader.exists():
        served = any("canon-init-loader" in f.read_text() for f in SERVER.rglob("*.rs"))
        if served:
            errors.append("[CR-ART-012] canon-init-loader.js served but per-group wasm not generated")
        else:
            print("  [OK] canon-init-loader.js: not served (dead code)")

    print("\n" + "="*50)
    if errors:
        for e in errors: print(f"[ERRO] {e}")
        print(f"[FAIL] {len(errors)} artifact violations")
        return 1

    print("[OK] WASM artifact integrity certified")
    return 0

if __name__ == "__main__":
    sys.exit(run())