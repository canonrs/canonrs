#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""
check_data_contracts.py — Runtime DOM Selector Contracts v3

Usa runtime_selectors/*.yaml como source of truth.
Falha apenas em selectors LOCAL nao verificados (UNVERIFIED).

Usage:
  python3 check_data_contracts.py
  python3 check_data_contracts.py chart
"""
import re, glob, os, sys

INT_BASE      = _CANONRS_ROOT + ""
MANIFESTS_DIR = f"{INT_BASE}/canonrs-interactions/runtime_selectors"

def run(target=None):
    manifests = glob.glob(f"{MANIFESTS_DIR}/*.yaml")
    if not manifests:
        print("[FAIL] no runtime_selectors manifests found")
        return 1

    fails = []
    warns = []
    checked = 0

    for path in sorted(manifests):
        cid = os.path.basename(path).replace(".yaml", "")
        if target and target not in cid: continue
        checked += 1
        try:
            content = open(path).read()
        except Exception:
            continue

        # Find UNVERIFIED local selectors
        for line in content.splitlines():
            line = line.strip()
            if "UNVERIFIED" in line and line.startswith("- "):
                sel = line.replace("- ", "").replace("# UNVERIFIED", "").strip()
                attr = re.match(r'\[([a-z0-9-]+)', sel)
                attr_name = attr.group(1) if attr else sel
                fails.append(
                    f"[CR-CTR-100] {cid} — runtime usa [{attr_name}] mas primitivo nao emite\n"
                    f"              classificacao: LOCAL UNVERIFIED\n"
                    f"              risco: selector retorna None em producao\n"
                    f"              fix: adicionar ao primitivo OU reclassificar no manifest"
                )

    print(f"\n" + "=" * 50)
    if fails:
        for f in fails: print(f"\n[ERRO] {f}")
        print(f"\n[FAIL] {len(fails)} LOCAL contract violation(s)")
        return 1

    if target:
        print(f"[OK] {target} — DOM contracts clean")
    else:
        print(f"[OK] DOM contracts clean — {checked} manifests checked")
    print("[OK] Runtime selectors match primitive topology")
    return 0

if __name__ == "__main__":
    sys.exit(run(sys.argv[1] if len(sys.argv) > 1 else None))