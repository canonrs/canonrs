#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""
check_bundle.py — WASM bundle artifact governance

Valida que:
1. Bundle canonrs_interactions.js existe em assets/wasm/
2. Bundle canonrs_interactions_bg.wasm existe em assets/wasm/
3. Loader nao referencia paths de grupo individuais /wasm/<group>/
4. Todos os grupos do dispatcher existem como modulos no bundle JS
"""
import re, os, sys, glob

ASSETS_WASM  = _CANONRS_ROOT + "/canonrs-client/assets/wasm"
LOADER       = _CANONRS_ROOT + "/canonrs-client/src/loader/canon-loader.js"
DISPATCHER   = _CANONRS_ROOT + "/canonrs-interactions/src/runtime/dispatcher.rs"
BUNDLE_JS    = os.path.join(ASSETS_WASM, "canonrs_interactions.js")
BUNDLE_WASM  = os.path.join(ASSETS_WASM, "canonrs_interactions_bg.wasm")


def parse_dispatcher_groups():
    src = open(DISPATCHER).read()
    return set(re.findall(r'm\.insert\("([^"]+)"\.into\(\)', src))


def run():
    errors = []

    # CR-BDL-100: bundle JS deve existir
    if not os.path.exists(BUNDLE_JS):
        errors.append(
            f"[CR-BDL-100] bundle JS ausente: {BUNDLE_JS}\n"
            f"             executar: wasm-pack build canonrs-interactions"
        )

    # CR-BDL-101: bundle WASM deve existir
    if not os.path.exists(BUNDLE_WASM):
        errors.append(
            f"[CR-BDL-101] bundle WASM ausente: {BUNDLE_WASM}\n"
            f"             executar: wasm-pack build canonrs-interactions"
        )

    # CR-BDL-102: loader nao deve referenciar paths por grupo
    if os.path.exists(LOADER):
        loader_src = open(LOADER).read()
        group_refs = re.findall(r"/wasm/([^/\"]+)/canonrs_interactions_\1\.js", loader_src)
        for g in group_refs:
            errors.append(
                f"[CR-BDL-102] loader referencia bundle por grupo: /wasm/{g}/\n"
                f"             usar bundle unico: /wasm/canonrs_interactions.js"
            )

    # CR-BDL-103: bundle JS deve exportar init_subtree e init_all
    if os.path.exists(BUNDLE_JS):
        bundle_src = open(BUNDLE_JS).read()
        for fn in ["init_subtree", "init_all"]:
            if fn not in bundle_src:
                errors.append(
                    f"[CR-BDL-103] bundle nao exporta: {fn}\n"
                    f"             funcao obrigatoria para bootstrap e MutationObserver"
                )

    # CR-BDL-104: bundle JS deve conter init_ para cada grupo do dispatcher
    if os.path.exists(BUNDLE_JS) and os.path.exists(DISPATCHER):
        bundle_src = open(BUNDLE_JS).read()
        groups = parse_dispatcher_groups()
        for group in sorted(groups):
            fn_name = f"init_{group.replace('-', '_')}"
            if fn_name not in bundle_src:
                errors.append(
                    f"[CR-BDL-104] bundle nao contem: {fn_name}\n"
                    f"             grupo '{group}' registrado no dispatcher mas ausente no bundle"
                )

    print(f"\n{'='*50}")
    if errors:
        for e in errors: print(f"[ERRO] {e}")
        print(f"[FAIL] {len(errors)} bundle violations found")
        return 1
    print("[OK] Bundle architecture canonical")
    return 0


if __name__ == "__main__":
    sys.exit(run())