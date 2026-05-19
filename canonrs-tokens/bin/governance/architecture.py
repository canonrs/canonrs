"""
architecture.py — Architecture governance layer
Validações: CR-350..357 (primitive), CR-360..365 (ui), CR-370..375 (boundary), CR-380..384 (preview)
"""

import re
import glob
import os


def check_primitive(component_id: str, primitives_dir: str) -> list:
    """CR-350 a CR-357"""
    errors = []
    prim_file = os.path.join(primitives_dir, f"{component_id}.rs")
    if not os.path.exists(prim_file):
        return errors
    with open(prim_file) as f:
        content = f.read()

    has_uid = 'data-rs-uid=' in content
    has_component = '#[component]' in content
    if has_component and not has_uid:
        errors.append(f"[CR-350] {component_id}.rs -- data-rs-uid ausente\n            usar crate::infra::uid::generate(\"prefix\")")
    if re.search(r'fn \w+_uid\(', content):
        errors.append(f"[CR-351] {component_id}.rs -- fn *_uid() propria proibida")
    if 'data-rs-open' in content:
        errors.append(f"[CR-352] {component_id}.rs -- data-rs-open proibido")
    if 'data-rs-visible' in content:
        errors.append(f"[CR-353] {component_id}.rs -- data-rs-visible proibido")
    if re.search(r'data-rs-selected\s*=(?!.*state)', content):
        errors.append(f"[CR-354] {component_id}.rs -- data-rs-selected fora de state")
    if 'data-rs-component' in content:
        errors.append(f"[CR-355] {component_id}.rs -- data-rs-component redundante")
    if 'data-rs-behavior' in content:
        errors.append(f"[CR-356] {component_id}.rs -- data-rs-behavior redundante\n            usar data-rs-interaction")
    for line in content.splitlines():
        if line.count('data-rs-state') > 1:
            errors.append(f"[CR-357] {component_id}.rs -- dois data-rs-state no mesmo elemento\n            {line.strip()[:80]}")
            break
    return errors


def check_ui(component_id: str, ui_dir: str) -> list:
    """CR-360 a CR-365"""
    errors = []
    matches = glob.glob(f"{ui_dir}/**/{component_id}_ui.rs", recursive=True)
    if not matches:
        return errors
    with open(matches[0]) as f:
        content = f.read()

    DYNAMIC_RENDER_EXCEPTIONS = ["markdown", "code_block", "chart", "virtual_list", "icon", "stat", "color_picker", "data_table"]
    is_exception = any(e in component_id for e in DYNAMIC_RENDER_EXCEPTIONS)

    if not is_exception:
        html_tags = re.findall(r'<(div|span|button|input|textarea|select|form|ul|ol|li|table|tr|td|th|nav|header|footer|main|section|article|aside|h[1-6]|p|a)', content)
        if html_tags:
            errors.append(f"[CR-360] {component_id}_ui.rs -- HTML direto proibido: <{set(html_tags).pop()}>")

    for sig in ["create_signal", "signal(", "RwSignal::new", "create_rw_signal"]:
        if sig in content:
            errors.append(f"[CR-361] {component_id}_ui.rs -- signal proibido: {sig}")
            break
    if "provide_context" in content:
        errors.append(f"[CR-362] {component_id}_ui.rs -- provide_context proibido")
    if "use_navigate" in content:
        errors.append(f"[CR-363] {component_id}_ui.rs -- use_navigate proibido")

    has_primitive = "canonrs_core::primitives" in content or "canonrs_core" in content
    if not has_primitive and not is_exception:
        errors.append(f"[CR-364] {component_id}_ui.rs -- nao importa Primitive")

    return errors


