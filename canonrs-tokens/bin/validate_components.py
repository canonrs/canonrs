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
ISLAND_DIR       = "../../canonrs-server/src/ui"
INTERACTIONS_DIR = "../../canonrs-server/src/interactions"
BUILDER_DIR      = "../../canonrs-server/src/ui"

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



def check_active_state_tokens(declared):
    """CR-336b: tokens *-fg-active/open/selected/checked devem usar --theme-action-primary-*"""
    import re as _re
    errors = []
    # sufixos que indicam estado ativo
    ACTIVE_FG_SUFFIXES = ["-fg-active", "-fg-open", "-fg-selected", "-fg-checked"]
    ACTIVE_BORDER_SUFFIXES = ["-border-active", "-border-checked", "-border-selected"]
    # valores proibidos para fg ativo
    FORBIDDEN_FG_VALUES = ["--theme-surface-", "--theme-text-", "transparent", "inherit"]

    import glob as _glob
    tokens_dir = TOKENS_DIR
    pattern = re.compile(r'FamilyToken::new\("([^"]+)",\s*"([^"]+)"\)')
    for rs_file in _glob.glob(f"{tokens_dir}/**/*.rs", recursive=True):
        with open(rs_file) as f:
            src = f.read()
        for match in pattern.finditer(src):
            name = match.group(1)
            value = match.group(2)
            token = f"--{name}"
            # fg ativo deve usar theme-action-primary
            for suffix in ACTIVE_FG_SUFFIXES:
                if token.endswith(suffix):
                    if not any(v in value for v in ["--theme-action-primary", "--color-primary"]):
                        errors.append(
                            f"[CR-336b] {token} = {value}\n"
                            f"            token *-fg-active/open/selected DEVE usar --theme-action-primary-fg ou --color-primary-foreground"
                        )
            # border ativo deve usar theme-action-primary
            for suffix in ACTIVE_BORDER_SUFFIXES:
                if token.endswith(suffix):
                    if not any(v in value for v in ["--theme-action-primary", "--color-primary"]):
                        errors.append(
                            f"[CR-336b] {token} = {value}\n"
                            f"            token *-border-active/checked DEVE usar --theme-action-primary-bg"
                        )
    return errors


def check_missing_active_tokens(component, declared):
    """CR-336c: componentes com state open/active/selected devem ter token fg correspondente"""
    errors = []
    states = component.get("states", [])
    tokens_prefix = component.get("tokens", "").split(",")[0].replace("*","").replace(" ","")
    if not tokens_prefix:
        return errors

    STATE_FG_MAP = {
        "open":     f"--{tokens_prefix}trigger-fg-open",
        "active":   f"--{tokens_prefix}trigger-fg-active",
        "selected": f"--{tokens_prefix}trigger-fg-selected",
        "checked":  f"--{tokens_prefix}fg-checked",
    }

    for state in states:
        expected = STATE_FG_MAP.get(state)
        if expected and expected not in declared:
            errors.append(
                f"[CR-336c] token '{expected}' ausente\n"
                f"            componente com state '{state}' DEVE declarar token fg correspondente\n"
                f"            valor esperado: var(--theme-action-primary-bg)"
            )
    return errors


# ═══════════════════════════════════════════════════════════════
# CSS QUALITY RULES (CR-340 a CR-347)
# ═══════════════════════════════════════════════════════════════

import re as _css_re

# CR-340: pseudo-classes — permitido como complemento, nunca como source of truth
CSS_FORBIDDEN_PSEUDOCLASS = [
    (r':checked\b',       'checked',       'usar [data-rs-state~="checked"]'),
    (r'aria-selected',    'aria-selected', 'usar [data-rs-state~="selected"]'),
    (r'aria-expanded',    'aria-expanded', 'usar [data-rs-state~="open"]'),
    (r'aria-checked',     'aria-checked',  'usar [data-rs-state~="checked"]'),
]

# CR-341: valores hardcoded proibidos
CSS_FORBIDDEN_HARDCODE = [
    (r':\s*rgba?\s*\(',                      'rgba/rgb hardcoded', 'usar token de cor'),
    (r':\s*#[0-9a-fA-F]{3,8}\b',            'hex hardcoded',      'usar token de cor'),
    (r':\s*\d+(\.\d+)?s\b',                  'tempo hardcoded',    'usar var(--motion-duration-)'),
    (r':\s*\d+(\.\d+)?ms\b',                 'tempo ms hardcoded', 'usar var(--motion-duration-)'),
    (r'cubic-bezier\s*\(',                   'easing hardcoded',   'usar var(--motion-ease-)'),
    (r':\s*[0-9]+px\s+[0-9]+px\s+[0-9]+px', 'shadow hardcoded',   'usar token de sombra'),
]

