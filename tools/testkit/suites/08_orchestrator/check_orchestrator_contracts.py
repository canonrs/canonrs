#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""check_orchestrator_contracts.py — Orchestrator runtime contracts (static)"""
import re, sys
from pathlib import Path

SRC = Path(_CANONRS_ROOT + "/tools/canonrs-orchestrator/src")

def run():
    watchers = (SRC / "watchers.rs").read_text()
    wasm     = (SRC / "wasm.rs").read_text()
    ws       = (SRC / "ws.rs").read_text()
    config   = (SRC / "config.rs").read_text()
    errors = []

    # CONTRACT: watcher — single rebuild (debounce)
    if "WASM_DEBOUNCE_MS" not in watchers:
        errors.append("[CR-CONT-001] wasm_watcher: missing debounce — violates single_rebuild contract")
    if "CORE_DEBOUNCE_MS" not in watchers:
        errors.append("[CR-CONT-002] core_watcher: missing debounce — violates bounded_debounce contract")

    # CONTRACT: watcher — no parallel build (last_build timestamp guard)
    if "last_build.elapsed()" not in watchers:
        errors.append("[CR-CONT-003] watcher: missing elapsed guard — parallel builds possible")

    # CONTRACT: ws — broadcast all clients
    if "resubscribe" not in ws:
        errors.append("[CR-CONT-004] ws: missing resubscribe — not all clients receive reload")
    if "broadcast" not in ws:
        errors.append("[CR-CONT-005] ws: must use broadcast channel for multi-client support")

    # CONTRACT: wasm — hash changes after rebuild
    if "wasm_hash" not in wasm:
        errors.append("[CR-CONT-006] wasm: missing hash generation — cache busting broken")
    if "inject_hash_in_html" not in wasm:
        errors.append("[CR-CONT-007] wasm: missing hash injection — browser gets stale wasm")

    # CONTRACT: wasm — reload sent after successful build only
    if "reload_tx.send" not in wasm:
        errors.append("[CR-CONT-008] wasm: reload not sent after build")
    # reload must be inside success branch
    success_branch = re.search(r"s\.success\(\).*?reload_tx\.send", wasm, re.DOTALL)
    if not success_branch:
        errors.append("[CR-CONT-009] wasm: reload must only send on success — not on failure")

    # CONTRACT: loader watcher — no wasm rebuild trigger
    loader = re.search(r"spawn_loader_watcher.*?^}", watchers, re.DOTALL | re.MULTILINE)
    if loader and "build_wasm" in loader.group(0):
        errors.append("[CR-CONT-010] loader_watcher: must not trigger wasm rebuild")

    # CONTRACT: wasm-opt e gzip em release
    pipeline = (SRC / "pipeline.rs").read_text()
    if "CANON_RELEASE" not in pipeline:
        errors.append("[CR-CONT-011] pipeline: CANON_RELEASE nao detectado")
    if "wasm-opt" not in wasm:
        errors.append("[CR-CONT-012] wasm: wasm-opt ausente em release")
    if "gzip" not in wasm:
        errors.append("[CR-CONT-013] wasm: gzip ausente em release")
    if "LEPTOS_WASM_OPT_VERSION" not in pipeline:
        errors.append("[CR-CONT-014] pipeline: LEPTOS_WASM_OPT_VERSION ausente")
    if "gzip" not in pipeline:
        errors.append("[CR-CONT-015] pipeline: gzip ausente no leptos release")

        print("\n" + "="*50)
    if errors:
        for e in errors: print(f"[ERRO] {e}")
        print(f"[FAIL] {len(errors)} orchestrator contract violations")
        return 1

    print("[OK] watcher: single_rebuild (debounce)")
    print("[OK] watcher: bounded_debounce (elapsed guard)")
    print("[OK] ws: broadcast_all (resubscribe)")
    print("[OK] wasm: hash_changes_after_rebuild")
    print("[OK] wasm: reload_on_success_only")
    print("[OK] loader: no_wasm_trigger")
    print("[OK] wasm: wasm-opt em release")
    print("[OK] wasm: gzip em release")
    print("[OK] pipeline: LEPTOS_WASM_OPT_VERSION configurado")
    print("[OK] pipeline: gzip no leptos release")
    print("[OK] Orchestrator contracts compliant")
    return 0

if __name__ == "__main__":
    sys.exit(run())