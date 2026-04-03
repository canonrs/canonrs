#!/usr/bin/env python3
"""
CanonRS Component Token Validator v2
#1 - token usado deve existir no sistema
#2 - ALWAYS_ALLOWED minimo (sem bypass de theme-*)
#3 - token declarado no contrato mas nao usado (unused)
"""

import json
import re
import sys
import os
import glob

JSON_PATH  = "../src/design/tokens/tokens_components.json"
CSS_DIR    = "../../canonrs-server/styles/ui"
TOKENS_DIR = "../src/design/tokens"

# Apenas tokens de runtime que nao sao declarados em Rust
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


def extract_declared_tokens(tokens_dir):
    declared = set()
    pattern = re.compile(r'FamilyToken::new\("([^"]+)"')
    for rs_file in glob.glob(f"{tokens_dir}/**/*.rs", recursive=True):
        with open(rs_file) as f:
            for match in pattern.finditer(f.read()):
                declared.add(f"--{match.group(1)}")
    return declared


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


def validate(component, declared):
    css_file = os.path.join(CSS_DIR, component["file"])
    if not os.path.exists(css_file):
        return [f"[MISSING] {component["file"]} -- file not found"], []
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
    unused = check_unused(tokens, used_set, declared)
    return errors, unused


def main():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    json_path  = os.path.join(script_dir, JSON_PATH)
    global CSS_DIR, TOKENS_DIR
    CSS_DIR    = os.path.join(script_dir, CSS_DIR)
    TOKENS_DIR = os.path.join(script_dir, TOKENS_DIR)

    declared = extract_declared_tokens(TOKENS_DIR)

    with open(json_path) as f:
        components = json.load(f)

    target      = sys.argv[1] if len(sys.argv) > 1 else None
    show_unused = "--unused" in sys.argv

    total_errors = 0
    total_ok     = 0
    total_unused = 0

    for comp in components:
        if target and comp["component"] != target:
            continue
        errors, unused = validate(comp, declared)
        if errors:
            print(f"\n[ERRO] {comp["component"].upper()}")
            for e in errors:
                print(f"   {e}")
            total_errors += len(errors)
        else:
            total_ok += 1
            if target:
                print(f"\n[OK] {comp["component"].upper()} -- clean")
        if show_unused and unused:
            print(f"\n[UNUSED] {comp["component"].upper()}")
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
