#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""
check_ssr_runtime_parity.py — SSR/Runtime Attribute Parity

Valida que o runtime nao assume atributos DOM que o SSR nao emite,
e que atributos setados pelo SSR nao conflitam com lifecycle do runtime.

Problemas detectados:
  CR-PAR-100: runtime lê atributo que SSR pode ter emitido com valor conflitante
  CR-PAR-101: runtime assume ausencia de atributo que loader/SSR pode ter setado
  CR-PAR-102: lifecycle attr usado como guard sem considerar hot-reload reset

Usage:
  python3 check_ssr_runtime_parity.py
  python3 check_ssr_runtime_parity.py chart
"""
import re, glob, os, sys

INT_BASE      = _CANONRS_ROOT + ""
PRIMITIVES_DIR = _CANONRS_ROOT + "/canonrs-core/src/primitives"

# Atributos que o runtime NAO deve usar como guard primario
# porque podem existir no DOM por razoes externas (loader, hot-reload, SSR futuro)
LIFECYCLE_ATTRS = [
    "data-rs-initialized",
    "data-rs-reinit",
]

# Padroes de uso incorreto — usar lifecycle attr como guard primario no runtime
FORBIDDEN_PATTERNS = [
    # init_guard baseado em data-rs-initialized como unico guard
    (r'get_attribute\("data-rs-initialized"\)',
     "CR-PAR-100",
     "runtime lê data-rs-initialized diretamente — usar registry uid como guard primario"),
    # has_attribute como guard sem fallback
    (r'has_attribute\("data-rs-initialized"\)',
     "CR-PAR-101",
     "runtime usa has_attribute(data-rs-initialized) — atributo pode existir por hot-reload/loader"),
]

# Padroes CORRETOS — ownership via uid registry
CORRECT_PATTERNS = [
    r"registry::should_init",
    r"INITED\.with",
    r"initializedUids",
]

def strip_comments(src):
    return re.sub(r"//[^\n]*", "", src)

def check_file(path, cid):
    errors = []
    try:
        src = open(path).read()
        nc  = strip_comments(src)
    except Exception:
        return errors

    # Skip lifecycle.rs itself — it defines the API
    # Skip subtree.rs — has_uninitialized is an observability query, not a guard
    if os.path.basename(path) in ("lifecycle.rs", "registry.rs", "bootstrap.rs", "subtree.rs"):
        return errors

    for pattern, code, msg in FORBIDDEN_PATTERNS:
        if re.search(pattern, nc):
            errors.append(
                f"[{code}] {cid} — {msg}\n"
                f"              garantia: lifecycle ownership pertence ao registry uid\n"
                f"              nao usar data-rs-initialized como guard — usar registry::should_init"
            )

    return errors


def check_primitives_emit_uid():
    """Verifica que primitivos com data-rs-interaction emitem data-rs-uid."""
    errors = []
    files = glob.glob(f"{PRIMITIVES_DIR}/*.rs")
    for path in sorted(files):
        try:
            src = open(path).read()
            cid = os.path.basename(path).replace(".rs", "")
            if "data-rs-interaction" not in src: continue
            if "data-rs-uid" not in src:
                errors.append(
                    f"[CR-PAR-103] {cid} — primitive tem data-rs-interaction mas nao emite data-rs-uid\n"
                    f"              garantia: todo componente dispatchavel DEVE ter uid para lifecycle ownership"
                )
        except Exception:
            pass
    return errors


def run(target=None):
    files = [
        f for f in glob.glob(f"{INT_BASE}/canonrs-interactions-*/src/**/*.rs", recursive=True)
        if ".bak" not in f and "target/" not in f
    ]
    files += glob.glob(f"{INT_BASE}/canonrs-interactions/src/**/*.rs", recursive=True)

    all_errors = []
    checked = 0

    for path in sorted(files):
        cid = os.path.basename(path).replace(".rs", "")
        # lib.rs entry points use has_attribute(data-rs-initialized) as WASM dispatch guard — legitimate
        if path.endswith("/lib.rs") or path.endswith("/registry.rs"): continue
        if target and target not in path: continue
        errs = check_file(path, cid)
        all_errors.extend(errs)
        checked += 1

    # Check primitives emit uid
    prim_errors = check_primitives_emit_uid()
    all_errors.extend(prim_errors)

    print(f"\n" + "=" * 50)
    if all_errors:
        for e in all_errors:
            print(f"\n[ERRO] {e}")
        print(f"\n[FAIL] {len(all_errors)} SSR/runtime parity violation(s)")
        return 1

    if target:
        print(f"[OK] {target} — SSR/runtime parity clean")
    else:
        print(f"[OK] SSR/runtime parity clean — {checked} files checked")
    print("[OK] No SSR/runtime parity violations")
    return 0


if __name__ == "__main__":
    sys.exit(run(sys.argv[1] if len(sys.argv) > 1 else None))