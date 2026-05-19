#!/usr/bin/env python3
"""
check_ui.py — UI layer governance
Critérios: CRITÉRIOS DE AVALIAÇÃO — CANONRS — UI (2 pts)

Regras:
- 100% proxy do Primitive — nunca HTML direto
- Não recria DOM
- Não cria estado (sem signal, provide_context, use_navigate)
- Apenas composição — recebe props, passa para Primitive
- node_ref: Option<NodeRef<T>> com unwrap_or_default() ao passar para Primitive — permitido
"""

import re
import glob
import os
import sys


UI_DIR = os.path.join(
    os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
    "../../canonrs-server/src/ui"
)

# Exceções: componentes com rendering dinâmico legítimo
DYNAMIC_RENDER_EXCEPTIONS = [
    "markdown", "code_block", "chart", "virtual_list",
    "icon", "stat", "color_picker", "data_table"
]

# Tags HTML diretas proibidas no UI
HTML_TAGS = r'<(div|span|button|input|textarea|select|form|ul|ol|li|table|tr|td|th|nav|header|footer|main|section|article|aside|h[1-6]|p)\b'


def check_ui_file(path: str, component_id: str) -> list:
    errors = []
    with open(path) as f:
        content = f.read()

    is_exception = any(e in component_id for e in DYNAMIC_RENDER_EXCEPTIONS)

    # CR-360: HTML direto proibido
    if not is_exception:
        tags = re.findall(HTML_TAGS, content)
        if tags:
            errors.append(
                f"[CR-360] {component_id}_ui.rs -- HTML direto proibido: <{set(tags).pop()}>\n"
                f"            usar Primitive correspondente"
            )

    # CR-361: signals proibidos
    for sig in ["create_signal", "signal(", "RwSignal::new", "create_rw_signal"]:
        if sig in content:
            errors.append(f"[CR-361] {component_id}_ui.rs -- signal proibido: {sig}\n            UI nao cria estado")
            break

    # CR-362: provide_context proibido
    if "provide_context" in content:
        errors.append(f"[CR-362] {component_id}_ui.rs -- provide_context proibido")

    # CR-363: use_navigate proibido
    if "use_navigate" in content:
        errors.append(f"[CR-363] {component_id}_ui.rs -- use_navigate proibido")

    # CR-364: deve importar Primitive
    has_primitive = "canonrs_core::primitives" in content or "canonrs_core" in content
    if not has_primitive and not is_exception:
        errors.append(f"[CR-364] {component_id}_ui.rs -- nao importa Primitive\n            UI DEVE ser proxy de canonrs_core::primitives")

    # node_ref: unwrap_or_default() permitido ao passar para Primitive
    # CR-365: data-rs-* direto no UI proibido (deve vir do Primitive)
    if not is_exception:
        content_no_attr = re.sub(r'attr:data-rs-\w+', '', content)
        direct_data_rs = re.findall(r'data-rs-\w+=(?!class)', content_no_attr)
        if len(direct_data_rs) > 3:
            errors.append(f"[CR-365] {component_id}_ui.rs -- data-rs-* direto no UI\n            atributos data-rs-* pertencem ao Primitive")

    return errors


def run(target: str = None) -> int:
    total_ok = 0
    total_errors = 0

    files = glob.glob(f"{UI_DIR}/**/*_ui.rs", recursive=True)

    for path in sorted(files):
        component_id = os.path.basename(path).replace("_ui.rs", "")
        if target and component_id != target:
            continue
        errors = check_ui_file(path, component_id)
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
    print(f"[OK] {total_ok} UI files clean")
    if total_errors:
        print(f"[FAIL] {total_errors} violations found")
        return 1
    print("[OK] All UI files canonical")
    return 0


if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else None
    sys.exit(run(target))