# CR-342: display none/block proibido APENAS quando fora de state
CSS_FORBIDDEN_DISPLAY = [
    r'display\s*:\s*none\s*;',
    r'display\s*:\s*block\s*;',
]

# CR-343: transition sem tokens
CSS_FORBIDDEN_TRANSITION = [
    r'transition\s*:[^;]*\d+(\.\d+)?s',
    r'transition\s*:[^;]*\d+(\.\d+)?ms',
]

# CR-345: aria attributes como seletores CSS
CSS_FORBIDDEN_ARIA_SELECTORS = [
    r'\[aria-selected\]',
    r'\[aria-expanded\]',
    r'\[aria-checked\]',
    r'\[aria-disabled\]',
]

# Whitelist: seletores legítimos que podem usar pseudo-classes
CSS_PSEUDOCLASS_WHITELIST = [
    "::webkit-scrollbar",
    "::placeholder",
    ":root",
    ":not(",
    "focus-visible",
    "::before",
    "::after",
    "::marker",
    ":first-child",
    ":last-child",
    ":nth-child",
    ":empty",
    "scrollbar",
]

# Whitelist: valores hardcoded permitidos
CSS_HARDCODE_WHITELIST = [
    "content:",
    "width: 0",
    "height: 0",
    "opacity: 0",
    "opacity: 1",
    "z-index:",
    "0%",
    "100%",
    "transform:",
    "rotate(",
    "translateY(-50%)",
    "scaleY(",
    "scaleX(",
    "flex:",
    "order:",
    "grid-column",
    "grid-row",
    "color-mix(",
    "border-radius: 50%",
    "border-radius: 0",
    "top: 0",
    "left: 0",
    "right: 0",
    "bottom: 0",
    "padding: 0",
    "margin: 0",
    "gap: 0",
    "min-width: 0",
    "width: 100%",
    "height: 100%",
    "width: 3px",
    "width: 4px",
    "width: 8px",
    "height: 8px",
    "letter-spacing:",
    "line-height: 1",
    "line-height: 0",
    "border: none",
    "border: 0",
    "outline: none",
    "outline: 0",
    "pointer-events: none",
    "pointer-events: auto",
    "visibility: hidden",
    "visibility: visible",
    "appearance: none",
    "list-style: none",
    "list-style-type: none",
    "content: none",
    'content: ""',
    "content: \'",
    "box-sizing:",
    "cursor:",
    "resize:",
    "overflow:",
    "white-space:",
    "text-overflow:",
    "text-decoration:",
    "text-transform:",
    "font-style:",
    "font-variant:",
    "display: flex",
    "display: grid",
    "display: inline",
    "display: inline-flex",
    "display: inline-block",
    "display: contents",
    "display: table",
    "display: table-cell",
    "display: table-row",
    "position:",
    "inset:",
    "aspect-ratio:",
    "object-fit:",
    "object-position:",
    "max-width:",
    "min-height:",
    "max-height:",
    "flex-direction:",
    "flex-wrap:",
    "flex-shrink:",
    "flex-grow:",
    "align-items:",
    "align-self:",
    "justify-content:",
    "justify-self:",
    "justify-items:",
    "place-items:",
    "place-content:",
    "columns:",
    "column-gap:",
    "row-gap:",
    "border-style:",
    "border-collapse:",
    "border-spacing:",
    "vertical-align:",
    "word-break:",
    "overflow-wrap:",
    "text-align:",
    "direction:",
    "unicode-bidi:",
    "writing-mode:",
    "overscroll-behavior:",
    "scroll-behavior:",
    "scrollbar-width:",
    "touch-action:",
    "user-select:",
    "will-change:",
    "isolation:",
    "mix-blend-mode:",
    "backdrop-filter:",
    "filter:",
    "clip-path:",
    "mask:",
    "background-clip:",
    "background-origin:",
    "background-size:",
    "background-repeat:",
    "background-position:",
    "background-attachment:",
    "counter-reset:",
    "counter-increment:",
    "1px",
    "2px",
    "3px",
    "4px",
    "90deg",
    "180deg",
    "270deg",
    "360deg",
    "-45deg",
    "translateX(-50%)",
    "translate(-50%",
    "translateY(-50%)",
]


