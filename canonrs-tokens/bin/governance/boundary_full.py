"""
boundary_full.py — Full boundary validation (ex-island)
Validações: CR-330..344 — estrutura, props, DOM mutation, state, UI import
"""

import re
import glob
import os


def check_boundary_full(boundary_file: str, ui_dir: str, component: str, boundary_type: str = "state") -> list:
    """Validacao completa de boundary (CR-330 a CR-344)"""
    errors = []
    matches = glob.glob(f"{ui_dir}/**/{boundary_file}", recursive=True)
    if not matches:
        errors.append(f"[BOUNDARY-MISSING] {boundary_file} nao encontrado")
        return errors
    with open(matches[0]) as f:
        content = f.read()
    lines = content.splitlines()

    # CR-334: use_context inside closure
    for i, line in enumerate(lines, 1):
        if line.strip().startswith("//"):
            continue
        if "use_context" in line:
            block_before = "\n".join(lines[max(0,i-10):i])
            if "move |" in block_before or "move||" in block_before:
                errors.append(
                    f"[CR-334] {boundary_file} linha {i} -- use_context dentro de closure proibido\n"
                    f"            {line.strip()[:80]}"
                )

    # CR-335b: style mutation
    for i, line in enumerate(lines, 1):
        if line.strip().startswith("//"):
            continue
        if "set_property" in line and "var(--" in line:
            continue
        if "style().set_property" in line or "style().set_" in line:
            errors.append(f"[CR-335] {boundary_file} linha {i} -- style DOM mutation proibido\n            {line.strip()[:80]}")

    # CR-337: inner_html dinamico
    for i, line in enumerate(lines, 1):
        if line.strip().startswith("//"):
            continue
        if "inner_html" in line:
            if "move ||" in line or ".get(" in line:
                errors.append(f"[CR-337] {boundary_file} linha {i} -- inner_html dinamico proibido\n            {line.strip()[:80]}")

    # CR-339: dynamic class
    for i, line in enumerate(lines, 1):
        if line.strip().startswith("//"):
            continue
        if re.search(r'class\s*=\s*(move\s*\|\||if\s|\w+\.get\()', line):
            errors.append(f"[CR-339] {boundary_file} linha {i} -- dynamic class state proibido\n            {line.strip()[:80]}")

    # CR-344: boundary DEVE importar UI
    has_ui_import = (
        bool(re.search(r'use (super|crate)::.*_ui::', content)) or
        bool(re.search(r'pub use crate::', content)) or
        bool(re.search(r'super::\w+_ui::', content)) or
        bool(re.search(r'use super::\w+_boundary::', content))
    )
    if not has_ui_import and component not in ['data_table']:
        errors.append(f"[CR-344] {boundary_file} -- boundary NAO importa UI\n            todo boundary DEVE importar seu UI")

    # CR-342: boundary usa primitives diretamente sem UI
    if component not in ['data_table']:
        primitive_pattern = re.findall(r'use canonrs_core::primitives::{([^}]+)}', content)
        if primitive_pattern:
            has_ui = (
                bool(re.search(r'use (super|crate)::.*_ui::', content)) or
                bool(re.search(r'use super::\w+_boundary::', content))
            )
            if not has_ui:
                prims = primitive_pattern[0][:60]
                errors.append(f'[CR-342] {boundary_file} -- boundary usa primitives diretamente sem UI\n            {prims}')

    return errors
