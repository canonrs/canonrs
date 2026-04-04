#!/usr/bin/env python3
"""
CanonRS Component Token Validator v2
#1 - token usado deve existir no sistema
#2 - ALWAYS_ALLOWED minimo (sem bypass de theme-*)
#3 - token declarado no contrato mas nao usado (unused)
#4 - state engine: behaviors devem usar padroes canonicos
"""

import json
import re
import sys
import os
import glob

JSON_PATH    = "../src/design/tokens/tokens_components.json"
CSS_DIR      = "../../canonrs-server/styles/ui"
TOKENS_DIR   = "../src/design/tokens"
BEHAVIORS_DIR= "../../canonrs-client/src/behaviors"

RUNTIME_ALLOWED = [
    "--theme-",
    "--primitive-",
    "--slider-fill",  # inline runtime property set by behavior
]

FOUNDATION_PREFIXES = {
    "spacing":     ["--space-"],
    "size":        ["--size-"],
    "radius":      ["--radius-"],
    "motion":      ["--motion-"],
    "typography":  ["--font-", "--line-height-"],
    "shadow":      ["--shadow-"],
    "border":      ["--border-"],
    "interaction": ["--state-", "--focus-ring-", "--opacity-", "--transform-", "--blur-"],
    "z":           ["--z-", "--layer-"],
    "color":       ["--color-"],
}

# State engine anti-patterns
STATE_ENGINE_VIOLATIONS = [
    'get_attribute("data-rs-disabled").as_deref() == Some("disabled")',
    'get_attribute("data-rs-state").as_deref() == Some("disabled")',
    'data-rs-open',
    'data-rs-active',
    'data-rs-visible',
    'data-rs-attached',
    'data-rs-copy-attached',
]

# Allowed patterns (canonical)
STATE_ENGINE_ALLOWED = [
    'has_attribute("data-rs-disabled")',
    'add_state',
    'remove_state',
    'remove_states',
]


def extract_declared_tokens(tokens_dir):
    declared = set()
    pattern = re.compile(r'FamilyToken::new\("([^"]+)"')
    for rs_file in glob.glob(f"{tokens_dir}/**/*.rs", recursive=True):
        with open(rs_file) as f:
            for match in pattern.finditer(f.read()):
                declared.add(f"--{match.group(1)}")
    return declared


def check_state_engine_violations(behaviors_dir):
    errors = []
    for rs_file in glob.glob(f"{behaviors_dir}/*.rs"):
        with open(rs_file) as f:
            content = f.read()
        filename = os.path.basename(rs_file)
        for violation in STATE_ENGINE_VIOLATIONS:
            if violation in content:
                # check if it is in a comment
                for line in content.splitlines():
                    if violation in line and not line.strip().startswith("//"):
                        errors.append(f"[STATE-ENGINE] {filename} -- viola padrao canonico: {violation[:50]}")
                        break
    return errors


def extract_vars(css):
    return re.findall(r"var\((--[a-zA-Z0-9-]+)", css)


def is_allowed(var, tokens, foundations, declared):
    for prefix in RUNTIME_ALLOWED:
        if var.startswith(prefix):
            return True, ""
    if var not in declared:
        return False, f"[INEXISTENTE] {var} -- token nao existe no sistema"
    for pattern in tokens:
        prefix = pattern.replace("*", "").replace(" ", "")
        if var.startswith(f"--{prefix}"):
            return True, ""
    for foundation in foundations:
        for prefix in FOUNDATION_PREFIXES.get(foundation, []):
            if var.startswith(prefix):
                return True, ""
    return False, f"[CONTRATO] {var} -- existe mas nao declarado em @tokens/@foundation"


def check_unused(tokens, vars_used, declared):
    warnings = []
    for pattern in tokens:
        if "*" not in pattern:
            continue
        prefix = f"--{pattern.replace(chr(42), "").replace(" ", "")}"
        matching_declared = [t for t in declared if t.startswith(prefix)]
        for t in matching_declared:
            if t not in vars_used:
                warnings.append(f"[UNUSED] {t} -- declarado, nao usado no CSS")
    return warnings



def check_states_in_css(states, css):
    errors = []
    for state in states:
        selector = f'[data-rs-state~="{state}"]'
        if selector not in css:
            errors.append(f"[STATE-CSS] estado '{state}' declarado mas sem seletor CSS correspondente")
    return errors


