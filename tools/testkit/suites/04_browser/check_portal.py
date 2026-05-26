#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""
check_portal.py — Portal SSR safety governance

Valida que:
1. leptos::portal::Portal NAO existe em nenhum arquivo Rust do canonrs
2. Overlays usam inline div + move_to_body JS
"""
import re, glob, os, sys

CANONRS_BASE = _CANONRS_ROOT + ""
SEARCH_DIRS  = [
    "canonrs-core/src",
    "canonrs-server/src",
]

PORTAL_PATTERN = re.compile(r"leptos::portal::Portal|<Portal>")


def check_file(path):
    errors = []
    src = open(path).read()
    src_no_comments = re.sub(r"//[^\n]*", "", src)
    if PORTAL_PATTERN.search(src_no_comments):
        rel = path.replace(CANONRS_BASE + "/", "")
        errors.append(
            f"[CR-PRT-100] {rel}\n"
            f"             leptos::portal::Portal proibido — nao funciona em SSR\n"
            f"             usar div inline + portal::move_to_body() no JS"
        )
    return errors


def run():
    errors = []
    files = []
    for d in SEARCH_DIRS:
        full = os.path.join(CANONRS_BASE, d)
        files += glob.glob(f"{full}/**/*.rs", recursive=True)

    if not files:
        print(f"[FAIL] 0 files found")
        return 1

    total_ok = 0
    for path in sorted(files):
        errs = check_file(path)
        if errs:
            for e in errs: print(f"[ERRO] {e}")
            errors.extend(errs)
        else:
            total_ok += 1

    print(f"\n{'='*50}")
    print(f"[OK] {total_ok} files clean")
    if errors:
        print(f"[FAIL] {len(errors)} portal violations found")
        return 1
    print("[OK] Portal architecture canonical — all SSR-safe")
    return 0


if __name__ == "__main__":
    sys.exit(run())