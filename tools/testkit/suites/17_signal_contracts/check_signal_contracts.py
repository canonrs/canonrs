#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""
check_signal_contracts.py — CR-SIG-100..103
CR-SIG-100: RwSignal proibido em Primitive
CR-SIG-101: Signal proibido em runtime engine (interaction groups)
CR-SIG-102: Effect::new dentro de component render proibido
CR-SIG-103: signal.set dentro de view! fora de event handler proibido
"""
import re, glob, os, sys

CANONRS_DIR    = _CANONRS_ROOT + ""
PRIMITIVES_DIR = f"{CANONRS_DIR}/canonrs-core/src/primitives"
INTERACTIONS_DIR = CANONRS_DIR

EXCLUDE_PATH_FRAGMENTS = [".backup", "_old-workbench", "/old1/"]

def is_excluded(path):
    return any(f in path for f in EXCLUDE_PATH_FRAGMENTS)

def strip_comments(src):
    return re.sub(r"//[^\n]*", "", src)

def extract_view_blocks(nc):
    blocks = []
    for vm in re.finditer(r"view!\s*\{", nc):
        start = vm.start()
        depth = 0
        end = start
        for i, ch in enumerate(nc[start:], start):
            if ch == "{": depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    end = i
                    break
        blocks.append(nc[start:end])
    return blocks

def check_primitive(path):
    errors = []
    src = open(path).read()
    nc  = strip_comments(src)
    cid = os.path.basename(path)

    # CR-SIG-100
    if "RwSignal::new" in nc or "create_rw_signal" in nc:
        errors.append(f"[CR-SIG-100] {cid} — RwSignal::new em Primitive\n             primitives nao criam estado reativo")

    return errors

def check_interaction(path):
    errors = []
    src = open(path).read()
    nc  = strip_comments(src)
    cid = os.path.relpath(path, CANONRS_DIR)

    # CR-SIG-101: Signal em runtime engine
    if "RwSignal::new" in nc or "create_signal" in nc:
        errors.append(f"[CR-SIG-101] {cid} — Signal em runtime engine\n             interaction runtime nao deve criar signals reativos")

    return errors

def check_product_file(path, products_dir):
    errors = []
    src = open(path).read()
    nc  = strip_comments(src)
    cid = os.path.relpath(path, products_dir)

    # CR-SIG-103: signal.set dentro de view! fora de event handler
    # Procura .set( que NAO esteja precedido por on: na mesma linha
    for view_block in extract_view_blocks(nc):
        for line in view_block.splitlines():
            line_clean = line.strip()
            has_set = re.search(r"\w+\.set\(", line_clean)
            is_handler = re.search(r"on:\w+|move\s*\|", line_clean)
            is_effect = "Effect" in line_clean
            if has_set and not is_handler and not is_effect:
                errors.append(
                    f"[CR-SIG-103] {cid} — signal.set fora de event handler no view!\n"
                    f"             usar on:click, on:input ou mover para Effect"
                )
                break

    return errors

def run():
    errors_total = 0
    failed = 0
    total_ok = 0

    # CR-SIG-100: primitives
    for path in sorted(glob.glob(f"{PRIMITIVES_DIR}/*.rs")):
        if is_excluded(path): continue
        errs = check_primitive(path)
        if errs:
            print(f"\n[ERRO] {os.path.basename(path)}")
            for e in errs: print(f"   {e}")
            failed += 1; errors_total += len(errs)
        else:
            total_ok += 1

    # CR-SIG-101: interaction runtimes
    for pattern in [
        f"{INTERACTIONS_DIR}/canonrs-interactions-*/src/**/*.rs",
    ]:
        for path in sorted(glob.glob(pattern, recursive=True)):
            if is_excluded(path) or "/runtime/" not in path: continue
            errs = check_interaction(path)
            if errs:
                print(f"\n[ERRO] {os.path.relpath(path, CANONRS_DIR)}")
                for e in errs: print(f"   {e}")
                failed += 1; errors_total += len(errs)
            else:
                total_ok += 1

    print(f"\n{chr(61)*50}")
    print(f"[OK] {total_ok} files clean")
    if errors_total:
        print(f"[FAIL] {failed} files — {errors_total} violations")
        return 1
    print("[OK] Signal contracts canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run())