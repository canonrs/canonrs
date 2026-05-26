#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""check_cargo_features.py — Cargo.toml web-sys features governance"""
import sys, re, glob
from pathlib import Path

ROOT = Path(_CANONRS_ROOT + "")

# web-sys types used in code -> required feature name
TYPE_TO_FEATURE = {
    "CustomEvent":      "CustomEvent",
    "CustomEventInit":  "CustomEventInit",
    "Navigator":        "Navigator",
    "Clipboard":        "Clipboard",
    "ResizeObserver":   "ResizeObserver",
    "MutationObserver": "MutationObserver",
    "IntersectionObserver": "IntersectionObserver",
    "HtmlCanvasElement": "HtmlCanvasElement",
    "CanvasRenderingContext2d": "CanvasRenderingContext2d",
    "HtmlInputElement": "HtmlInputElement",
    "HtmlTextAreaElement": "HtmlTextAreaElement",
    "PointerEvent":     "PointerEvent",
    "KeyboardEvent":    "KeyboardEvent",
    "FocusEvent":       "FocusEvent",
}

CRATES = [
    "canonrs-interactions-gesture",
    "canonrs-interactions-overlay",
    "canonrs-interactions-data",
    "canonrs-interactions-selection",
    "canonrs-interactions-content",
    "canonrs-interactions-nav",
    "canonrs-interactions-init",
]

def get_declared_features(cargo_path):
    content = open(cargo_path).read()
    m = re.search(r'web-sys.*?features\s*=\s*\[(.*?)\]', content, re.DOTALL)
    if not m: return set()
    return set(re.findall(r'"([^"]+)"', m.group(1)))

def get_used_types(src_dir):
    used = set()
    for rs in Path(src_dir).rglob("*.rs"):
        src = rs.read_text()
        for type_name in TYPE_TO_FEATURE:
            if type_name in src:
                used.add(type_name)
    return used

def run():
    errors = []

    for crate in CRATES:
        cargo = ROOT / crate / "Cargo.toml"
        src   = ROOT / crate / "src"
        if not cargo.exists(): continue

        declared  = get_declared_features(cargo)
        used_types = get_used_types(src)
        missing = []
        for type_name in used_types:
            feature = TYPE_TO_FEATURE[type_name]
            if feature not in declared:
                missing.append(f"{type_name} -> feature:{feature}")

        if missing:
            for m in missing:
                errors.append(f"[CR-FEAT-001] {crate} uses {m} but feature not in Cargo.toml")
        else:
            print(f"  [OK] {crate}: features complete")

    print("\n" + "="*50)
    if errors:
        for e in errors: print(f"[ERRO] {e}")
        print(f"[FAIL] {len(errors)} cargo feature violations")
        return 1

    print("[OK] All Cargo.toml features governance compliant")
    return 0

if __name__ == "__main__":
    sys.exit(run())