def check_states_in_behavior(states, behavior_file, behaviors_dir):
    if not behavior_file:
        return []
    full_path = os.path.join(behaviors_dir, behavior_file)
    if not os.path.exists(full_path):
        return [
            f"[BEHAVIOR-MISSING] {behavior_file} nao encontrado\n"
            f"            esperado em: {full_path}"
        ]
    errors = []
    with open(full_path) as f:
        content = f.read()
    has_add_state = 'add_state(' in content
    for state in states:
        if not has_add_state and f'"{state}"' not in content:
            errors.append(
                f"[STATE-BEHAVIOR] estado '{state}' ausente no behavior\n"
                f"            adicione: add_state(el, \"{state}\") em {behavior_file}"
            )
        elif f'"{state}"' not in content:
            errors.append(
                f"[STATE-BEHAVIOR] estado '{state}' nao encontrado em {behavior_file}\n"
                f"            adicione: add_state(el, \"{state}\")"
            )
    return errors


def check_registered(behavior_file, registered, auto_init_path):
    if not behavior_file or registered is None:
        return []
    if not registered:
        return []
    with open(auto_init_path) as f:
        content = f.read()
    module = behavior_file.replace(".rs", "")
    if f"{module}::register()" not in content:
        return [
            f"[NOT-REGISTERED] {behavior_file} nao registrado\n"
            f"            adicione em auto_init.rs: {module}::register();"
        ]
    return []


def validate(component, declared):
    css_file = os.path.join(CSS_DIR, component["file"])
    if not os.path.exists(css_file):
        return [f"[MISSING] {component['file']} nao encontrado\n            esperado em: {css_file}"], []
    with open(css_file) as f:
        css = f.read()
    vars_used   = extract_vars(css)
    tokens      = [t.strip() for t in component["tokens"].split(",")]
    foundations = [f.strip() for f in component["foundation"].split(",")]
    errors  = []
    seen    = set()
    used_set = set(vars_used)
    for var in vars_used:
        if var in seen:
            continue
        seen.add(var)
        ok, msg = is_allowed(var, tokens, foundations, declared)
        if not ok:
            errors.append(msg)

    # #4 states check
    states = component.get("states", [])
    behavior_file = component.get("behavior", None)
    registered = component.get("registered", None)

    errors += check_states_in_css(states, css)
    errors += check_states_in_behavior(states, behavior_file, BEHAVIORS_DIR)

    auto_init = os.path.join(BEHAVIORS_DIR, "auto_init.rs")
    errors += check_registered(behavior_file, registered, auto_init)

    unused = check_unused(tokens, used_set, declared)
    return errors, unused




ISLAND_DIR = "../../canonrs-server/src/ui"


def check_island_props_LEGACY(island_file, island_dir, component):
    """CR-330: island props must use serde enums, not Option<String> or Option<bool>"""
    errors = []
    # find island file
    import glob
    matches = glob.glob(f"{island_dir}/**/{island_file}", recursive=True)
    if not matches:
        errors.append(f"[ISLAND-MISSING] {island_file} nao encontrado")
        return errors
    with open(matches[0]) as f:
        content = f.read()
    lines = content.splitlines()
    in_island = False
    for line in lines:
        if "#[island]" in line:
            in_island = True
        if in_island and "#[prop(optional" in line:
            if "Option<String>" in line or "Option<bool>" in line:
                # allow cosmetic props
                cosmetic = any(p in line for p in ["class", "aria_label", "validation", "disabled", "text", "target", "href", "external", "placeholder", "selected_value", "value", "label", "name"])
                if not cosmetic:
                    prop = line.strip()
                    errors.append(
                        f"[CR-330] {island_file} -- prop nao serde-safe: {prop[:80]}\n"
                        f"            use enum com serde ao inves de Option<String>/Option<bool>"
                    )
        if in_island and ") -> impl IntoView" in line:
            in_island = False
    return errors


def check_island_ssr_state_LEGACY(island_file, island_dir):
    """CR-331: island SSR state must be fully materialized without signals"""
    errors = []
    import glob
    matches = glob.glob(f"{island_dir}/**/{island_file}", recursive=True)
    if not matches:
        return errors
    with open(matches[0]) as f:
        content = f.read()
    has_signals = "signal(" in content
    has_data_rs_state = "data-rs-state" in content
    has_initial_state = "initial_state" in content
    if has_signals and has_data_rs_state and not has_initial_state:
        errors.append(
            f"[CR-331] {island_file} -- usa signals em data-rs-state sem initial_state materializado\n"
            f"            adicione initial_state pre-computado como fallback SSR"
        )
    return errors


