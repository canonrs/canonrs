#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""
check_preview.py -- Preview layer governance
Criterios: CANONRS PREVIEW (0.5 pt)
Localizacao: canonrs-server/src/ui/<component>/preview.rs

Imports obrigatorios:
  use leptos::prelude::*;
  use canonrs::prelude::*;  (ou super::*_boundary:: para compatibilidade)

Regras:
- Usa obrigatoriamente Boundary (nunca UI ou Primitive direto)
- Sem signals, sem logica de negocio, sem fetch
"""
import re, glob, os, sys

UI_DIR = _CANONRS_ROOT + "/canonrs-server/src/ui"

SIGNAL_PATTERNS = ["create_signal", "signal(", "RwSignal::new", "create_rw_signal", "create_memo"]

# CR-PRV-206: sufixos semanticos permitidos em tipos de canonrs_core::primitives
# PERMITIDO: enums de estado/variante/contrato
# PROIBIDO: componente renderizavel (PascalCase puro sem sufixo semantico)
ALLOWED_SUFFIXES = (
    "State", "Variant", "Size", "Side", "Mode",
    "Orientation", "Selection", "Interactivity", "Hint",
    "Contract", "Meta", "Item",
)


def check_file(path, cid):
    errors = []
    src = open(path).read()

    # CR-PRV-200: imports canonicos
    has_leptos_prelude  = "use leptos::prelude::*" in src
    has_canonrs_prelude = "use canonrs::prelude::*" in src
    has_boundary_import = bool(re.search(r"use (super|crate)::.*_boundary::", src))
    imports_ui_direct   = bool(re.search(r"use (super|crate)::.*_ui::", src))
    uses_primitive_view = bool(re.search(r"<\w+Primitive", src))
    has_valid_import    = has_canonrs_prelude or has_boundary_import

    if not has_leptos_prelude:
        errors.append(f"[CR-PRV-200] {cid}/preview - use leptos::prelude::* ausente")
    if not has_valid_import:
        errors.append(f"[CR-PRV-201] {cid}/preview - nao importa canonrs::prelude::* nem boundary")
    if imports_ui_direct:
        errors.append(f"[CR-PRV-202] {cid}/preview - importa _ui diretamente")
    if uses_primitive_view:
        errors.append(f"[CR-PRV-203] {cid}/preview - usa *Primitive diretamente no view!")

    # CR-PRV-204: signals proibidos
    for sig in SIGNAL_PATTERNS:
        if sig in src:
            errors.append(f"[CR-PRV-204] {cid}/preview - signal proibido: {sig}")
            break

    # CR-PRV-205: logica de negocio proibida
    for bl in ["fetch(", "reqwest::", "http::", "async fn", "spawn_local", "use_navigate"]:
        if bl in src:
            errors.append(f"[CR-PRV-205] {cid}/preview - logica de negocio proibida: {bl}")
            break

    # CR-PRV-206: semantic ownership -- canonrs_core::primitives
    # PERMITIDO: layout::*, tipos com sufixo semantico (State/Variant/Size/Mode/etc)
    # PROIBIDO: identificador PascalCase puro (componente renderizavel)
    for m in re.finditer(r"use canonrs_core::primitives::([^;]+);", src):
        segment = m.group(1)
        if segment.startswith("layout::"):
            continue
        identifiers = re.findall(r"([A-Z][a-zA-Z0-9]+)", segment)
        for ident in identifiers:
            alias_m = re.search(re.escape(ident) + r" as ([A-Z][a-zA-Z0-9]+)", segment)
            check = alias_m.group(1) if alias_m else ident
            if not any(check.endswith(s) for s in ALLOWED_SUFFIXES):
                errors.append(
                    f"[CR-PRV-206] {cid}/preview - componente renderizavel proibido: {check}"
                )
                break

    return errors


def run(target=None):
    files = glob.glob(f"{UI_DIR}/**/preview.rs", recursive=True)
    if not files:
        print(f"[FAIL] 0 files analyzed -- path: {UI_DIR}")
        return 1
    total_ok = total_err = failed = 0
    for path in sorted(files):
        cid = os.path.basename(os.path.dirname(path))
        if target and cid != target: continue
        errs = check_file(path, cid)
        if errs:
            print(f"[ERRO] {cid.upper()}")
            for e in errs: print(f"   {e}")
            failed += 1
            total_err += len(errs)
        else:
            total_ok += 1
            if target: print(f"[OK] {cid.upper()} -- clean")
    print(f"={50*chr(61)}")
    print(f"[OK] {total_ok} previews clean")
    if total_err:
        print(f"[FAIL] {failed} components failed -- {total_err} violations found")
        return 1
    print("[OK] All previews canonical")
    return 0


if __name__ == "__main__":
    sys.exit(run(sys.argv[1] if len(sys.argv) > 1 else None))