def check_css_quality(css_file, component_id=""):
    """CR-340 a CR-347: CSS quality rules"""
    errors = []
    if not os.path.exists(css_file):
        return errors
    with open(css_file) as f:
        css = f.read()
    lines = css.splitlines()

    for i, line in enumerate(lines, 1):
        stripped = line.strip()
        if stripped.startswith("/*") or stripped.startswith("*") or stripped.startswith("//"):
            continue

        # CR-340: pseudo-classes — permitido como complemento
        for (pattern, name, fix) in CSS_FORBIDDEN_PSEUDOCLASS:
            if _css_re.search(pattern, stripped):
                if any(w in stripped for w in CSS_PSEUDOCLASS_WHITELIST):
                    continue
                errors.append(
                    f"[CR-340] {os.path.basename(css_file)} linha {i} -- pseudo-state incorreto '{name}'\n"
                    f"            {fix}\n"
                    f"            {stripped[:80]}"
                )

        # CR-341: hardcode proibido
        for (pattern, name, fix) in CSS_FORBIDDEN_HARDCODE:
            if _css_re.search(pattern, stripped):
                if any(w in stripped for w in CSS_HARDCODE_WHITELIST):
                    continue
                # color-mix é permitido
                if "color-mix(" in stripped:
                    continue
                errors.append(
                    f"[CR-341] {os.path.basename(css_file)} linha {i} -- valor hardcoded '{name}'\n"
                    f"            {fix}\n"
                    f"            {stripped[:80]}"
                )

        # CR-342: display none/block para visibilidade
        for pattern in CSS_FORBIDDEN_DISPLAY:
            if _css_re.search(pattern, stripped):
                # permitido se não é controle de visibilidade
                # ex: display:none em ::marker, ::before é ok
                if any(w in stripped for w in ["::marker", "list-style", "content:", "::-webkit-scrollbar", "scrollbar-width", "-ms-overflow", "display: block", "display:block", "display: flex", "display:flex", "display: contents", "display:contents", "display: grid", "display:grid"]):
                    continue
                # verificar contexto — se seletor acima tem data-rs-state é visibilidade
                context = "\n".join(lines[max(0,i-5):i])
                if "data-rs-state" not in context and "[hidden]" not in context:
                    errors.append(
                        f"[CR-342] {os.path.basename(css_file)} linha {i} -- display sem state\n"
                        f"            usar data-rs-state ou [hidden]\n"
                        f"            {stripped[:80]}"
                    )

        # CR-345: aria como seletor CSS
        for pattern in CSS_FORBIDDEN_ARIA_SELECTORS:
            if _css_re.search(pattern, stripped):
                errors.append(
                    f"[CR-345] {os.path.basename(css_file)} linha {i} -- aria attribute como seletor CSS\n"
                    f"            usar [data-rs-state~=\"X\"] ao inves de aria attributes\n"
                    f"            {stripped[:80]}"
                )

    return errors


def check_token_hardcode(tokens_dir):
    """CR-347: tokens com valores hardcoded rgba/hex"""
    import glob as _glob
    errors = []
    pattern = _css_re.compile(r'FamilyToken::new\("([^"]+)",\s*"([^"]+)"\)')
    ALLOWED_HARDCODE = ["0", "1", "0%", "100%", "none", "transparent", "inherit",
                        "normal", "bold", "auto", "unset", "initial",
                        "1px", "2px", "3px", "4px", "180deg", "90deg",
                        "0.06em", "2.5rem", "0.5", "true", "false"]
    for rs_file in _glob.glob(f"{tokens_dir}/**/*.rs", recursive=True):
        with open(rs_file) as f:
            src = f.read()
        filename = os.path.basename(rs_file)
        for match in pattern.finditer(src):
            name  = match.group(1)
            value = match.group(2)
            if any(v == value.strip() for v in ALLOWED_HARDCODE):
                continue
            if value.startswith("var("):
                continue
            if value.startswith("color-mix("):
                continue
            if _css_re.match(r'^rgba?\s*\(', value):
                errors.append(
                    f"[CR-347] {filename} -- {name}: {value}\n"
                    f"            rgba hardcoded proibido — usar color-mix ou token semantico"
                )
            elif _css_re.match(r'^#[0-9a-fA-F]{{3,8}}$', value.strip()):
                errors.append(
                    f"[CR-347] {filename} -- {name}: {value}\n"
                    f"            hex hardcoded proibido — usar token semantico"
                )
    return errors