def check_island_css_child_combinator(css_file):
    """CR-333: CSS must not use > combinator across island boundaries"""
    errors = []
    if not os.path.exists(css_file):
        return errors
    with open(css_file) as f:
        content = f.read()
    lines = content.splitlines()
    for i, line in enumerate(lines, 1):
        if line.strip().startswith("//") or line.strip().startswith("/*"):
            continue
        if re.search(r"> *\[data-rs-", line):
            errors.append(
                f"[CR-333] linha {i} -- child combinator (>) proibido em seletor que cruza island boundary\n"
                f"            use descendant selector (espaco) ao inves de >\n"
                f"            {line.strip()[:80]}"
            )
    return errors


# ═══════════════════════════════════════════════════════════════
# ISLAND VALIDATOR — LEI IMUTÁVEL (CR-330 a CR-339)
# ═══════════════════════════════════════════════════════════════

ISLAND_FORBIDDEN_PROPS = [
    ("Option<String>", "into", "CR-330: Option<String> + into proibido — use String direto"),
    ("Option<bool>",   "into", "CR-330: Option<bool> + into proibido — use bool direto"),
]

ISLAND_FORBIDDEN_PATTERNS = [
    ("set_attribute",          "CR-335: DOM mutation proibido em island — use signals"),
    ("get_attribute",          "CR-333: DOM read proibido em island — use signals"),
    ("spawn_local",            "CR-336: async state proibido em island — use signals sincrono"),
    ("inner_html",             "CR-337: inner_html proibido em island — estrutura MUST ser estavel"),
]

ISLAND_REQUIRED_PATTERNS = [
    ("initial_state",          "CR-331: initial_state obrigatorio em todo island com signals em data-rs-state"),
    ("data-rs-state",          "CR-339: data-rs-state obrigatorio — state MUST ser emitido via data-rs-state"),
]

ISLAND_SSOT_PATTERNS = [
    ("set_selected.set",       "set_"),
    ("set_open.set",           "set_"),
    ("set_hover.set",          "set_"),
    ("set_focused.set",        "set_"),
]

CFG_REQUIRED = [
    "web_sys",
    "wasm_bindgen",
    "Closure",
]


def check_island_full(island_file, island_dir, component):
    """LEI IMUTAVEL — validacao completa de island"""
    errors = []
    import glob
    matches = glob.glob(f"{island_dir}/**/{island_file}", recursive=True)
    if not matches:
        errors.append(f"[ISLAND-MISSING] {island_file} nao encontrado")
        return errors
    with open(matches[0]) as f:
        content = f.read()
    lines = content.splitlines()

    # ── CR-330: props proibidas ──────────────────────────────────
    in_island = False
    for line in lines:
        if "#[island]" in line:
            in_island = True
        if in_island and ") -> impl IntoView" in line:
            in_island = False
        if in_island and "#[prop(" in line:
            for (type_, attr, msg) in ISLAND_FORBIDDEN_PROPS:
                if type_ in line and attr in line:
                    # allow cosmetic props
                    cosmetic = any(p in line for p in ["class", "aria_label", "validation", "disabled", "text", "target", "href", "external", "placeholder", "selected_value", "value", "label", "name"])
                    if not cosmetic:
                        errors.append(f"[CR-330] {island_file} linha {i} -- {msg}\n            {line.strip()[:80]}")

    # ── CR-331: initial_state obrigatorio ────────────────────────
    has_signals = "signal(" in content
    has_data_rs_state = "data-rs-state" in content
    has_initial_state = "initial_state" in content
    if has_signals and has_data_rs_state and not has_initial_state:
        errors.append(
            f"[CR-331] {island_file} -- initial_state AUSENTE (lei imutavel)\n"
            f"            todo island com signals em data-rs-state DEVE ter initial_state materializado"
        )

    # ── CR-333: DOM → SIGNAL proibido (data-flow real) ────────────
    import re as _re
    for i, line in enumerate(lines, 1):
        stripped = line.strip()
        if stripped.startswith("//"):
            continue

        if "query_selector" in line:
            var = None
            for j in range(i, max(i-25, -1), -1):
                m = _re.search(r'let\s+(\w+)\s*=', lines[j])
                if m:
                    var = m.group(1)
                    break
            if var:
                block = "\n".join(lines[i:i+25])
                if _re.search(rf'\.set\(\s*{var}\s*[,)]', block):
                    errors.append(
                        f"[CR-333] {island_file} linha {i} -- DOM → SIGNAL proibido\n"
                        f"            query_selector → {var} → .set() viola SSOT\n"
                        f"            {stripped[:80]}"
                    )

    # ── CR-335: outros padroes DOM proibidos ─────────────────────
    for (pattern, msg) in ISLAND_FORBIDDEN_PATTERNS:
        for i, line in enumerate(lines, 1):
            stripped = line.strip()
            if stripped.startswith("//"):
                continue
            if pattern in line:
                errors.append(f"[{msg.split(':')[0]}] {island_file} linha {i} -- {msg.split(':', 1)[1].strip()}\n            {stripped[:80]}")
                break

    # ── CR-336: spawn_local proibido ─────────────────────────────
    # já coberto por ISLAND_FORBIDDEN_PATTERNS

    # ── CR-339: class-based state proibido (dynamic class) ──────
    import re as _re
    for i, line in enumerate(lines, 1):
        stripped = line.strip()
        if stripped.startswith("//"):
            continue
        if _re.search(r'class\s*=\s*(move\s*\|\||if\s|\w+\.get\()', line):
            errors.append(
                f"[CR-339] {island_file} linha {i} -- dynamic class state proibido\n"
                f"            state MUST ser emitido via data-rs-state, nao via class\n"
                f"            {stripped[:80]}"
            )

    # ── CR-338: web_sys DEVE ser cfg-gated ──────────────────────
    # verifica se uso de web_sys/wasm_bindgen/Closure esta dentro
    # de bloco protegido por #[cfg(feature = "hydrate")]
    # estrategia: encontrar ultimo cfg antes do padrao
    for pattern in CFG_REQUIRED:
        for i, line in enumerate(lines, 1):
            stripped = line.strip()
            if stripped.startswith("//"):
                continue
            if pattern in line:
                # buscar cfg guard nas linhas anteriores (ate o inicio)
                context = "\n".join(lines[0:i])
                # contar cfgs e end de blocos para determinar se ainda estamos dentro
                cfg_count = context.count('#[cfg(feature = "hydrate")]')
                not_hydrate_count = context.count('#[cfg(not(feature = "hydrate"))]')
                if cfg_count == 0:
                    errors.append(
                        f"[CR-338] {island_file} linha {i} -- {pattern} sem cfg hydrate\n"
                        f"            web_sys MUST ser cfg-gated"
                    )
                break

    return errors

