#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""
check_mutation_governance.py — MutationObserver Governance

Valida que o MutationObserver nao causa replay loops.

CR-MUT-100: observer observa attributes — causa replay em runtime mutations
CR-MUT-101: observer observa characterData — desnecessario para runtime
CR-MUT-102: observer nao filtra initialized subtrees — causa replay loop
CR-MUT-103: observer processa todos os nodes sem filtro de data-rs-interaction
CR-MUT-104: checagem duplicada de initialized — codigo morto

Usage:
  python3 check_mutation_governance.py
"""
import re, glob, os, sys

INT_BASE = _CANONRS_ROOT + ""

def strip_comments(src):
    return re.sub(r"//[^\n]*", "", src)

def check_observer_file(path):
    errors = []
    try:
        src = open(path).read()
        nc  = strip_comments(src)
    except Exception:
        return errors

    rel = os.path.relpath(path, INT_BASE)

    # CR-MUT-100: set_attributes(true) — replay em state/style mutations
    if re.search(r"set_attributes\s*\(\s*true\s*\)", nc):
        errors.append(
            f"[CR-MUT-100] {rel}\n"
            f"              observer observa attributes=true\n"
            f"              garantia: observer NAO deve reagir a attribute mutations\n"
            f"              risco: data-rs-state, style, class mutations causam replay loop"
        )

    # CR-MUT-101: set_character_data(true)
    if re.search(r"set_character_data\s*\(\s*true\s*\)", nc):
        errors.append(
            f"[CR-MUT-101] {rel}\n"
            f"              observer observa characterData=true\n"
            f"              garantia: characterData mutations sao irrelevantes para runtime init"
        )

    # CR-MUT-102: nao filtra initialized subtrees
    has_initialized_filter = (
        "data-rs-initialized" in nc and
        ("closest" in nc or "has_attribute" in nc)
    )
    if not has_initialized_filter:
        errors.append(
            f"[CR-MUT-102] {rel}\n"
            f"              observer nao filtra subtrees ja inicializados\n"
            f"              garantia: mutations dentro de [data-rs-initialized] DEVEM ser ignoradas\n"
            f"              risco: runtime DOM mutations causam replay infinito"
        )

    # CR-MUT-103: nao filtra por data-rs-interaction
    if "data-rs-interaction" not in nc:
        errors.append(
            f"[CR-MUT-103] {rel}\n"
            f"              observer nao filtra por data-rs-interaction\n"
            f"              garantia: observer DEVE processar apenas nodes dispatchaveis"
        )

    # CR-MUT-104: checagem duplicada de initialized (codigo morto)
    initialized_checks = len(re.findall(r"data-rs-initialized", nc))
    if initialized_checks > 2:
        errors.append(
            f"[CR-MUT-104] {rel}\n"
            f"              {initialized_checks} verificacoes de data-rs-initialized — provavel duplicacao\n"
            f"              garantia: uma unica verificacao de initialized por node processado"
        )

    return errors


def find_observer_files():
    """Find GLOBAL MutationObserver files — replay/init observers only.
    Excludes: compiled bundles, local component observers (single-element scope).
    """
    results = []
    # Only scan runtime observer files — not component-local observers
    RUNTIME_OBSERVER_PATTERNS = [
        f"{INT_BASE}/canonrs-interactions/src/runtime/observer.rs",
        # canonrs-interactions-init/src/runtime/observer.rs is a scoped attr helper — exempt
    ]
    for pattern in RUNTIME_OBSERVER_PATTERNS:
        for path in glob.glob(pattern):
            if ".bak" not in path and "target/" not in path:
                results.append(path)
    # JS loader observers — global scope
    JS_LOADER_PATHS = [
        f"{INT_BASE}/canonrs-client/src/loader/canon-loader.js",
        # canon-init-loader.js uses initAll() which has its own guard — exempt
    ]
    for path in JS_LOADER_PATHS:
        if os.path.exists(path):
            results.append(path)
    return results

def check_js_observer(path):
    errors = []
    try:
        src = open(path).read()
    except Exception:
        return errors
    rel = os.path.relpath(path, INT_BASE)

    # attributes: true in JS observer
    if re.search(r"attributes\s*:\s*true", src):
        # check if attributeFilter is set (scoped — ok)
        if not re.search(r"attributeFilter\s*:", src):
            errors.append(
                f"[CR-MUT-100] {rel}\n"
                f"              JS observer observa attributes=true sem attributeFilter\n"
                f"              garantia: attribute observation DEVE ter attributeFilter restrito"
            )

    # no initialized filter
    if "data-rs-initialized" not in src and "initializedUids" not in src:
        errors.append(
                f"[CR-MUT-102] {rel}\n"
                f"              JS observer nao filtra initialized subtrees"
        )

    return errors


def run(target=None):
    files = find_observer_files()
    all_errors = []

    for path in sorted(files):
        if target and target not in path: continue
        if path.endswith(".js"):
            errs = check_js_observer(path)
        else:
            errs = check_observer_file(path)
        all_errors.extend(errs)

    print(f"\n" + "=" * 50)
    if all_errors:
        for e in all_errors:
            print(f"\n[ERRO] {e}")
        print(f"\n[FAIL] {len(all_errors)} mutation governance violation(s)")
        return 1

    print(f"[OK] {len(files)} observer file(s) clean")
    print("[OK] Mutation governance compliant")
    return 0


if __name__ == "__main__":
    sys.exit(run(sys.argv[1] if len(sys.argv) > 1 else None))