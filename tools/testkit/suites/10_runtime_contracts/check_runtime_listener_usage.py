#!/usr/bin/env python3
"""
check_runtime_listener_usage.py — CR-RT-100..102
CR-RT-100: add_event_listener fora de canvas_events.rs (canvas-editor only)
CR-RT-101: listeners dentro de Effect proibido
CR-RT-102: listeners sem .forget() e sem listeners:: proibido
"""
import re, glob, os, sys, yaml

PRODUCTS_DIR  = "/opt/docker/monorepo/products"
REGISTRY_PATH = "/opt/docker/monorepo/tools/testkit/testkit_registry.yaml"

def load_excluded_products():
    try:
        with open(REGISTRY_PATH) as f:
            data = yaml.safe_load(f)
        excluded = []
        for name, info in data.get("products", {}).items():
            if not info.get("test", True):
                excluded.append(name)
        return excluded
    except Exception:
        return []

EXCLUDED_PRODUCTS = load_excluded_products()

def is_excluded(path):
    return any(f"/{p}/" in path or path.endswith(f"/{p}") for p in EXCLUDED_PRODUCTS)

def strip_comments(src):
    return re.sub(r"//[^\n]*", "", src)

def check_file(path):
    if is_excluded(path):
        return []
    errors = []
    src = open(path).read()
    nc  = strip_comments(src)
    cid = os.path.relpath(path, PRODUCTS_DIR)

    if "add_event_listener" not in nc:
        return errors

    # CR-RT-100: canvas-editor — listener fora de canvas_events.rs
    if "canvas-editor" in path and "canvas_events.rs" not in path:
        errors.append(
            f"[CR-RT-100] {cid} — add_event_listener fora de canvas_events.rs\n"
            f"             mover listeners para canvas_events.rs"
        )

    # CR-RT-101: listener dentro de Effect
    if re.search(r"Effect::new[\s\S]{0,300}add_event_listener", nc):
        errors.append(
            f"[CR-RT-101] {cid} — add_event_listener dentro de Effect\n"
            f"             listeners nao devem ser registrados em Effects reativos"
        )

    # CR-RT-102: listener sem ownership — sem .forget() e sem listeners::
    has_forget    = ".forget()" in nc
    has_listeners = "listeners::" in nc
    if not has_forget and not has_listeners:
        errors.append(
            f"[CR-RT-102] {cid} — add_event_listener sem .forget() ou listeners::\n"
            f"             listener permanente deve ter ownership declarado"
        )

    return errors

def run():
    files = glob.glob(f"{PRODUCTS_DIR}/**/*.rs", recursive=True)
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
    print(f"[OK] {total_ok} files clean")
    if total_err:
        print(f"[FAIL] {failed} files — {total_err} violations")
        return 1
    print("[OK] Runtime listener contracts canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run())
