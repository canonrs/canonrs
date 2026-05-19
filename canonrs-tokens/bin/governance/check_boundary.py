#!/usr/bin/env python3
"""
check_boundary.py — Boundary layer governance
Critérios: CRITÉRIOS DE AVALIAÇÃO — CANONRS — BOUNDARY (2 pts)

Tipos:
- Tipo 1: Passthrough — 100% proxy do UI, zero lógica, zero branching
  node_ref: Option<NodeRef<T>> repassado com unwrap_or_default() ao UI
- Tipo 2: Init — normaliza props, delega para canonrs-interactions-init
- Tipo 3: Interaction — composição estrutural, delega para canonrs-interactions-{group}

PROIBIDO em todos os tipos:
- signals, closures reativas, lógica de negócio, reconstrução de estado, DOM direto
"""

import re
import glob
import os
import sys

UI_DIR = os.path.join(
    os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
    "../../canonrs-server/src/ui"
)

BUILDER_DIR = UI_DIR

BOUNDARY_HTML_EXCEPTIONS = [
    "alert_dialog", "drawer", "sheet", "status_dot",
    "switch", "toggle_group", "toolbar"
]

COMPOSITE_EXCEPTIONS = ["sidebar", "data_table", "dropdown_menu"]


def get_boundary_type(component_id: str) -> str:
    import yaml
    builder = os.path.join(BUILDER_DIR, component_id, "builder.yaml")
    if not os.path.exists(builder):
        return ""
    with open(builder) as f:
        raw = yaml.safe_load(f)
    return raw.get("boundary_type", "") if raw else ""


def check_boundary_file(path: str, component_id: str, boundary_type: str) -> list:
    errors = []
    with open(path) as f:
        src = f.read()

    # CR-370: signals proibidos em todos os tipos
    for sig in ["create_signal", "signal(", "RwSignal::new", "create_rw_signal", "create_memo"]:
        if sig in src:
            errors.append(f"[CR-370] {component_id}_boundary.rs -- signal proibido: {sig}\n            boundary nao cria estado reativo")
            break

    # CR-371: closures reativas proibidas
    if re.search(r'move\s*\|\s*\|', src):
        errors.append(f"[CR-371] {component_id}_boundary.rs -- closure reativa proibida\n            boundary nao tem reatividade")

    # CR-372: lógica de negócio proibida
    for bl in ["use_navigate", "fetch(", "reqwest::", "http::", "async fn", "spawn_local"]:
        if bl in src:
            errors.append(f"[CR-372] {component_id}_boundary.rs -- logica de negocio proibida: {bl}")
            break

    # CR-373: passthrough sem branching excessivo
    if boundary_type == "passthrough":
        match_arms = re.findall(r'=>', src)
        if len(match_arms) > 4:
            errors.append(f"[CR-373] {component_id}_boundary.rs -- passthrough com branching excessivo ({len(match_arms)} arms)")

    # CR-374: provide_context proibido
    if "provide_context" in src:
        errors.append(f"[CR-374] {component_id}_boundary.rs -- provide_context proibido")

    # CR-375: HTML direto proibido
    if any(e in component_id for e in BOUNDARY_HTML_EXCEPTIONS):
        return errors
    is_composite = 'unified' in src.lower() or 'composite' in src.lower() or component_id in COMPOSITE_EXCEPTIONS
    if not is_composite:
        view_blocks = re.findall(r'view!\s*\{(.+?)^\s*\}', src, re.DOTALL | re.MULTILINE)
        view_content = "\n".join(view_blocks) if view_blocks else ""
        html_tags = re.findall(r'<(div|span|button|input|textarea|select|form|ul|ol|li|table)\b', view_content)
        if html_tags:
            errors.append(f"[CR-375] {component_id}_boundary.rs -- HTML direto proibido: <{html_tags[0]}>\n            boundary delega para UI, nao reconstroi DOM")

    # CR-344: deve importar UI
    has_ui_import = (
        bool(re.search(r'use (super|crate)::.*_ui::', src)) or
        bool(re.search(r'pub use crate::', src)) or
        bool(re.search(r'super::\w+_ui::', src)) or
        bool(re.search(r'use super::\w+_boundary::', src))
    )
    if not has_ui_import and component_id not in ['data_table']:
        errors.append(f"[CR-344] {component_id}_boundary.rs -- boundary NAO importa UI\n            todo boundary DEVE importar seu UI")

    # CR-342: boundary usa primitives sem UI
    if component_id not in ['data_table']:
        prim_pat = re.findall(r'use canonrs_core::primitives::{([^}]+)}', src)
        if prim_pat:
            has_ui = bool(re.search(r'use (super|crate)::.*_ui::', src)) or bool(re.search(r'use super::\w+_boundary::', src))
            if not has_ui:
                errors.append(f"[CR-342] {component_id}_boundary.rs -- usa primitives diretamente sem UI")

    return errors


def run(target: str = None) -> int:
    total_ok = 0
    total_errors = 0

    files = glob.glob(f"{UI_DIR}/**/*_boundary.rs", recursive=True)

    for path in sorted(files):
        component_id = os.path.basename(path).replace("_boundary.rs", "")
        if target and component_id != target:
            continue
        boundary_type = get_boundary_type(component_id)
        errors = check_boundary_file(path, component_id, boundary_type)
        if errors:
            print(f"\n[ERRO] {component_id.upper()}")
            for e in errors:
                print(f"   {e}")
            total_errors += len(errors)
        else:
            total_ok += 1
            if target:
                print(f"\n[OK] {component_id.upper()} -- clean")

    print(f"\n{'='*50}")
    print(f"[OK] {total_ok} boundaries clean")
    if total_errors:
        print(f"[FAIL] {total_errors} violations found")
        return 1
    print("[OK] All boundaries canonical")
    return 0


if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else None
    sys.exit(run(target))
