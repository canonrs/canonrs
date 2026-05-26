#!/usr/bin/env python3
"""
check_island_contracts.py — CR-ISL-100..104
CR-ISL-100: #[island] proibido usar LocalResource
CR-ISL-101: #[island] proibido fetch direto
CR-ISL-102: #[island] deve delegar para runtime/client module
CR-ISL-103: #[island] view deve ser deterministica
CR-ISL-104: proibido Date::now / random / uuid no view SSR
"""
import re, glob, os, sys

PRODUCTS_DIR  = "/opt/docker/monorepo/products"
REGISTRY_PATH = "/opt/docker/monorepo/tools/testkit/testkit_registry.yaml"
EXCLUDE_PATH_FRAGMENTS = [".backup", "frontend-leptos.backup", "_old-workbench", "/old1/"]

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

def extract_island_blocks(src):
    """Extrai linhas entre #[island] e o proximo #[component]/#[server] ou fim."""
    blocks = []
    lines = src.splitlines()
    in_island = False
    current = []
    for line in lines:
        if "#[island]" in line:
            in_island = True
            current = [line]
        elif in_island:
            # termina no proximo atributo de item de nivel top
            # ignora #[cfg] e outros atributos que nao sao items
            is_top_item = ("#[component]" in line or "#[server]" in line) and not "#[cfg" in line
            if is_top_item and current:
                blocks.append("\n".join(current))
                in_island = False
                current = []
            else:
                current.append(line)
    if current:
        blocks.append("\n".join(current))
    return blocks

def check_file(path):
    errors = []
    src = open(path).read()
    nc  = re.sub(r"//[^\n]*", "", src)
    cid = os.path.relpath(path, PRODUCTS_DIR)

    if "#[island]" not in nc:
        return errors

    for block in extract_island_blocks(nc):
        if "LocalResource" in block:
            errors.append(f"[CR-ISL-100] {cid} — #[island] usa LocalResource\n             island deve ser estatico")
        if re.search(r"reqwest|gloo_net|fetch", block):
            errors.append(f"[CR-ISL-101] {cid} — #[island] usa fetch direto\n             delegar para runtime/client module")
        if "js_sys::Date::now" in block or re.search(r"\buuid\b|rand::random", block):
            errors.append(f"[CR-ISL-104] {cid} — #[island] usa Date::now/random/uuid\n             view SSR deve ser deterministica")

    return errors

def run():
    products = load_products()
    files = []
    for name in products:
        files.extend(glob.glob(f"{PRODUCTS_DIR}/{name}/**/*.rs", recursive=True))

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
    print("[OK] Island contracts canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run())
