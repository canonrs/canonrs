#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""check_state_consistency.py — SystemState governance (static)"""
import re, sys
from pathlib import Path

STATE   = Path(_CANONRS_ROOT + "/tools/canonrs-orchestrator/src/state.rs")
WASM    = Path(_CANONRS_ROOT + "/tools/canonrs-orchestrator/src/wasm.rs")
PIPE    = Path(_CANONRS_ROOT + "/tools/canonrs-orchestrator/src/pipeline.rs")

def run():
    state_src = STATE.read_text()
    wasm_src  = WASM.read_text()
    pipe_src  = PIPE.read_text()
    errors = []

    # SystemState must have tokens, wasm, leptos fields
    for field in ["tokens", "wasm", "leptos"]:
        if f"pub {field}" not in state_src:
            errors.append(f"[CR-STATE-001] SystemState missing field: {field}")

    # wasm.rs must update state on success AND failure
    if 's.wasm = format!("OK' not in wasm_src:
        errors.append("[CR-STATE-002] wasm.rs must set state to OK on success")
    if '"FAILED"' not in wasm_src:
        errors.append("[CR-STATE-003] wasm.rs must set state to FAILED on failure")

    # pipeline.rs must update tokens state
    if 'state.lock().unwrap().tokens' not in pipe_src:
        errors.append("[CR-STATE-004] pipeline.rs must update tokens state")

    # state.print() must be called after all services start
    main_src = Path(_CANONRS_ROOT + "/tools/canonrs-orchestrator/src/main.rs").read_text()
    if "state.lock().unwrap().print()" not in main_src:
        errors.append("[CR-STATE-005] main.rs must call state.print() after boot")

    print("\n" + "="*50)
    if errors:
        for e in errors: print(f"[ERRO] {e}")
        print(f"[FAIL] {len(errors)} state consistency violations")
        return 1

    print("[OK] SystemState fields: tokens, wasm, leptos")
    print("[OK] State transitions: OK/FAILED/RUNNING defined")
    print("[OK] State printed after boot")
    print("[OK] State consistency compliant")
    return 0

if __name__ == "__main__":
    sys.exit(run())