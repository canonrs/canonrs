#!/usr/bin/env python3
"""
check_build_artifacts.py — CR-BLD-200..203
CR-BLD-200: frontend wasm > 10MB = FAIL
CR-BLD-201: cargo leptos watch sem --release = FAIL no Makefile
CR-BLD-202: Makefile dev deve usar --release
CR-BLD-203: pkg/*.wasm deve ter origem declarada no Leptos.toml ou Cargo.toml
"""
import os, sys, glob, re

PRODUCTS_DIR  = "/opt/docker/monorepo/products"
REGISTRY_PATH = "/opt/docker/monorepo/tools/testkit/testkit_registry.yaml"
WASM_MAX_MB   = 10

EXCLUDE_PATH_FRAGMENTS = [
    ".backup", "frontend-leptos.backup", "_old-workbench", "/old1/", "/old2/",
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

def check_product(name):
    errors = []
    pattern = f"{PRODUCTS_DIR}/{name}/*frontend*"
    dirs = [d for d in glob.glob(pattern) if os.path.isdir(d) and not is_excluded(d)]

    for frontend_dir in dirs:
        label = frontend_dir.replace(PRODUCTS_DIR + "/", "")

        # CR-BLD-200: wasm > 10MB
        for wasm in glob.glob(f"{frontend_dir}/**/*.wasm", recursive=True):
            if is_excluded(wasm):
                continue
            size_mb = os.path.getsize(wasm) / (1024 * 1024)
            if size_mb > WASM_MAX_MB:
                errors.append(
                    f"[CR-BLD-200] {label} — {os.path.basename(wasm)} = {size_mb:.1f}MB (max {WASM_MAX_MB}MB)\n"
                    f"             usar cargo leptos build --release"
                )

        # CR-BLD-201/202: Makefile dev deve usar --release
        makefile = os.path.join(frontend_dir, "Makefile")
        if os.path.exists(makefile):
            src = open(makefile).read()
            if "cargo leptos watch" in src and "--release" not in src:
                errors.append(
                    f"[CR-BLD-201] {label}/Makefile — cargo leptos watch sem --release\n"
                    f"             Canon Rule #96: dev deve usar --release"
                )
            if "cargo leptos serve" in src and "--release" not in src:
                errors.append(
                    f"[CR-BLD-202] {label}/Makefile — cargo leptos serve sem --release"
                )

        # CR-BLD-203: pkg/*.wasm deve ter origem declarada
        pkg_dir = os.path.join(frontend_dir, "site", "pkg")
        if os.path.exists(pkg_dir):
            wasms = glob.glob(f"{pkg_dir}/*.wasm")
            if wasms:
                has_leptos_toml = os.path.exists(os.path.join(frontend_dir, "Leptos.toml"))
                has_cargo_meta  = False
                cargo = os.path.join(frontend_dir, "Cargo.toml")
                if os.path.exists(cargo):
                    has_cargo_meta = "metadata.leptos" in open(cargo).read()
                if not has_leptos_toml and not has_cargo_meta:
                    errors.append(
                        f"[CR-BLD-203] {label} — pkg/*.wasm sem origem declarada\n"
                        f"             adicionar Leptos.toml com [[workspace]] ou [package.metadata.leptos]"
                    )

    return errors

def run():
    products = load_products()
    total_ok = failed = total_err = 0
    for name in sorted(products):
        errs = check_product(name)
        if errs:
            print(f"\n[ERRO] {name}")
            for e in errs: print(f"   {e}")
            failed += 1; total_err += len(errs)
        else:
            total_ok += 1
    print(f"\n{chr(61)*50}")
    print(f"[OK] {total_ok} products clean")
    if total_err:
        print(f"[FAIL] {failed} products — {total_err} violations")
        return 1
    print("[OK] Build artifacts canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run())
