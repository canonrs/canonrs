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
    path = os.path.join(behaviors_dir, behavior_file)
    if not os.path.exists(path):
        return [f"[BEHAVIOR-MISSING] {behavior_file} -- arquivo nao encontrado"]
    errors = []
    with open(path) as f:
        content = f.read()
    for state in states:
        if f'add_state(' not in content and f'"{state}"' not in content:
            errors.append(f"[STATE-BEHAVIOR] estado '{state}' nao orquestrado no behavior")
        elif f'"{state}"' not in content:
            errors.append(f"[STATE-BEHAVIOR] estado '{state}' nao encontrado no behavior")
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
        return [f"[NOT-REGISTERED] {behavior_file} -- nao registrado em auto_init.rs"]
    return []


def validate(component, declared):
    css_file = os.path.join(CSS_DIR, component["file"])
    if not os.path.exists(css_file):
        return [f"[MISSING] {component['file']} -- file not found"], []
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


def main():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    json_path  = os.path.join(script_dir, JSON_PATH)
    global CSS_DIR, TOKENS_DIR, BEHAVIORS_DIR
    CSS_DIR      = os.path.join(script_dir, CSS_DIR)
    TOKENS_DIR   = os.path.join(script_dir, TOKENS_DIR)
    BEHAVIORS_DIR= os.path.join(script_dir, BEHAVIORS_DIR)

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
