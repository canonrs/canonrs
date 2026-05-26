#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""
check_capability_chain.py — Capability Chain Deadlock Governance

Detecta init_guard duplicado na chain pai->filho.
Um pai que chama init_guard E delega para capabilities
que tambem chamam init_guard no mesmo elemento = subtree deadlock.

Usage:
  python3 check_capability_chain.py
  python3 check_capability_chain.py data_table
"""
import re, glob, os, sys

INT_BASE = _CANONRS_ROOT + ""

def strip_comments(src):
    return re.sub(r"//[^\n]*", "", src)

def find_caps_in_mod(nc):
    """Extract capability function names from CAPS array."""
    m = re.search(r"const CAPS[^=]*=\s*&\[([^\]]+)\]", nc, re.DOTALL)
    if not m: return []
    block = m.group(1)
    return re.findall(r"(\w+)::init", block)

def has_init_guard(nc):
    return "init_guard" in nc

def has_caps_delegation(nc):
    """Detects pattern: iterates CAPS and calls each cap."""
    return "CAPS" in nc and ("cap(&el)" in nc or "cap(el" in nc or "for cap in" in nc)

def check_group(group_path, target=None):
    errors = []
    src_dir = os.path.join(group_path, "src")
    if not os.path.exists(src_dir): return errors

    # Find all mod.rs files that are capability registries
    mod_files = glob.glob(f"{src_dir}/**/mod.rs", recursive=True)

    for mod_path in sorted(mod_files):
        mod_src = open(mod_path).read()
        mod_nc  = strip_comments(mod_src)

        # Skip if not a capability registry
        if not has_caps_delegation(mod_nc): continue
        if not has_init_guard(mod_nc): continue

        mod_dir = os.path.dirname(mod_path)
        mod_id  = os.path.relpath(mod_path, src_dir)

        if target and target not in mod_id and target not in mod_path: continue

        caps = find_caps_in_mod(mod_nc)

        # Check each capability for duplicate init_guard
        for cap in caps:
            cap_path = os.path.join(mod_dir, f"{cap}.rs")
            if not os.path.exists(cap_path): continue
            cap_src = open(cap_path).read()
            cap_nc  = strip_comments(cap_src)
            if has_init_guard(cap_nc):
                rel_mod = os.path.relpath(mod_path, INT_BASE)
                rel_cap = os.path.relpath(cap_path, INT_BASE)
                errors.append({
                    "mod": rel_mod,
                    "cap": rel_cap,
                    "cap_name": cap,
                })

    return errors

def run(target=None):
    groups = sorted(glob.glob(f"{INT_BASE}/canonrs-interactions-*/"))
    if not groups:
        print("[FAIL] no interaction groups found")
        return 1

    all_errors = []
    checked = 0

    for group in groups:
        errs = check_group(group, target=target)
        all_errors.extend(errs)
        checked += 1

    print(f"\n" + "=" * 50)

    if all_errors:
        for e in all_errors:
            print(f"\n[ERRO] {e['mod']}")
            print(f"   [CR-CAP-100] init_guard duplicado: {e['cap_name']}")
            print(f"              pai:  {e['mod']}")
            print(f"              filho: {e['cap']}")
            print(f"              garantia: init_guard DEVE existir apenas no pai da capability chain")
            print(f"              efeito: capabilities nunca executam (subtree deadlock)")
        print(f"\n[FAIL] {len(all_errors)} init_guard topology violation(s)")
        return 1

    if target:
        print(f"[OK] {target} — init topology clean")
    else:
        print(f"[OK] init topology clean — {checked} groups checked")
    print("[OK] No init_guard topology violations")
    return 0

if __name__ == "__main__":
    sys.exit(run(sys.argv[1] if len(sys.argv) > 1 else None))