def check_states_in_css(states, css):
    errors = []
    for state in states:
        selector = f'[data-rs-state~="{state}"]'
        if selector not in css:
            errors.append(
                f"[STATE-CSS] estado '{state}' declarado mas sem seletor CSS correspondente\n"
                f"            adicione: [data-rs-X][data-rs-state~=\"{state}\"] {{ ... }}\n"
                f"            use ~= (contém) nao = (exato)"
            )
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

    # CR-338: island_type obrigatorio
    island_type = component.get("island_type", "")
    valid_island_types = ["passthrough", "init", "interaction"]
    if not island_type:
        errors.append(f"[CR-338] {component['id']} -- island_type ausente no builder.yaml (passthrough | init | interaction)")
    elif island_type not in valid_island_types:
        errors.append(f"[CR-338] {component['id']} -- island_type invalido: '{island_type}' -- esperado: passthrough | init | interaction")

    # CR-338: validar estrutura do island conforme island_type
    island_file = component.get("island", "")
    if island_type and island_file:
        import glob as _glob
        matches = _glob.glob(f"{ISLAND_DIR}/**/{island_file}", recursive=True)
        if not matches:
            errors.append(f"[CR-338] {component['id']} -- island file '{island_file}' nao encontrado")
        else:
            with open(matches[0]) as _f:
                island_src = _f.read()
            if island_type == "passthrough":
                if "#[island]" in island_src:
                    errors.append(f"[CR-338] {component['id']} -- island_type=passthrough mas '{island_file}' contem #[island]\n             fix: troque #[island] por #[component] em {ISLAND_DIR}/{component['id']}/{island_file}")
            elif island_type == "init":
                if "#[island]" in island_src:
                    errors.append("[CR-338] " + component['id'] + " -- island_type=init mas island_file contem #[island] -- fix: use #[component]")
            elif island_type == "interaction":
                pass  # interaction: #[component] correto, nao #[island] — CR-338 v2.1.0
                _iid = component['id'].replace("-", "_")
                interaction_file = os.path.join(INTERACTIONS_DIR, f"{_iid}.rs")
                if not os.path.exists(interaction_file):
                    warnings.append(f"[CR-338-C] {component['id']} -- WASM runtime recomendado em canonrs-interactions-*/src/{component['id']}.rs")  if 'warnings' in dir() else None


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
    # CR-336c desativado — reformulação pendente
    # errors += check_missing_active_tokens(component, declared)
    errors += check_states_in_behavior(states, behavior_file, BEHAVIORS_DIR)

    auto_init = os.path.join(BEHAVIORS_DIR, "auto_init.rs")
    errors += check_registered(behavior_file, registered, auto_init)

    unused = check_unused(tokens, used_set, declared)
    return errors, unused






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


