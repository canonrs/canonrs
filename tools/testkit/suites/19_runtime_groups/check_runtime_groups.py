#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""
check_runtime_groups.py — CR-RTG-100..103
CR-RTG-100: interaction group deve ser isolado
CR-RTG-101: overlay runtime nao pode importar nav runtime
CR-RTG-102: selection runtime nao pode importar overlay runtime
CR-RTG-103: cross-group coupling proibido
"""
import re, glob, os, sys

CANONRS_DIR = _CANONRS_ROOT + ""

FORBIDDEN_IMPORTS = [
    ("canonrs-interactions-overlay", "canonrs-interactions-nav",       "CR-RTG-101"),
    ("canonrs-interactions-overlay", "canonrs-interactions-selection",  "CR-RTG-102"),
    ("canonrs-interactions-selection","canonrs-interactions-overlay",   "CR-RTG-102"),
    ("canonrs-interactions-nav",     "canonrs-interactions-overlay",   "CR-RTG-103"),
    ("canonrs-interactions-gesture", "canonrs-interactions-overlay",   "CR-RTG-103"),
    ("canonrs-interactions-content", "canonrs-interactions-overlay",   "CR-RTG-103"),
]

def check_cargo(crate, forbidden_dep, rule):
    cargo = os.path.join(CANONRS_DIR, crate, "Cargo.toml")
    if not os.path.exists(cargo):
        return []
    src = open(cargo).read()
    if forbidden_dep in src:
        return [f"[{rule}] {crate} — importa {forbidden_dep}\n             cross-group coupling proibido"]
    return []

def run():
    errors_total = 0
    failed = 0
    total_ok = 0

    for crate, forbidden, rule in FORBIDDEN_IMPORTS:
        errs = check_cargo(crate, forbidden, rule)
        if errs:
            print(f"\n[ERRO] {crate}")
            for e in errs: print(f"   {e}")
            failed += 1; errors_total += len(errs)
        else:
            total_ok += 1

    print(f"\n{chr(61)*50}")
    print(f"[OK] {total_ok} checks clean")
    if errors_total:
        print(f"[FAIL] {failed} checks — {errors_total} violations")
        return 1
    print("[OK] Runtime groups canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run())