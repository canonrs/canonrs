"""
registry.py — Component Registry (SSOT: builder.yaml)
"""

import glob
import os


def parse_builder(builder_path: str) -> dict:
    import yaml
    with open(builder_path) as f:
        raw = yaml.safe_load(f)
    if not raw:
        return {}
    comp = {
        "id":            raw.get("id", ""),
        "label":         raw.get("label", ""),
        "family":        raw.get("family", ""),
        "category":      raw.get("category", ""),
        "file":          raw.get("file", "") or "",
        "tokens":        raw.get("tokens", "") or "",
        "foundation":    raw.get("foundation", "") or "",
        "island":        raw.get("island", "") or "",
        "boundary_type": raw.get("boundary_type", "") or "",
        "badges":        raw.get("badges", []) or [],
        "rules":         raw.get("rules", []) or [],
    }
    _states = raw.get("states", [])
    if isinstance(_states, list):
        comp["states"] = [s.strip().strip('"') for s in _states if s]
    elif isinstance(_states, str):
        comp["states"] = [s.strip().strip('"') for s in _states.split(",") if s.strip()]
    else:
        comp["states"] = []
    comp["component"] = comp["id"].replace("-", "_")
    return comp


def load_components(builder_dir: str) -> list:
    components = []
    for b in sorted(glob.glob(f"{builder_dir}/**/builder.yaml", recursive=True)):
        try:
            comp = parse_builder(b)
            if comp.get("component"):
                components.append(comp)
        except Exception as e:
            print(f"[WARN] erro ao parsear {b}: {e}")
    return components


def merge_json_fallback(components: list, json_path: str) -> list:
    """Herda campos do JSON legado para campos ausentes no builder.yaml"""
    import json
    with open(json_path) as f:
        json_components = json.load(f)
    json_map = {c['component']: c for c in json_components}
    for comp in components:
        json_comp = json_map.get(comp['component'], {})
        for field in ['file', 'tokens', 'foundation', 'states', 'island']:
            if not comp.get(field):
                comp[field] = json_comp.get(field, comp.get(field, ''))
    return components
