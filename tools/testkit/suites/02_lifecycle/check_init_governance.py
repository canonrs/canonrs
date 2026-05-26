#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""
check_init_governance.py — Lifecycle Ownership Governance

Regra: bootstrap e o unico owner do lifecycle.
init_guard() e PROIBIDO em modulos dispatchaveis.
Permitido APENAS em: runtime/bootstrap.rs e dom/lifecycle.rs

Usage:
  python3 check_init_governance.py
  python3 check_init_governance.py data_table
"""
import re, glob, os, sys

INT_BASE = _CANONRS_ROOT + ""

# Arquivos onde init_guard E permitido
ALLOWED = ["bootstrap.rs", "lifecycle.rs", "mod.rs"]

def is_allowed(path):
    basename = os.path.basename(path)
    # bootstrap.rs e lifecycle.rs sempre permitidos
    if basename in ("bootstrap.rs", "lifecycle.rs"): return True
    # mod.rs de capability registry (tem CAPS array) — owner do lifecycle do grupo
    if basename == "mod.rs":
        try:
            src = open(path).read()
            return "CAPS" in src and "for cap in" in src
        except Exception:
            return False
    return False

def run(target=None):
    files = [
        f for f in glob.glob(f"{INT_BASE}/canonrs-interactions-*/src/**/*.rs", recursive=True)
        if ".bak" not in f and "target/" not in f
    ]
    violations = []
    checked = 0
    for path in sorted(files):
        cid = os.path.basename(path).replace(".rs", "")
        if target and target not in path: continue
        if is_allowed(path): continue
        checked += 1
        try:
            src = open(path).read()
            nc  = re.sub(r"//[^\n]*", "", src)
            if "init_guard" in nc:
                rel = os.path.relpath(path, INT_BASE)
                violations.append(rel)
        except Exception:
            pass
    print(f"\n" + "=" * 50)
    if violations:
        for v in violations:
            print(f"\n[ERRO] {v}")
            print(f"   [CR-GOV-100] init_guard em modulo dispatchavel")
            print(f"              garantia: bootstrap e o unico owner do lifecycle")
            print(f"              remover init_guard — bootstrap ja protege contra double-init")
        print(f"\n[FAIL] {len(violations)} lifecycle governance violation(s)")
        return 1
    if target:
        print(f"[OK] {target} — lifecycle governance clean")
    else:
        print(f"[OK] lifecycle governance clean — {checked} files checked")
    print("[OK] Bootstrap is sole lifecycle owner")
    return 0

if __name__ == "__main__":
    sys.exit(run(sys.argv[1] if len(sys.argv) > 1 else None))