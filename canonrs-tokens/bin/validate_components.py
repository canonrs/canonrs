#!/usr/bin/env python3
"""
CanonRS Component Token Validator
Validates CSS files against their token contracts defined in tokens_components.json
"""

import json
import re
import sys
import os

JSON_PATH = "../src/design/tokens/tokens_components.json"
CSS_DIR   = "../../canonrs-server/styles/ui"

# Foundation prefix map
FOUNDATION_PREFIXES = {
    "spacing":     ["--space-"],
    "size":        ["--size-"],
    "radius":      ["--radius-"],
    "motion":      ["--motion-"],
    "typography":  ["--font-", "--line-height-"],
    "shadow":      ["--shadow-"],
    "border":      ["--border-"],
    "interaction": ["--opacity-", "--transform-", "--focus-ring-", "--blur-"],
    "z":           ["--z-", "--layer-"],
    "color":       ["--color-overlay-", "--color-primary-alpha-"],
}

# Tokens that are always allowed (theme, semantic)
ALWAYS_ALLOWED = [
    "--theme-",
    "--color-",
    "--primitive-",
    "--layer-",
    "--state-",
    "--focus-ring-shadow",
    "--validation-",
    "--button-active-transform",
]

def extract_vars(css: str) -> list[str]:
    return re.findall(r"var\((--[a-zA-Z0-9-]+)", css)

def is_allowed(var: str, component: str, tokens: list[str], foundations: list[str]) -> tuple[bool, str]:
    # always allowed
    for prefix in ALWAYS_ALLOWED:
        if var.startswith(prefix):
            return True, ""

    # component tokens (e.g. button-*, space-*)
    for pattern in tokens:
        prefix = pattern.replace("*", "").replace(" ", "")
        if var.startswith(f"--{prefix}"):
            return True, ""

    # foundation tokens
    for foundation in foundations:
        for prefix in FOUNDATION_PREFIXES.get(foundation, []):
            if var.startswith(prefix):
                return True, ""

    return False, f"❌ {var} — not declared in @tokens or @foundation"

def validate(component: dict) -> list[str]:
    css_file = os.path.join(CSS_DIR, component["file"])
    if not os.path.exists(css_file):
        return [f"⚠️  {component['file']} — file not found"]

    with open(css_file) as f:
        css = f.read()

    vars_used = extract_vars(css)
    tokens    = [t.strip() for t in component["tokens"].split(",")]
    foundations = [f.strip() for f in component["foundation"].split(",")]

    errors = []
    seen   = set()
    for var in vars_used:
        if var in seen:
            continue
        seen.add(var)
        ok, msg = is_allowed(var, component["component"], tokens, foundations)
        if not ok:
            errors.append(msg)

    return errors

def main():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    json_path  = os.path.join(script_dir, JSON_PATH)
    global CSS_DIR
    CSS_DIR    = os.path.join(script_dir, CSS_DIR)

    with open(json_path) as f:
        components = json.load(f)

    target = sys.argv[1] if len(sys.argv) > 1 else None

    total_errors = 0
    total_ok     = 0

    for comp in components:
        if target and comp["component"] != target:
            continue

        errors = validate(comp)
        if errors:
            print(f"\n🔴 {comp['component'].upper()}")
            for e in errors:
                print(f"   {e}")
            total_errors += len(errors)
        else:
            total_ok += 1
            if target:
                print(f"\n✅ {comp['component'].upper()} — clean")

    print(f"\n{'='*50}")
    print(f"✅ {total_ok} components clean")
    if total_errors:
        print(f"❌ {total_errors} violations found")
        sys.exit(1)
    else:
        print("🎯 All components canonical")

if __name__ == "__main__":
    main()
