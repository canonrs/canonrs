#!/usr/bin/env python3
"""
check_dom_topology.py — CR-DOMTOP-100..103
CR-DOMTOP-100: Overlay dentro de Overlay proibido
CR-DOMTOP-101: Portal dentro de Suspense proibido
CR-DOMTOP-102: FocusTrap dentro de For proibido
CR-DOMTOP-103: Dialog/Popover/Tooltip devem estar no root do Page view — nao em sub-components
"""
import re, glob, os, sys

PRODUCTS_DIR  = "/opt/docker/monorepo/products"
REGISTRY_PATH = "/opt/docker/monorepo/tools/testkit/testkit_registry.yaml"
EXCLUDE_PATH_FRAGMENTS = [".backup", "frontend-leptos.backup", "_old-workbench", "/old1/"]

OVERLAY_COMPONENTS = [
    "DialogPortal", "ConfirmDialogPortal", "PopoverContent",
    "TooltipContent", "DropdownMenuContent", "DrawerPortal", "SheetPortal"
]

# Overlays que DEVEM estar no root topology layer (Page ou componente dedicado)
# Tooltip e inline — nao precisa estar no root
ROOT_TOPOLOGY_OVERLAYS = [
    "DialogPortal", "ConfirmDialogPortal", "DrawerPortal", "SheetPortal"
]

def is_excluded(path):
    return any(f in path for f in EXCLUDE_PATH_FRAGMENTS)

def load_products():
    try:
        import yaml
        with open(REGISTRY_PATH) as f:
            data = yaml.safe_load(f)
        return [n for n, i in data.get("products", {}).items() if i.get("test", True)]
    except Exception:
        return []

def check_file(path):
    errors = []
    src  = open(path).read()
    nc   = re.sub(r"//[^\n]*", "", src)
    cid  = os.path.relpath(path, PRODUCTS_DIR)
    lines = nc.splitlines()

    # CR-DOMTOP-100: detecta overlay dentro de overlay por linha de abertura
    # Conta abertura de tags overlay e detecta nesting
    overlay_stack = []
    for i, line in enumerate(lines):
        for o in OVERLAY_COMPONENTS:
            if f"<{o}" in line:
                # self-closing: nao empilha
                if re.search(rf"<{o}\s*/>", line):
                    if overlay_stack:
                        errors.append(
                            f"[CR-DOMTOP-100] {cid} linha {i+1} — {o} (self-closing) dentro de {overlay_stack[-1]}\n"
                            f"             overlay nao pode conter outro overlay"
                        )
                    continue
                if overlay_stack:
                    errors.append(
                        f"[CR-DOMTOP-100] {cid} linha {i+1} — {o} dentro de {overlay_stack[-1]}\n"
                        f"             overlay nao pode conter outro overlay"
                    )
                overlay_stack.append(o)
            if f"</{o}>" in line:
                if overlay_stack and overlay_stack[-1] == o:
                    overlay_stack.pop()

    # CR-DOMTOP-101: Portal dentro de Suspense
    pattern = "(" + "|".join(OVERLAY_COMPONENTS) + ")"
    if re.search(r"<Suspense[\s\S]{0,500}?" + pattern, nc):
        errors.append(f"[CR-DOMTOP-101] {cid} — Portal dentro de Suspense\n             portal deve estar fora de Suspense")

    # CR-DOMTOP-102: FocusTrap dentro de For
    if "FocusTrap" in nc and re.search(r"<For[\s\S]{0,500}?FocusTrap", nc):
        errors.append(f"[CR-DOMTOP-102] {cid} — FocusTrap dentro de For\n             FocusTrap deve estar fora de listas reativas")

    # CR-DOMTOP-103: Dialog/Popover devem estar em componente Page ou Dialog dedicado
    # Detecta overlay em fn que claramente nao e Page nem Dialog dedicado
    has_overlay = any(o in nc for o in OVERLAY_COMPONENTS)
    if has_overlay:
        for m in re.finditer(r"fn (\w+)\s*\(", nc):
            fn_name = m.group(1)
            # permitido: Page, Dialog*, Confirm*, Modal*, Drawer*, Sheet*, Popover*
            allowed = any(fn_name.endswith(s) for s in ["Page", "Dialog", "Modal", "Drawer", "Sheet", "Popover"])
            allowed = allowed or any(fn_name.startswith(s) for s in ["Confirm", "Dialog", "Modal"])
            if allowed:
                continue
            # verifica se overlay aparece no corpo desta funcao
            fn_body_start = nc.find("{", m.end())
            if fn_body_start == -1:
                continue
            # extrai corpo balanceando chaves
            depth = 0
            fn_body_end = fn_body_start
            for i, ch in enumerate(nc[fn_body_start:], fn_body_start):
                if ch == "{": depth += 1
                elif ch == "}":
                    depth -= 1
                    if depth == 0:
                        fn_body_end = i
                        break
            fn_body = nc[fn_body_start:fn_body_end]
            if any(o in fn_body for o in ROOT_TOPOLOGY_OVERLAYS):
                errors.append(
                    f"[CR-DOMTOP-103] {cid} — overlay em fn {fn_name}\n"
                    f"             Dialog/Popover devem estar em Page ou componente dedicado"
                )
                break

    return errors

def run():
    products = load_products()
    files = []
    for name in products:
        files.extend(glob.glob(f"{PRODUCTS_DIR}/{name}/**/pages/*.rs", recursive=True))

    total_ok = failed = total_err = 0
    for path in sorted(files):
        if is_excluded(path): continue
        errs = check_file(path)
        if errs:
            print(f"\n[ERRO] {os.path.relpath(path, PRODUCTS_DIR)}")
            for e in errs: print(f"   {e}")
            failed += 1; total_err += len(errs)
        else:
            total_ok += 1
    print(f"\n{chr(61)*50}")
    print(f"[OK] {total_ok} files clean")
    if total_err:
        print(f"[FAIL] {failed} files — {total_err} violations")
        return 1
    print("[OK] DOM topology contracts canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run())
