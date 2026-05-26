#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""
check_build_parity.py — CR-BLD-300..303
CR-BLD-300: release build deve gerar mesmos artifacts do debug
CR-BLD-301: OUT_DIR generated files obrigatorios em release
CR-BLD-302: build.rs deve funcionar em debug e release
CR-BLD-303: generated/* parity check
"""
import os, sys, glob

CANONRS_DIR = _CANONRS_ROOT + ""
TARGET_DIR  = os.path.join(_CANONRS_ROOT, "target")

REQUIRED_GENERATED = [
    "component_meta.rs",
    "block_meta.rs",
    "catalog.rs",
]

def check_release_artifacts():
    errors = []

    # CR-BLD-301: verifica arquivos gerados em release
    release_builds = glob.glob(f"{TARGET_DIR}/release/build/canonrs-core-*/out/generated/")
    if not release_builds:
        errors.append(
            "[CR-BLD-301] release build — nenhum OUT_DIR encontrado em target/release\n"
            "             rodar: cargo build -p canonrs-core --release"
        )
        return errors

    for build_dir in release_builds:
        for required in REQUIRED_GENERATED:
            if not os.path.exists(os.path.join(build_dir, required)):
                errors.append(
                    f"[CR-BLD-301] release — {required} ausente em {build_dir}\n"
                    f"             build.rs nao gerou artifact em release"
                )

    # CR-BLD-302: build.rs nao pode estar vazio
    build_rs = os.path.join(CANONRS_DIR, "canonrs-core/build.rs")
    if os.path.exists(build_rs):
        src = open(build_rs).read().strip()
        if src == "fn main() {}" or len(src) < 50:
            errors.append(
                "[CR-BLD-302] canonrs-core/build.rs — vazio ou stub\n"
                "             build.rs deve chamar os generators"
            )

    # CR-BLD-303: parity debug vs release
    debug_builds  = glob.glob(f"{TARGET_DIR}/debug/build/canonrs-core-*/out/generated/")
    release_builds = glob.glob(f"{TARGET_DIR}/release/build/canonrs-core-*/out/generated/")

    if debug_builds and release_builds:
        debug_files   = set(os.path.basename(f) for d in debug_builds for f in glob.glob(f"{d}*.rs"))
        release_files = set(os.path.basename(f) for d in release_builds for f in glob.glob(f"{d}*.rs"))
        missing = debug_files - release_files
        if missing:
            errors.append(
                f"[CR-BLD-303] parity — arquivos em debug mas nao em release: {missing}\n"
                f"             release build nao gerou todos os artifacts"
            )

    return errors

def run():
    errs = check_release_artifacts()
    if errs:
        print("\n[ERRO] build parity")
        for e in errs: print(f"   {e}")
        print(f"\n{chr(61)*50}")
        print(f"[FAIL] 1 checks — {len(errs)} violations")
        return 1
    print(f"\n{chr(61)*50}")
    print("[OK] 1 checks clean")
    print("[OK] Build parity canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run())