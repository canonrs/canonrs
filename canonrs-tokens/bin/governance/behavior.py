"""
behavior.py — Behavior/State governance layer
Validações: state engine, states in behavior, registered
"""

import re
import glob
import os

STATE_ENGINE_VIOLATIONS = [
    'get_attribute("data-rs-disabled").as_deref() == Some("disabled")',
    'get_attribute("data-rs-state").as_deref() == Some("disabled")',
    'data-rs-open',
    'data-rs-active',
    'data-rs-visible',
    'data-rs-attached',
    'data-rs-copy-attached',
]

STATE_ENGINE_ALLOWED = [
    'has_attribute("data-rs-disabled")',
    'add_state', 'remove_state', 'remove_states',
]


def check_state_engine_violations(behaviors_dir: str) -> list:
    errors = []
    for rs_file in glob.glob(f"{behaviors_dir}/*.rs"):
        with open(rs_file) as f:
            content = f.read()
        filename = os.path.basename(rs_file)
        for violation in STATE_ENGINE_VIOLATIONS:
            if violation in content:
                for line in content.splitlines():
                    if violation in line and not line.strip().startswith("//"):
                        errors.append(f"[STATE-ENGINE] {filename} -- viola padrao canonico: {violation[:50]}")
                        break
    return errors


def check_states_in_behavior(states: list, behavior_file: str, behaviors_dir: str) -> list:
    if not behavior_file:
        return []
    full_path = os.path.join(behaviors_dir, behavior_file)
    if not os.path.exists(full_path):
        return [f"[BEHAVIOR-MISSING] {behavior_file} nao encontrado\n            esperado em: {full_path}"]
    errors = []
    with open(full_path) as f:
        content = f.read()
    has_add_state = 'add_state(' in content
    for state in states:
        if not has_add_state and f'"{state}"' not in content:
            errors.append(f"[STATE-BEHAVIOR] estado '{state}' ausente no behavior\n            adicione: add_state(el, \"{state}\") em {behavior_file}")
        elif f'"{state}"' not in content:
            errors.append(f"[STATE-BEHAVIOR] estado '{state}' nao encontrado em {behavior_file}")
    return errors


def check_registered(behavior_file: str, registered, auto_init_path: str) -> list:
    if not behavior_file or registered is None or not registered:
        return []
    with open(auto_init_path) as f:
        content = f.read()
    module = behavior_file.replace(".rs", "")
    if f"{module}::register()" not in content:
        return [f"[NOT-REGISTERED] {behavior_file} nao registrado\n            adicione em auto_init.rs: {module}::register();"]
    return []


def check_active_state_tokens(declared: set, tokens_dir: str) -> list:
    """CR-336b: tokens *-fg-active devem usar --theme-action-primary-*"""
    errors = []
    pattern = re.compile(r'FamilyToken::new\("([^"]+)",\s*"([^"]+)"\)')
    ACTIVE_FG_SUFFIXES = ["-fg-active", "-fg-open", "-fg-selected", "-fg-checked"]
    ACTIVE_BORDER_SUFFIXES = ["-border-active", "-border-checked", "-border-selected"]

    for rs_file in glob.glob(f"{tokens_dir}/**/*.rs", recursive=True):
        with open(rs_file) as f:
            src = f.read()
        for match in pattern.finditer(src):
            name  = match.group(1)
            value = match.group(2)
            token = f"--{name}"
            for suffix in ACTIVE_FG_SUFFIXES:
                if token.endswith(suffix):
                    if not any(v in value for v in ["--theme-action-primary", "--color-primary"]):
                        errors.append(f"[CR-336b] {token} = {value} -- deve usar --theme-action-primary-fg")
            for suffix in ACTIVE_BORDER_SUFFIXES:
                if token.endswith(suffix):
                    if not any(v in value for v in ["--theme-action-primary", "--color-primary"]):
                        errors.append(f"[CR-336b] {token} = {value} -- deve usar --theme-action-primary-bg")
    return errors
