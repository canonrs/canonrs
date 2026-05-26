#!/usr/bin/env python3
import re, glob, os, sys

PRODUCTS_DIR = "/opt/docker/monorepo/products"

def strip_comments(src):
    return re.sub(r"//[^\n]*", "", src)

def check_file(path):
    errors = []
    src = strip_comments(open(path).read())
    cid = os.path.relpath(path, PRODUCTS_DIR)

    if "ConfirmDialogPortal" not in src and "DialogPortal" not in src:
        return errors

    forbidden = [
        (r"slot!\s*\(move[\s\S]{0,200}?(ConfirmDialogPortal|DialogPortal)", "slot!"),
        (r"<For[^>]*>[\s\S]{0,500}?(ConfirmDialogPortal|DialogPortal)", "For"),
        (r"<Suspense[^>]*>[\s\S]{0,500}?(ConfirmDialogPortal|DialogPortal)", "Suspense"),
        (r"body\s*=\s*slot![\s\S]{0,1000}?(ConfirmDialogPortal|DialogPortal)", "body slot!"),
    ]

    for pattern, label in forbidden:
        if re.search(pattern, src):
            errors.append(
                f"[CR-USG-DLG-100] {cid} — Dialog/ConfirmDialogPortal dentro de {label}\n"
                f"                  portal deve ficar fora de regioes reativas/listas/slots"
            )

    return errors

def run():
    files = glob.glob(f"{PRODUCTS_DIR}/**/pages/*.rs", recursive=True)
    total_ok = failed = total_err = 0

    for path in sorted(files):
        errs = check_file(path)
        if errs:
            print(f"\n[ERRO] {os.path.relpath(path, PRODUCTS_DIR)}")
            for e in errs:
                print(f"   {e}")
            failed += 1
            total_err += len(errs)
        else:
            total_ok += 1

    print("\n" + "=" * 50)
    print(f"[OK] {total_ok} pages clean")
    if total_err:
        print(f"[FAIL] {failed} pages — {total_err} dialog usage violations")
        return 1
    print("[OK] Dialog usage canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run())
