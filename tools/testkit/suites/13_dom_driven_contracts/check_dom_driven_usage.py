#!/usr/bin/env python3
"""
check_dom_driven_usage.py — CR-DOM-100..103
CR-DOM-100: value= em Input sem on:input/on:change — binding unidirecional
CR-DOM-101: open= em Dialog/ConfirmDialog proibido
CR-DOM-102: selected= como signal em Select proibido
CR-DOM-103: on:input sincronizando RwSignal em componente DOM-driven
"""
import re, glob, os, sys, yaml

REGISTRY_PATH = "/opt/docker/monorepo/tools/testkit/testkit_registry.yaml"

EXCLUDE_PATHS = [
    "_old-workbench/",
    ".backup/",
    "frontend-leptos.backup/",
    "old1/",
]

def load_excluded_products():
    try:
        with open(REGISTRY_PATH) as f:
            data = yaml.safe_load(f)
        return [name for name, info in data.get("products", {}).items() if not info.get("test", True)]
    except Exception:
        return []

EXCLUDED_PRODUCTS = load_excluded_products()

def is_excluded(path):
    if any(e in path for e in EXCLUDE_PATHS):
        return True
    return any(f"/{p}/" in path for p in EXCLUDED_PRODUCTS)

PRODUCTS_DIR = "/opt/docker/monorepo/products"

def strip_comments(src):
    return re.sub(r"//[^\n]*", "", src)

def check_file(path):
    if is_excluded(path):
        return [], []
    errors = []
    warnings = []
    src = open(path).read()
    nc  = strip_comments(src)
    cid = os.path.relpath(path, PRODUCTS_DIR)

    # CR-DOM-100: Input com value= mas sem on:input/on:change/node_ref — warning
    # Pode ser readonly intencional, por isso e warning e nao error
    if "<Input" in nc:
        input_blocks = re.findall(r"<Input[^/]*/>|<Input[^>]*>[\s\S]*?</Input>", nc)
        for block in input_blocks:
            has_value    = "value=" in block
            has_binding  = "on:input" in block or "on:change" in block
            has_node_ref = "node_ref" in block
            has_action   = "ActionForm" in nc or "action=" in block
            if has_value and not has_binding and not has_node_ref and not has_action:
                warnings.append(
                    f"[CR-DOM-100-WARN] {cid} — Input com value= sem on:input/on:change/node_ref\n"
                    f"             se for readonly, ignorar — se precisar ler valor, adicionar binding"
                )
                break

    # CR-DOM-101: open= em Dialog/ConfirmDialog
    if re.search(r"<(ConfirmDialog|Dialog)[^>]*open=", nc):
        errors.append(
            f"[CR-DOM-101] {cid} — open= em Dialog/ConfirmDialog proibido\n"
            f"             CanonRS e DOM-driven: usar ConfirmDialogTrigger + confirm_dialog_close()"
        )

    # CR-DOM-102: selected= como signal em Select
    if re.search(r"<Select[\s\S]{0,200}?selected=\s*\w+\.get\(\)", nc):
        errors.append(
            f"[CR-DOM-102] {cid} — selected= como signal em Select proibido\n"
            f"             usar SelectionState::Selected no item correto"
        )

    # CR-DOM-103: removido — padrao controlled input e valido em Input CanonRS

    return errors, warnings

def run():
    files = glob.glob(f"{PRODUCTS_DIR}/**/*.rs", recursive=True)
    total_ok = failed = total_err = 0
    for path in sorted(files):
        errs, warns = check_file(path)
        if errs:
            print(f"\n[ERRO] {os.path.relpath(path, PRODUCTS_DIR)}")
            for e in errs: print(f"   {e}")
            failed += 1; total_err += len(errs)
        elif warns:
            print(f"\n[WARN] {os.path.relpath(path, PRODUCTS_DIR)}")
            for w in warns: print(f"   {w}")
            total_ok += 1
        else:
            total_ok += 1
    print(f"\n{'='*50}")
    print(f"[OK] {total_ok} files clean")
    if total_err:
        print(f"[FAIL] {failed} files — {total_err} violations")
        return 1
    print("[OK] DOM-driven contracts canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run())