def main():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    json_path  = os.path.join(script_dir, JSON_PATH)
    global CSS_DIR, TOKENS_DIR, BEHAVIORS_DIR, ISLAND_DIR
    CSS_DIR      = os.path.join(script_dir, CSS_DIR)
    TOKENS_DIR   = os.path.join(script_dir, TOKENS_DIR)
    BEHAVIORS_DIR= os.path.join(script_dir, BEHAVIORS_DIR)
    ISLAND_DIR   = os.path.join(script_dir, ISLAND_DIR)

    declared = extract_declared_tokens(TOKENS_DIR)

    with open(json_path) as f:
        components = json.load(f)

    target      = sys.argv[1] if len(sys.argv) > 1 else None
    show_unused = "--unused" in sys.argv

    # #4 state engine check (global, nao por componente)
    if not target:
        se_errors = check_state_engine_violations(BEHAVIORS_DIR)
        if se_errors:
            print("\n[STATE-ENGINE VIOLATIONS]")
            for e in se_errors:
                print(f"   {e}")

    total_errors = 0
    total_ok     = 0
    total_unused = 0

    for comp in components:
        if target and comp["component"] != target:
            continue
        errors, unused = validate(comp, declared)
        island_file = comp.get("island", None)
        if island_file:
            island_dir = os.path.join(script_dir, ISLAND_DIR)
            errors += check_island_full(island_file, island_dir, comp["component"])
            css_file = os.path.join(CSS_DIR, comp["file"])
            errors += check_island_css_child_combinator(css_file)
        if errors:
            print(f"\n[ERRO] {comp['component'].upper()}")
            for e in errors:
                print(f"   {e}")
            total_errors += len(errors)
        else:
            total_ok += 1
            if target:
                print(f"\n[OK] {comp['component'].upper()} -- clean")
        if show_unused and unused:
            print(f"\n[UNUSED] {comp['component'].upper()}")
            for u in unused:
                print(f"   {u}")
            total_unused += len(unused)

    print(f"\n" + "="*50)
    print(f"[OK] {total_ok} components clean")
    if total_errors:
        print(f"[FAIL] {total_errors} violations found")
        sys.exit(1)
    if total_unused:
        print(f"[INFO] {total_unused} unused tokens")
    if not total_errors:
        print("[OK] All components canonical")


if __name__ == "__main__":
    main()
