#!/usr/bin/env python3
"""
check_overlay_usage.py — CR-OVL-USG-100..104
CR-OVL-USG-100: overlay dentro de For proibido
CR-OVL-USG-101: overlay dentro de slot! proibido
CR-OVL-USG-102: overlay dentro de Suspense proibido
CR-OVL-USG-103: overlay com position:absolute proibido — usar fixed
CR-OVL-USG-104: overlay sem isolation:isolate quando usa style inline
"""
import re, glob, os, sys

PRODUCTS_DIR = "/opt/docker/monorepo/products"

OVERLAY_COMPONENTS = [
    "DialogPortal", "ConfirmDialogPortal", "PopoverContent",
    "TooltipContent", "DropdownMenuContent", "DrawerPortal",
    "SheetPortal", "ModalPortal",
]

def strip_comments(src):
    return re.sub(r"//[^\n]*", "", src)

def check_file(path):
    errors = []
    src = open(path).read()
    nc  = strip_comments(src)
    cid = os.path.relpath(path, PRODUCTS_DIR)

    has_overlay = any(o in nc for o in OVERLAY_COMPONENTS)
    if not has_overlay:
        return errors

    pattern = "(" + "|".join(OVERLAY_COMPONENTS) + ")"

    # CR-OVL-USG-100: overlay dentro de For
    if re.search(r"<For[^>]*>[\s\S]{0,500}?" + pattern, nc):
        errors.append(
            f"[CR-OVL-USG-100] {cid} — overlay dentro de For\n"
            f"             overlay deve ficar fora de listas reativas"
        )

    # CR-OVL-USG-101: overlay dentro de slot! reativo
    if re.search(r"slot!\s*\(move[\s\S]{0,300}?" + pattern, nc):
        errors.append(
            f"[CR-OVL-USG-101] {cid} — overlay dentro de slot!\n"
            f"             overlay deve ficar fora de slots reativos"
        )

    # CR-OVL-USG-102: overlay dentro de Suspense
    if re.search(r"<Suspense[^>]*>[\s\S]{0,500}?" + pattern, nc):
        errors.append(
            f"[CR-OVL-USG-102] {cid} — overlay dentro de Suspense\n"
            f"             overlay deve ficar fora de Suspense"
        )

    # CR-OVL-USG-103: overlay com position:absolute
    # Verificar nas linhas proximas ao componente overlay
    lines = open(path).readlines()
    for i, line in enumerate(lines):
        if any(o in line for o in OVERLAY_COMPONENTS):
            context = "".join(lines[max(0,i-2):i+5])
            if "position:absolute" in context or "position: absolute" in context:
                errors.append(
                    f"[CR-OVL-USG-103] {cid} linha {i+1} — overlay com position:absolute\n"
                    f"             overlay deve usar position:fixed para stacking correto"
                )

    # CR-OVL-USG-104: overlay com style inline sem isolation:isolate
    # Verifica bloco de 15 linhas apos o componente overlay
    for i, line in enumerate(lines):
        if any(o in line for o in OVERLAY_COMPONENTS):
            context = "".join(lines[i:i+15])
            if "style=" in context and "isolation:isolate" not in context:
                errors.append(
                    f"[CR-OVL-USG-104] {cid} linha {i+1} — overlay com style= sem isolation:isolate\n"
                    f"             adicionar isolation:isolate para stacking context correto"
                )

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
    print("[OK] Overlay contracts canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run())