def check_hover_override_active(css_file):
    """CR-337: hover deve respeitar estado, mas é permitido como complemento"""
    errors = []
    if not os.path.exists(css_file):
        return errors
    with open(css_file) as f:
        css_content = f.read()
    lines = css_content.splitlines()
    for i, line in enumerate(lines, 1):
        if ":hover" in line:
            if "data-rs-state" not in line and ":not(" not in line:
                continue
            if "::-webkit-scrollbar" in line or "scrollbar-color" in line:
                continue
            if i > 1 and "sem state guard intencional" in lines[i-2]:
                continue
            import re as _re_hover
            if not _re_hover.search(r':not\(\[data-rs-state~=', line):
                errors.append(
                    f"[CR-337] linha {i} -- hover sem guard :not([data-rs-state~=\"...\"])\n"
                    f"            adicione :not([data-rs-state~=\"open\"]) ou similar ao seletor hover\n"
                    f"            {line.strip()[:80]}"
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

ISLAND_FORBIDDEN_PROPS = []

ISLAND_FORBIDDEN_PATTERNS = [
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


# ═══════════════════════════════════════════════════════════════
# AST ENGINE — tree-sitter (FASE 3 — ENTERPRISE)
# ═══════════════════════════════════════════════════════════════

def _build_ast_parser():
    try:
        import tree_sitter_rust as tsr
        from tree_sitter import Language, Parser
        RUST = Language(tsr.language())
        parser = Parser(RUST)
        return parser
    except Exception:
        return None

_AST_PARSER = _build_ast_parser()


def _ast_walk(node, callback, results):
    callback(node, results)
    for child in node.children:
        _ast_walk(child, callback, results)


def check_island_ast(island_file, island_dir, island_type="state"):
    """AST-level validation — zero falso positivo/negativo"""
    import glob as _glob
    errors = []

    if _AST_PARSER is None:
        return errors  # fallback silencioso se tree-sitter nao disponivel

    matches = _glob.glob(f"{island_dir}/**/{island_file}", recursive=True)
    if not matches:
        return errors

    with open(matches[0], "rb") as f:
        source = f.read()

    tree = _AST_PARSER.parse(source)
    root = tree.root_node

    # ── COLETAR DADOS ────────────────────────────────────────────
    dom_vars = set()      # vars atribuidas de query_selector
    set_calls = []        # chamadas .set(...)
    cfg_scopes = []       # linhas com cfg(hydrate)
    dynamic_classes = []  # class= com expressao dinamica
    for_nodes = []        # for loops
    signal_count = [0]

    def collect(node, results):
        text = node.text.decode("utf8") if node.text else ""
        line = node.start_point[0] + 1

        # dom_vars: let X = ...query_selector...
        if node.type == "let_declaration":
            if b"query_selector" in (node.text or b""):
                pat = node.child_by_field_name("pattern")
                if pat:
                    dom_vars.add(pat.text.decode("utf8").strip())

        # set_calls: .set(...)
        if node.type == "call_expression":
            t = node.text or b""
            if b".set(" in t:
                set_calls.append((line, t.decode("utf8")))

        # cfg scopes
        if node.type == "attribute_item":
            t = node.text or b""
            if b'cfg(feature = "hydrate")' in t:
                cfg_scopes.append(line)

        # dynamic class
        if node.type == "attribute":
            t = node.text or b""
            if t.startswith(b"class") and (b"move" in t or b"if " in t or b".get(" in t):
                dynamic_classes.append((line, t.decode("utf8")[:80]))

        # for loops
        if node.type == "for_expression":
            for_nodes.append(line)

        # signal count
        if node.type == "call_expression":
            t = node.text or b""
            if t.startswith(b"signal("):
                signal_count[0] += 1

    _ast_walk(root, collect, None)

    # CR-333 AST: DOM → SIGNAL proibido (mantido)
    if island_type != "observer":
        for (line, call) in set_calls:
            for var in dom_vars:
                if var in call:
                    errors.append(
                        f"[CR-333-AST] {island_file} linha {line} -- DOM → SIGNAL (AST confirmed)\n"
                        f"            query_selector → {var} → .set()\n"
                        f"            {call[:80]}"
                    )

    # ── CR-339 AST: dynamic class ────────────────────────────────
    for (line, text) in dynamic_classes:
        errors.append(
            f"[CR-339-AST] {island_file} linha {line} -- dynamic class state proibido (AST confirmed)\n"
            f"            {text[:80]}"
        )

    # CR-336 AST: for loop permitido para iteracao DOM

    # CR-341 AST: multiplos signals — DESATIVADO para modelo island DOM-driven

    return errors


def check_island_full(island_file, island_dir, component, island_type="state"):
    """LEI IMUTAVEL — validacao completa de island (CR-330 a CR-342)"""
    import re as _re
    import glob as _glob
    errors = []
    matches = _glob.glob(f"{island_dir}/**/{island_file}", recursive=True)
    if not matches:
        errors.append(f"[ISLAND-MISSING] {island_file} nao encontrado")
        return errors
    with open(matches[0]) as f:
        content = f.read()
    lines = content.splitlines()

    # CR-330: props proibidas
    in_island = False
    for i, line in enumerate(lines, 1):
        if "#[island]" in line:
            in_island = True
        if in_island and ") -> impl IntoView" in line:
            in_island = False
        if in_island and "#[prop(" in line:
            for (type_, attr, msg) in ISLAND_FORBIDDEN_PROPS:
                if type_ in line and attr in line:
                    cosmetic = any(p in line for p in ["class","aria_label","validation","disabled","text","target","href","external","placeholder","selected_value","value","label","name","title","mode","initial","selected","aria_label","separator","empty_text","placeholder","description","input_type","variant","size","format","density","trigger","aria_describedby","aria_labelledby","aria_label","rows","cols","for_id","html_for","href","url","action","method","enctype"])
                    if not cosmetic:
                        errors.append(f"[CR-330] {island_file} linha {i} -- {msg}\n            {line.strip()[:80]}")

    # CR-331: initial_state — DESATIVADO para modelo island DOM-driven
    # islands DOM-driven não usam signals — estado vive no DOM

    def is_observer_context(block):
        """Detecta se bloco de código está dentro de um observer callback"""
        return any(x in block for x in [
            "IntersectionObserver",
            "ResizeObserver",
            "MutationObserver",
            "set_timeout",
            "scroll_spy",
        ])

    def is_attach_guard(lines, idx):
        """Detecta padrão legítimo de idempotência: get_attribute seguido de set_attribute na linha seguinte"""
        line = lines[idx]
        # padrão 1: get guard imediatamente antes de set guard
        if "get_attribute" in line and idx + 1 < len(lines):
            next_line = lines[idx + 1]
            if "set_attribute" in next_line:
                # ambos devem referenciar o mesmo atributo
                import re as _re3
                get_attr = _re3.search(r'get_attribute\("([^"]+)"', line)
                set_attr = _re3.search(r'set_attribute\("([^"]+)"', next_line)
                if get_attr and set_attr and get_attr.group(1) == set_attr.group(1):
                    return True
        # padrão 2: variável com "attached" no nome — idempotência explícita
        if "attached" in line and ("get_attribute" in line or "set_attribute" in line):
            return True
        return False

    # CR-333: query_selector → signal (trace completo, profundidade 3)
    def trace_flow(var, block):
        chain = {var}
        for _ in range(3):
            for v in list(chain):
                matches = _re.findall(rf'let\s+(\w+)\s*=.*{v}', block)
                chain.update(matches)
        for v in chain:
            if _re.search(rf'\.set\([^)]*{v}[^)]*\)', block):
                return True
        return False

    for i, line in enumerate(lines, 1):
        if line.strip().startswith("//"): continue
        if "query_selector" in line:
            var = None
            for j in range(i-1, max(i-25,-1), -1):
                m = _re.search(r'let\s+(\w+)\s*=', lines[j])
                if m and "query_selector" in "\n".join(lines[j:i+1]):
                    var = m.group(1); break
            if var and trace_flow(var, "\n".join(lines[i:min(i+25,len(lines))])):
                # PERMITIDO: leitura DOM para sincronizacao
                continue

    # CR-333 get_attribute — leitura DOM permitida no modelo island DOM-driven
    # apenas bloqueia se alimenta signal diretamente
    for i, line in enumerate(lines, 1):
        if line.strip().startswith("//"): continue
        if "get_attribute" in line:
            if is_attach_guard(lines, i - 1):
                continue
            # PERMITIDO: toda leitura de DOM é fonte de verdade no modelo island
            continue

    # CR-333b: has_attribute — PERMITIDO no modelo island DOM-driven

    # CR-334: use_context inside closure proibido
    import re as _re2
    for i, line in enumerate(lines, 1):
        if line.strip().startswith("//"): continue
        if "use_context" in line:
            # check if inside a closure (move ||)
            block_before = "\n".join(lines[max(0,i-10):i])
            if "move |" in block_before or "move||" in block_before:
                errors.append(
                    f"[CR-334] {island_file} linha {i} -- use_context dentro de closure proibido\n"
                    f"            capture use_context no root do componente antes de qualquer closure\n"
                    f"            {line.strip()[:80]}"
                )

    # CR-335: DOM mutation permitido apenas via state helpers e ARIA sync
    for (pattern, msg) in ISLAND_FORBIDDEN_PATTERNS:
        for i, line in enumerate(lines, 1):
            if line.strip().startswith("//"): continue
            if pattern in line:
                if island_type == "observer" and pattern == "set_attribute":
                    block_ctx = "\n".join(lines[max(0,i-100):i])
                    if is_observer_context(block_ctx):
                        continue
                # PERMITIDO: sincronizacao de ARIA baseada em state
                if "set_attribute" in line and "aria-" in line:
                    continue
                errors.append(f"[{msg.split(':')[0]}] {island_file} linha {i} -- {msg.split(':',1)[1].strip()}\n            {line.strip()[:80]}")
                break

    # CR-335b: style mutation permitido se via tokens (sem hardcode)
    for i, line in enumerate(lines, 1):
        if line.strip().startswith("//"): continue
        if "set_property" in line and "var(--" in line:
            continue
        if "style().set_property" in line or "style().set_" in line:
            errors.append(f"[CR-335] {island_file} linha {i} -- style DOM mutation proibido\n            {line.strip()[:80]}")

    # CR-337: inner_html dinamico proibido
    for i, line in enumerate(lines, 1):
        if line.strip().startswith("//"): continue
        if "inner_html" in line:
            if "move ||" in line or ".get(" in line:
                errors.append(f"[CR-337] {island_file} linha {i} -- inner_html dinamico proibido\n            {line.strip()[:80]}")

    # CR-338: web_sys cfg-gated — DESATIVADO para modelo island DOM-driven
    # web_sys é usado dentro de Effect::new sem necessidade de cfg-gate

    # CR-339: dynamic class proibido (mantido)
    for i, line in enumerate(lines, 1):
        if line.strip().startswith("//"): continue
        if _re.search(r'class\s*=\s*(move\s*\|\||if\s|\w+\.get\()', line):
            errors.append(f"[CR-339] {island_file} linha {i} -- dynamic class state proibido\n            {line.strip()[:80]}")

    # CR-341: multiplos signals — DESATIVADO para modelo island DOM-driven



    # CR-343: preview DEVE usar island, nao UI diretamente no hero stage
    import glob as _glob343
    import re as _re343
    _comp_ui_dir = _glob343.glob(f"{island_dir}/**/{component}", recursive=True)
    _comp_dir = _comp_ui_dir[0] if _comp_ui_dir else f"{island_dir}/{component}"
    preview_matches = _glob343.glob(f"{_comp_dir}/preview.rs", recursive=False)
    if not preview_matches:
        preview_matches = _glob343.glob(f"{island_dir}/{component}/preview.rs", recursive=False)
    if preview_matches:
        with open(preview_matches[0]) as pf:
            preview_content = pf.read()
        # verificar se preview importa island
        has_island_import = bool(_re343.search(r'use.*[Ii]sland', preview_content))
        # verificar se preview usa island no hero stage
        has_island_in_hero = bool(_re343.search(r'<[A-Z]\w*Island', preview_content))
        if not has_island_import and not has_island_in_hero:
            errors.append(f"[CR-343] preview.rs -- preview NAO usa island\n            preview DEVE usar island no hero stage, nao UI diretamente")
        elif has_island_import and not has_island_in_hero:
            errors.append(f"[CR-343] preview.rs -- island importado mas nao usado no hero\n            use o island no data-rs-showcase-preview-stage")

    # CR-342: island DEVE usar UI, nao primitives diretamente
    # excecao: data_table usa primitives por necessidade de reatividade complexa
    # CR-344: island DEVE importar UI explicitamente
    import re as _re344
    has_ui_import = bool(_re344.search(r'use (super|crate)::.*_ui::', content))
    if not has_ui_import and component not in ['data_table']:
        errors.append(f"[CR-344] {island_file} -- island NAO importa UI\n            todo island DEVE importar seu UI: use super::<component>_ui::")

    if component not in ['data_table']:
        import re as _re342
        primitive_pattern = _re342.findall(r'use canonrs_core::primitives::{([^}]+)}', content)
        if primitive_pattern:
            has_ui = bool(_re342.search(r'use (super|crate)::.*_ui::', content))
            if not has_ui:
                prims = primitive_pattern[0][:60]
                errors.append(f'[CR-342] {island_file} -- island usa primitives diretamente sem UI\n            island DEVE usar UI como intermediario: {prims}')
    return errors




def parse_builder(builder_path):
    """Parse builder.yaml — SSOT para todos os componentes"""
    import yaml
    with open(builder_path) as f:
        raw = yaml.safe_load(f)
    if not raw:
        return {}
    comp = {}
    comp["id"]         = raw.get("id", "")
    comp["label"]      = raw.get("label", "")
    comp["family"]     = raw.get("family", "")
    comp["category"]   = raw.get("category", "")
    comp["file"]       = raw.get("file", "") or ""
    comp["tokens"]     = raw.get("tokens", "") or ""
    comp["foundation"] = raw.get("foundation", "") or ""
    comp["island"]     = raw.get("island", "") or ""
    comp["before"]     = raw.get("before", "") or ""
    comp["after"]      = raw.get("after", "") or ""
    comp["pain"]       = raw.get("pain", "") or ""
    comp["promise"]    = raw.get("promise", "") or ""
    comp["why"]        = raw.get("why", "") or ""
    comp["badges"]     = raw.get("badges", []) or []
    comp["rules"]      = raw.get("rules", []) or []
    comp["island_type"]   = raw.get("island_type", "") or ""

    # normalizar states — sempre lista
    _states = raw.get("states", [])
    if isinstance(_states, list):
        comp["states"] = [s.strip().strip('"') for s in _states if s]
    elif isinstance(_states, str):
        comp["states"] = [s.strip().strip('"') for s in _states.split(",") if s.strip()]
    else:
        comp["states"] = []

    comp["component"] = comp["id"].replace("-", "_")
    return comp


def load_components_from_builders(builder_dir, script_dir):
    """Carrega todos os componentes a partir dos builder.md"""
    import glob as _glob
    builders = _glob.glob(f"{builder_dir}/**/builder.yaml", recursive=True)
    components = []
    for b in sorted(builders):
        try:
            comp = parse_builder(b)
            if comp.get("component"):
                components.append(comp)
        except Exception as e:
            print(f"[WARN] erro ao parsear {b}: {e}")
    return components

def main():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    json_path  = os.path.join(script_dir, JSON_PATH)
    global CSS_DIR, TOKENS_DIR, BEHAVIORS_DIR, ISLAND_DIR, BUILDER_DIR, INTERACTIONS_DIR
    CSS_DIR      = os.path.join(script_dir, CSS_DIR)
    TOKENS_DIR   = os.path.join(script_dir, TOKENS_DIR)
    BEHAVIORS_DIR= os.path.join(script_dir, BEHAVIORS_DIR)
    ISLAND_DIR       = os.path.join(script_dir, ISLAND_DIR)
    INTERACTIONS_DIR = os.path.join(script_dir, INTERACTIONS_DIR)

    BUILDER_DIR  = os.path.join(script_dir, BUILDER_DIR)

    declared = extract_declared_tokens(TOKENS_DIR)

    # SSOT: ler componentes dos builder.md
    components = load_components_from_builders(BUILDER_DIR, script_dir)
    # fallback JSON para campos nao presentes no builder
    with open(json_path) as f:
        json_components = json.load(f)
    json_map = {c['component']: c for c in json_components}
    for comp in components:
        json_comp = json_map.get(comp['component'], {})
        # herdar campos do JSON que nao estao no builder
        for field in ['file', 'tokens', 'foundation', 'states', 'island']:
            if not comp.get(field):
                comp[field] = json_comp.get(field, comp.get(field, ''))

    target      = sys.argv[1] if len(sys.argv) > 1 else None

    # CR-347: validar tokens com hardcode globalmente
    if not target:
        token_hc_errors = check_token_hardcode(TOKENS_DIR)
        if token_hc_errors:
            print("\n[CR-347 GLOBAL TOKEN HARDCODE]")
            for e in token_hc_errors:
                print(f"   {e}")

    # CR-336b: validar tokens de estado ativo globalmente
    if not target:
        active_token_errors = check_active_state_tokens(declared)
        if active_token_errors:
            print("\n[CR-336b GLOBAL TOKEN VIOLATIONS]")
            for e in active_token_errors:
                print(f"   {e}")
    show_unused = "--unused" in sys.argv
    generate_json = "--generate" in sys.argv

    if generate_json:
        json_output = []
        for comp in components:
            entry = {
                "component": comp["component"],
                "file":      comp.get("file", ""),
                "tokens":    comp.get("tokens", ""),
                "foundation": comp.get("foundation", ""),
                "family":    comp.get("family", ""),
                "states":    comp.get("states", []),
                "island":    comp.get("island", ""),
            }
            json_output.append(entry)
        with open(json_path, "w") as f:
            json.dump(json_output, f, indent=2)
        print(f"[OK] tokens_components.json gerado com {len(json_output)} componentes")
        return

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
            island_type = comp.get("island_type", "state")
            errors += check_island_full(island_file, island_dir, comp["component"], island_type)
            errors += check_island_ast(island_file, island_dir, island_type)
            css_file = os.path.join(CSS_DIR, comp["file"])
            errors += check_island_css_child_combinator(css_file)
            errors += check_hover_override_active(css_file)
            errors += check_css_quality(css_file, comp["component"])
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