def check_boundary(component_id: str, boundary_type: str, ui_dir: str) -> list:
    """CR-370 a CR-375"""
    errors = []
    matches = glob.glob(f"{ui_dir}/**/{component_id}_boundary.rs", recursive=True)
    if not matches:
        return errors
    with open(matches[0]) as f:
        src = f.read()

    for sig in ["create_signal", "signal(", "RwSignal::new", "create_rw_signal", "create_memo"]:
        if sig in src:
            errors.append(f"[CR-370] {component_id}_boundary.rs -- signal proibido: {sig}")
            break
    if re.search(r'move\s*\|\s*\|', src):
        errors.append(f"[CR-371] {component_id}_boundary.rs -- closure reativa proibida")
    for bl in ["use_navigate", "fetch(", "reqwest::", "http::", "async fn", "spawn_local"]:
        if bl in src:
            errors.append(f"[CR-372] {component_id}_boundary.rs -- logica de negocio proibida: {bl}")
            break
    if boundary_type == "passthrough":
        if len(re.findall(r'=>', src)) > 4:
            errors.append(f"[CR-373] {component_id}_boundary.rs -- passthrough com branching excessivo")
    if "provide_context" in src:
        errors.append(f"[CR-374] {component_id}_boundary.rs -- provide_context proibido")

    BOUNDARY_HTML_EXCEPTIONS = ["alert_dialog", "drawer", "sheet", "status_dot", "switch", "toggle_group", "toolbar"]
    if any(e in component_id for e in BOUNDARY_HTML_EXCEPTIONS):
        return errors

    is_unified = 'unified' in src.lower() or 'composite' in src.lower() or component_id in ['sidebar', 'data_table', 'dropdown_menu']
    if not is_unified:
        view_blocks = re.findall(r'view!\s*\{(.+?)^\s*\}', src, re.DOTALL | re.MULTILINE)
        view_content = "\n".join(view_blocks) if view_blocks else ""
        html_tags = re.findall(r'<(div|span|button|input|textarea|select|form|ul|ol|li|table)\b', view_content)
        if html_tags:
            errors.append(f"[CR-375] {component_id}_boundary.rs -- HTML direto proibido: <{html_tags[0]}>")

    return errors


def check_preview(component_id: str, ui_dir: str) -> list:
    """CR-380 a CR-384"""
    errors = []
    matches = glob.glob(f"{ui_dir}/**/{component_id}/preview.rs", recursive=True)
    if not matches:
        return errors
    with open(matches[0]) as f:
        src = f.read()

    ui_imports = re.findall(rf'use.*{component_id}_ui::{{([^}}]+)}}', src)
    bad_ui = []
    for imp in ui_imports:
        items = [i.strip() for i in imp.split(",")]
        config_suffixes = ("Column", "Action", "Config", "Options", "Props", "Data", "Row", "Cell")
        bad_ui += [i for i in items if not i.endswith(config_suffixes)]
    if bad_ui or (re.search(rf'use.*{component_id}_ui', src) and not ui_imports):
        errors.append(f"[CR-380] {component_id}/preview.rs -- importa _ui.rs diretamente: {bad_ui}")

    prim_imports = re.findall(r'use canonrs_core::primitives::([A-Z]\w+)', src)
    layout_ok = {"Stack", "StackPrimitive", "Grid", "GridPrimitive", "Flex", "FlexPrimitive"}
    enum_suffixes = ("Variant", "Side", "Size", "Orientation", "Selection", "Type", "Direction", "Gap", "State", "Align", "Shape", "Density")
    bad_prims = [p for p in prim_imports if p not in layout_ok and not p.endswith(enum_suffixes)]
    if bad_prims:
        errors.append(f"[CR-381] {component_id}/preview.rs -- importa primitives diretamente: {bad_prims[:3]}")

    for sig in ["create_signal", "signal(", "RwSignal::new", "create_rw_signal"]:
        if sig in src:
            errors.append(f"[CR-382] {component_id}/preview.rs -- signal proibido: {sig}")
            break
    if "fetch(" in src or "reqwest::" in src or "async fn" in src:
        errors.append(f"[CR-383] {component_id}/preview.rs -- fetch/async proibido")
    if "provide_context" in src:
        errors.append(f"[CR-384] {component_id}/preview.rs -- provide_context proibido")

    return errors
