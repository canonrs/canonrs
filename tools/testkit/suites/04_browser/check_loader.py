#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""
check_loader.py — Bootstrap architecture governance

Valida que:
1. Grupos no dispatcher.rs == grupos no canon-loader.js
2. Grupos no dispatcher.rs == crates canonrs-interactions-* existentes
3. canon-loader.js nao importa paths de grupo individuais (/wasm/<group>/)
4. canon-loader.js usa bundle unico /wasm/canonrs_interactions.js
"""
import re, os, sys

DISPATCHER = _CANONRS_ROOT + "/canonrs-interactions/src/runtime/dispatcher.rs"
LOADER     = _CANONRS_ROOT + "/canonrs-client/src/loader/canon-loader.js"
INT_BASE   = _CANONRS_ROOT + ""
BUNDLE     = _CANONRS_ROOT + "/canonrs-client/assets/wasm/canonrs_interactions.js"

# grupos que nao precisam de crate proprio
CORE_GROUPS = {"core"}


def parse_dispatcher_groups():
    """Extrai grupos registrados no dispatcher.rs."""
    src = open(DISPATCHER).read()
    return set(re.findall(r'm\.insert\("([^"]+)"\.into\(\)', src))


def parse_loader_imports():
    """Verifica se loader usa bundle unico ou imports por grupo."""
    src = open(LOADER).read()
    # imports por grupo — pattern antigo: /wasm/<group>/canonrs_interactions_<group>.js
    group_imports = re.findall(r"/wasm/([^/]+)/canonrs_interactions_\1\.js", src)
    # bundle unico — correto
    single_bundle = "/wasm/canonrs_interactions.js" in src
    return group_imports, single_bundle


def discover_crates():
    """Descobre crates canonrs-interactions-* existentes."""
    import glob
    dirs = glob.glob(f"{INT_BASE}/canonrs-interactions-*")
    return {os.path.basename(d).replace("canonrs-interactions-", "") for d in dirs}


def run():
    errors = []

    if not os.path.exists(DISPATCHER):
        print(f"[FAIL] dispatcher.rs nao encontrado: {DISPATCHER}")
        return 1
    if not os.path.exists(LOADER):
        print(f"[FAIL] canon-loader.js nao encontrado: {LOADER}")
        return 1

    dispatcher_groups = parse_dispatcher_groups()
    group_imports, single_bundle = parse_loader_imports()
    crates = discover_crates() - CORE_GROUPS

    # CR-LDR-100: loader deve usar bundle unico
    if not single_bundle:
        errors.append("[CR-LDR-100] canon-loader.js nao usa bundle unico\n"
                      "             deve importar /wasm/canonrs_interactions.js\n"
                      "             nao /wasm/<group>/canonrs_interactions_<group>.js")

    # CR-LDR-101: loader nao deve ter imports por grupo
    if group_imports:
        for g in group_imports:
            errors.append(f"[CR-LDR-101] loader importa grupo individual: {g}\n"
                          f"             bundle unico elimina imports por grupo")

    # CR-LDR-102: todo grupo do dispatcher deve ter crate
    for group in sorted(dispatcher_groups - CORE_GROUPS):
        if group not in crates:
            errors.append(f"[CR-LDR-102] dispatcher registra grupo '{group}' sem crate correspondente\n"
                          f"             criar canonrs-interactions-{group}/")

    # CR-LDR-103: todo crate deve estar registrado no dispatcher
    for crate in sorted(crates - CORE_GROUPS):
        if crate not in dispatcher_groups:
            errors.append(f"[CR-LDR-103] crate canonrs-interactions-{crate} sem registro no dispatcher\n"
                          f"             adicionar: m.insert(\"{crate}\".into(), canonrs_interactions_{crate.replace('-','_')}::init_{crate.replace('-','_')})")

    # CR-LDR-104: bundle deve existir em assets/wasm/
    if not os.path.exists(BUNDLE):
        errors.append(f"[CR-LDR-104] bundle nao encontrado: {BUNDLE}\n"
                      f"             executar: wasm-pack build canonrs-interactions")

    print(f"dispatcher groups : {sorted(dispatcher_groups)}")
    print(f"crates found      : {sorted(crates)}")
    print(f"single bundle     : {single_bundle}")
    print(f"group imports     : {group_imports or 'none'}")

    print(f"\n{'='*50}")
    if errors:
        for e in errors: print(f"[ERRO] {e}")
        print(f"[FAIL] {len(errors)} violations found")
        return 1
    print("[OK] Bootstrap architecture canonical")
    return 0


if __name__ == "__main__":
    sys.exit(run())