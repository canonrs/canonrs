#!/usr/bin/env python3
"""
check_ssr_contracts.py — CR-SSR-101..103
CR-SSR-101: LocalResource em pagina com componente interativo sem Suspense
CR-SSR-102: Select com valor inicial dinamico sem SelectionState::Selected
CR-SSR-103: fetch dentro de Effect/on_mount sem Suspense
"""
import re, glob, os, sys

PRODUCTS_DIR = "/opt/docker/monorepo/products"

def strip_comments(src):
    return re.sub(r"//[^\n]*", "", src)

def check_file(path):
    errors = []
    src = open(path).read()
    nc  = strip_comments(src)
    cid = os.path.relpath(path, PRODUCTS_DIR)

    # CR-SSR-101: LocalResource sem Suspense com componente interativo
    if "LocalResource::new" in nc and "Suspense" not in nc:
        if "ConfirmDialog" in nc or "Dialog" in nc:
            errors.append(
                f"[CR-SSR-101] {cid} — LocalResource sem Suspense com Dialog\n"
                f"             pode causar hydration mismatch — dialog deve vir apos lista"
            )

    # CR-SSR-102: Select com valor inicial dinamico sem SelectionState
    # Apenas quando valor vem de variavel/signal — nao static select
    if "<Select" in nc and "SelectionState" not in nc:
        has_dynamic_value = bool(re.search(
            r"<Select[\s\S]{0,200}?(value=|selected=|current=|default_value=)\s*\w+\.get",
            nc
        ))
        if has_dynamic_value:
            errors.append(
                f"[CR-SSR-102] {cid} — Select com valor dinamico sem SelectionState::Selected\n"
                f"             SSR nao sabe qual item esta selecionado\n"
                f"             usar SelectionState::Selected no item correto"
            )

    # CR-SSR-103: fetch dentro do corpo do Effect — nao em server functions
    # Detecta fetch/gloo_net DENTRO do bloco Effect::new, nao apenas no mesmo arquivo
    effect_blocks = re.findall(r"Effect::new\s*\([^{]*\{([\s\S]{0,500}?)\}\s*\)", nc)
    for block in effect_blocks:
        # reqwest em server fn e SSR — ok. gloo_net e fetch client-side — suspeito
        if re.search(r"gloo_net|wasm_bindgen_futures|JsFuture", block) and "Suspense" not in nc:
            errors.append(
                f"[CR-SSR-103] {cid} — fetch client-side dentro de Effect sem Suspense\n"
                f"             usar LocalResource + Suspense para fetch client-side"
            )
            break

    return errors

def run():
    files = glob.glob(f"{PRODUCTS_DIR}/**/pages/*.rs", recursive=True)
    total_ok = failed = total_err = 0
    for path in sorted(files):
        errs = check_file(path)
        if errs:
            print(f"\n[ERRO] {os.path.relpath(path, PRODUCTS_DIR)}")
            for e in errs: print(f"   {e}")
            failed += 1; total_err += len(errs)
        else:
            total_ok += 1
    print(f"\n{'='*50}")
    print(f"[OK] {total_ok} pages clean")
    if total_err:
        print(f"[FAIL] {failed} pages — {total_err} violations")
        return 1
    print("[OK] SSR contracts canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run())
