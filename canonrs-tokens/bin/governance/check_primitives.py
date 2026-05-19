#!/usr/bin/env python3
"""
check_primitives.py — Primitive layer governance
Critérios: CRITÉRIOS DE AVALIAÇÃO — CANONRS — PRIMITIVE (2 pts)

Regras:
- Apenas contrato HTML semântico + data-rs-*
- data-rs-uid obrigatório no root via generate()
- data-rs-interaction obrigatório
- Sem lógica condicional inline no view!
- Sem side-effects, sem DOM mutation, sem layout
- node_ref: Option<NodeRef<T>> com unwrap_or_default() ANTES do view! — permitido
- Estado semântico via data-rs-state
"""

import re
import glob
import os
import sys


PRIMITIVES_DIR = os.path.join(
    os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
    "../../canonrs-core/src/primitives"
)

FORBIDDEN_ATTRS = [
    ("data-rs-open",       "CR-352: usar data-rs-state~=\"open\""),
    ("data-rs-visible",    "CR-353: usar data-rs-state~=\"visible\""),
    ("data-rs-component",  "CR-355: redundante — remover"),
    ("data-rs-behavior",   "CR-356: usar data-rs-interaction"),
]

LAYOUT_PASSIVES = ["stack", "flex", "grid", "container", "center", "spacer"]


def check_primitive_file(path: str, component_id: str) -> list:
    errors = []
    with open(path) as f:
        content = f.read()

    has_component = '#[component]' in content

    # CR-350: data-rs-uid obrigatório no root via generate()
    has_uid = 'data-rs-uid=' in content
    is_layout_passive = any(p in component_id for p in LAYOUT_PASSIVES)
    if has_component and not has_uid and not is_layout_passive:
        errors.append(f"[CR-350] {component_id}.rs -- data-rs-uid ausente\n            usar crate::infra::uid::generate(\"prefix\")")

    # CR-351: fn *_uid() própria proibida
    if re.search(r'fn \w+_uid\(', content):
        errors.append(f"[CR-351] {component_id}.rs -- fn *_uid() própria proibida\n            usar crate::infra::uid::generate()")

    # CR-352..356: atributos proibidos
    for (attr, msg) in FORBIDDEN_ATTRS:
        if attr in content:
            errors.append(f"[{msg.split(':')[0]}] {component_id}.rs -- {attr} proibido\n            {msg.split(':',1)[1].strip()}")

    # CR-354: data-rs-selected fora de state
    if re.search(r'data-rs-selected\s*=(?!.*state)', content):
        errors.append(f"[CR-354] {component_id}.rs -- data-rs-selected fora de state\n            usar data-rs-state~=\"selected\"")

    # CR-357: dois data-rs-state no mesmo elemento
    for i, line in enumerate(content.splitlines(), 1):
        if line.count('data-rs-state') > 1:
            errors.append(f"[CR-357] {component_id}.rs linha {i} -- dois data-rs-state no mesmo elemento\n            {line.strip()[:80]}")
            break

    # node_ref: unwrap_or_default() DEVE estar antes do view! (não dentro)
    if 'node_ref' in content and 'unwrap_or_default()' in content:
        # verificar se está dentro do view! (proibido) ou antes (permitido)
        view_start = content.find('view!')
        unwrap_pos = content.find('unwrap_or_default()')
        if view_start > 0 and unwrap_pos > view_start:
            errors.append(f"[CR-358] {component_id}.rs -- unwrap_or_default() dentro do view!\n            calcular node_ref antes do view!")

    # Branching que produz elementos HTML diferentes — verificar match dentro do view!
    if has_component:
        view_blocks = re.findall(r'view!\s*\{(.+?)^\s*\}', content, re.DOTALL | re.MULTILINE)
        for block in view_blocks:
            if re.search(r'\bif\b.+<[a-z]', block) or re.search(r'\bmatch\b.+<[a-z]', block):
                errors.append(f"[CR-359] {component_id}.rs -- branching com elementos HTML diferentes no view!\n            separar em dois primitives")
                break

    return errors


def run(target: str = None) -> int:
    total_ok = 0
    total_errors = 0

    files = glob.glob(f"{PRIMITIVES_DIR}/*.rs")
    files = [f for f in files if not os.path.basename(f).startswith("mod")]

    for path in sorted(files):
        component_id = os.path.basename(path).replace(".rs", "")
        if target and component_id != target:
            continue
        errors = check_primitive_file(path, component_id)
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
    print(f"[OK] {total_ok} primitives clean")
    if total_errors:
        print(f"[FAIL] {total_errors} violations found")
        return 1
    print("[OK] All primitives canonical")
    return 0


if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else None
    sys.exit(run(target))
