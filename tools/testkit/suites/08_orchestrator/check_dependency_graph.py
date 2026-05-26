#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""check_dependency_graph.py — Watcher dependency graph governance (static)"""
import re, sys
from pathlib import Path

WATCHERS = Path(_CANONRS_ROOT + "/tools/canonrs-orchestrator/src/watchers.rs")
CONFIG   = Path(_CANONRS_ROOT + "/tools/canonrs-orchestrator/src/config.rs")

def run():
    watchers_src = WATCHERS.read_text()
    config_src   = CONFIG.read_text()
    errors = []

    # wasm watcher must only trigger on .rs files
    wasm_watcher = re.search(r'spawn_wasm_watcher.*?^}', watchers_src, re.DOTALL | re.MULTILINE)
    if wasm_watcher:
        body = wasm_watcher.group(0)
        if '"rs"' not in body:
            errors.append("[CR-DEP-001] wasm_watcher must filter .rs files only")
        if "build_wasm" not in body:
            errors.append("[CR-DEP-002] wasm_watcher must call build_wasm")
        if "reload_tx" not in body:
            errors.append("[CR-DEP-003] wasm_watcher must send reload after build")

    # core watcher must trigger on .yaml or .rs but NOT call build_wasm
    core_watcher = re.search(r'spawn_core_watcher.*?^}', watchers_src, re.DOTALL | re.MULTILINE)
    if core_watcher:
        body = core_watcher.group(0)
        if "build_wasm" in body:
            errors.append("[CR-DEP-004] core_watcher must NOT call build_wasm (touches build.rs instead)")
        if "build.rs" not in body:
            errors.append("[CR-DEP-005] core_watcher must touch build.rs to trigger leptos recompile")

    # loader watcher must be NonRecursive
    loader_watcher = re.search(r'spawn_loader_watcher.*?^}', watchers_src, re.DOTALL | re.MULTILINE)
    if loader_watcher:
        body = loader_watcher.group(0)
        if "NonRecursive" not in body:
            errors.append("[CR-DEP-006] loader_watcher must use NonRecursive mode")
        if "build_wasm" in body:
            errors.append("[CR-DEP-007] loader_watcher must NOT trigger wasm build")

    # WASM_CRATES must include canonrs-interactions
    crates = re.findall(r'"(canonrs-interactions[^"]*)"', config_src)
    if "canonrs-interactions" not in crates:
        errors.append("[CR-DEP-008] WASM_CRATES must include canonrs-interactions entry point")

    # debounce must be defined
    if "WASM_DEBOUNCE_MS" not in config_src:
        errors.append("[CR-DEP-009] WASM_DEBOUNCE_MS must be defined")
    if "CORE_DEBOUNCE_MS" not in config_src:
        errors.append("[CR-DEP-010] CORE_DEBOUNCE_MS must be defined")

    print("\n" + "="*50)
    if errors:
        for e in errors: print(f"[ERRO] {e}")
        print(f"[FAIL] {len(errors)} dependency graph violations")
        return 1

    print(f"[OK] {len(crates)} wasm crates registered")
    print(f"[OK] wasm_watcher: .rs → build_wasm → reload")
    print(f"[OK] core_watcher: .yaml/.rs → build.rs touch → leptos recompile")
    print(f"[OK] loader_watcher: NonRecursive, no wasm trigger")
    print("[OK] Dependency graph governance compliant")
    return 0

if __name__ == "__main__":
    sys.exit(run())