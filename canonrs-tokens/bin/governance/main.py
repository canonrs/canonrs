#!/usr/bin/env python3
"""
governance/main.py — CanonRS Governance Engine
Orquestrador modular: delega para cada camada especializada.
Compatível com validate_components.py original — mesma CLI, mesmos paths.
"""

import sys
import os
import json

# paths — mesmo base do validate_components.py original (bin/)
BIN_DIR         = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

TOKENS_DIR      = os.path.join(BIN_DIR, "../src/design/tokens")
CSS_DIR         = os.path.join(BIN_DIR, "../../canonrs-server/styles/ui")
LAYOUTS_CSS_DIR = os.path.join(BIN_DIR, "../../canonrs-server/styles/layouts")
BEHAVIORS_DIR   = os.path.join(BIN_DIR, "../../canonrs-client/src/behaviors")
UI_DIR          = os.path.join(BIN_DIR, "../../canonrs-server/src/ui")
PRIMITIVES_DIR  = os.path.join(BIN_DIR, "../../canonrs-core/src/primitives")
BUILDER_DIR     = os.path.join(BIN_DIR, "../../canonrs-server/src/ui")
JSON_PATH       = os.path.join(BIN_DIR, "../src/design/tokens/tokens_components.json")

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from tokens       import extract_declared_tokens, extract_vars, is_allowed, check_unused, check_token_hardcode
from css          import check_css_quality, check_states_in_css, check_hover_override_active, check_child_combinator, check_layout_contract
from architecture import check_primitive, check_ui, check_boundary, check_preview
from behavior     import check_state_engine_violations, check_states_in_behavior, check_registered, check_active_state_tokens
from boundary_full import check_boundary_full
from ast_check    import check_boundary_ast
from registry     import load_components, merge_json_fallback


def validate_component(comp: dict, declared: set, show_unused: bool = False) -> tuple:
    errors  = []
    unused  = []
    cid     = comp["component"]
    css_file = os.path.join(CSS_DIR, comp.get("file", ""))

    # CSS tokens
    if os.path.exists(css_file):
        with open(css_file) as f:
            css = f.read()
        vars_used = extract_vars(css)
        tokens    = [t.strip() for t in comp.get("tokens","").split(",")]
        foundations = [f.strip() for f in comp.get("foundation","").split(",")]
        seen = set()
        for var in vars_used:
            if var in seen: continue
            seen.add(var)
            ok, msg = is_allowed(var, tokens, foundations, declared)
            if not ok:
                errors.append(msg)
        if show_unused:
            unused = check_unused(tokens, set(vars_used), declared)

        # CSS quality
        errors += check_css_quality(css_file, cid)
        errors += check_hover_override_active(css_file)
        errors += check_child_combinator(css_file)

    # CR-338: boundary_type
    boundary_type = comp.get("boundary_type", "")
    valid_types = ["passthrough", "init", "interaction"]
    if not boundary_type:
        errors.append(f"[CR-338] {cid} -- boundary_type ausente no builder.yaml")
    elif boundary_type not in valid_types:
        errors.append(f"[CR-338] {cid} -- boundary_type invalido: '{boundary_type}'")

    # States
    states = comp.get("states", [])
    if os.path.exists(css_file):
        errors += check_states_in_css(states, open(css_file).read())

    behavior_file = comp.get("behavior")
    registered    = comp.get("registered")
    errors += check_states_in_behavior(states, behavior_file, BEHAVIORS_DIR)
    auto_init = os.path.join(BEHAVIORS_DIR, "auto_init.rs")
    errors += check_registered(behavior_file, registered, auto_init)

    # Architecture layers
    errors += check_primitive(cid, PRIMITIVES_DIR)
    errors += check_ui(cid, UI_DIR)
    errors += check_boundary(cid, boundary_type, UI_DIR)
    errors += check_preview(cid, UI_DIR)

    # Boundary full (ex-island)
    boundary_file = comp.get("island", "")
    if boundary_file:
        errors += check_boundary_full(boundary_file, UI_DIR, cid, boundary_type)
        errors += check_boundary_ast(boundary_file, UI_DIR, boundary_type)
        errors += check_hover_override_active(css_file)
        errors += check_child_combinator(css_file)
        errors += check_css_quality(css_file, cid)

    return errors, unused


def main():
    target      = sys.argv[1] if len(sys.argv) > 1 else None
    show_unused = "--unused" in sys.argv
    json_output = "--json" in sys.argv

    declared   = extract_declared_tokens(TOKENS_DIR)
    components = load_components(BUILDER_DIR)
    components = merge_json_fallback(components, JSON_PATH)

    # Global checks
    if not target:
        hc_errors = check_token_hardcode(TOKENS_DIR)
        if hc_errors:
            print("\n[CR-347 GLOBAL TOKEN HARDCODE]")
            for e in hc_errors: print(f"   {e}")

        active_errors = check_active_state_tokens(declared, TOKENS_DIR)
        if active_errors:
            print("\n[CR-336b GLOBAL TOKEN VIOLATIONS]")
            for e in active_errors: print(f"   {e}")

        se_errors = check_state_engine_violations(BEHAVIORS_DIR)
        if se_errors:
            print("\n[STATE-ENGINE VIOLATIONS]")
            for e in se_errors: print(f"   {e}")

        layout_errors = check_layout_contract(LAYOUTS_CSS_DIR)
        if layout_errors:
            print("\n[CR-390 LAYOUT CONTRACT VIOLATIONS]")
            for e in layout_errors: print(f"   {e}")
        else:
            print("\n[CR-390] Layout contract: OK")

    total_errors  = 0
    total_ok      = 0
    total_unused  = 0

    for comp in components:
        if target and comp["component"] != target:
            continue
        try:
            errors, unused = validate_component(comp, declared, show_unused)
        except Exception as e:
            print(f"[EXCEPTION] {comp['component']}: {e}")
            import traceback; traceback.print_exc()
            continue
        if errors:
            print(f"\n[ERRO] {comp['component'].upper()}")
            for e in errors: print(f"   {e}")
            total_errors += len(errors)
        else:
            total_ok += 1
            if target:
                print(f"\n[OK] {comp['component'].upper()} -- clean")
        if show_unused and unused:
            print(f"\n[UNUSED] {comp['component'].upper()}")
            for u in unused: print(f"   {u}")
            total_unused += len(unused)

    print(f"\n{'='*50}")
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
