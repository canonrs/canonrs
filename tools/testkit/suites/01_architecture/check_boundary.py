#!/usr/bin/env python3
import re, glob, os, sys, yaml
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')


UI_DIR   = _CANONRS_ROOT + "/canonrs-server/src/ui"
INT_BASE = _CANONRS_ROOT + ""

SIGNAL_PATTERNS = ["create_signal", "signal(", "RwSignal::new", "create_rw_signal", "create_memo"]
BOUNDARY_HTML_EXCEPTIONS = ["alert_dialog", "drawer", "sheet", "status_dot", "switch", "toggle_group", "toolbar"]
COMPOSITE_EXCEPTIONS = ["sidebar", "data_table", "dropdown_menu"]
HTML_RE = re.compile(r"<(div|span|button|input|textarea|select|form|ul|ol|li|table)\b")
INT_GROUPS = [
    "canonrs_interactions_nav", "canonrs_interactions_overlay",
    "canonrs_interactions_init", "canonrs_interactions_selection",
    "canonrs_interactions_gesture", "canonrs_interactions_content",
    "canonrs_interactions_data", "canonrs_interactions_core",
]
VALID_INT_GROUPS = ["overlay", "nav", "selection", "data", "content", "gesture", "init"]


def load_registry():
    registry = {}
    for path in glob.glob(f"{UI_DIR}/**/builder.yaml", recursive=True):
        try:
            with open(path) as f:
                data = yaml.safe_load(f)
            if data and "id" in data:
                raw_id = data["id"]
                registry[raw_id] = data
                registry[raw_id.replace("-", "_")] = data
        except Exception:
            pass
    return registry


def discover_interaction_groups():
    dirs = glob.glob(f"{INT_BASE}/canonrs-interactions-*")
    groups = set()
    for d in dirs:
        name = os.path.basename(d).replace("canonrs-interactions-", "")
        groups.add(name)
    return groups


def detect_type(src, registry, cid):
    """Usa registry como autoridade para boundary_type."""
    entry = registry.get(cid, {})
    btype = entry.get("boundary_type", "") or ""
    if btype:
        return btype
    # fallback: inferir do codigo
    if re.search(r"if\s+\w+\s*\{\s*\w+State::", src):
        return "init"
    if any(g in src for g in INT_GROUPS):
        return "interaction"
    return "passthrough"


def has_ui_import(src, cid):
    """Verifica se boundary importa seu UI — aceita varios padroes."""
    patterns = [
        r"use super::\w+_ui::",
        r"use (crate|super)::.*_ui::",
        r"super::\w+_ui::\w+",  # uso inline no view!
        r"pub use crate::",      # re-exports
    ]
    return any(bool(re.search(p, src)) for p in patterns)


def check_file(path, cid, registry, valid_groups):
    errors = []
    src = open(path).read()
    src_clean = re.sub(r"//[^\n]*", "", src)
    btype = detect_type(src, registry, cid)

    # CR-370: signals proibidos
    for sig in SIGNAL_PATTERNS:
        if sig in src_clean:
            errors.append(f"[CR-370] {cid}_boundary — signal proibido: {sig}\n"
                          f"         boundary nao cria estado reativo")
            break

    # CR-371: closures reativas proibidas
    if re.search(r"move\s*\|\s*\|", src_clean):
        errors.append(f"[CR-371] {cid}_boundary — closure reativa (move ||) proibida\n"
                      f"         boundary nao tem reatividade")

    # CR-372: logica de negocio proibida
    for bl in ["use_navigate", "fetch(", "reqwest::", "http::", "async fn", "spawn_local"]:
        if bl in src_clean:
            errors.append(f"[CR-372] {cid}_boundary — logica de negocio proibida: {bl}")
            break

    # CR-373: provide_context proibido
    if "provide_context" in src_clean:
        errors.append(f"[CR-373] {cid}_boundary — provide_context proibido")

    # CR-374: HTML direto proibido
    if cid not in BOUNDARY_HTML_EXCEPTIONS and cid not in COMPOSITE_EXCEPTIONS:
        tags = HTML_RE.findall(src_clean)
        if tags:
            errors.append(f"[CR-374] {cid}_boundary — HTML direto proibido: <{tags[0]}>\n"
                          f"         boundary delega para UI, nao reconstroi DOM")

    # CR-375: deve importar seu UI
    if not has_ui_import(src, cid) and cid not in ["data_table"]:
        errors.append(f"[CR-375] {cid}_boundary — nao importa seu UI\n"
                      f"         todo boundary DEVE importar <component>_ui")

    # CR-376: Tipo 1 passthrough sem branching excessivo
    if btype == "passthrough":
        clean = re.sub(r"let\s+_[^=]*=.*?;", "", src_clean)
        arrows = re.findall(r"=>", clean)
        if len(arrows) > 6:
            errors.append(f"[CR-376] {cid}_boundary — passthrough com branching excessivo "
                          f"({len(arrows)} =>)\n"
                          f"         passthrough = zero logica")

    # CR-378: Tipo 3 deve usar grupo de interactions valido
    if btype == "interaction":
        entry = registry.get(cid, {})
        ix_group = (entry.get("ix_group", "") or "").replace("ix_", "")
        found = ix_group in valid_groups or any(
            f"canonrs_interactions_{g}" in src for g in valid_groups
        )
        if not found:
            errors.append(f"[CR-378] {cid}_boundary — Tipo 3 sem grupo de interactions valido\n"
                          f"         grupos validos: {', '.join(sorted(valid_groups))}")

    return errors, btype


def run(target=None):
    files = glob.glob(f"{UI_DIR}/**/*_boundary.rs", recursive=True)
    if not files:
        print(f"\n[FAIL] 0 files analyzed — path: {UI_DIR}")
        return 1

    registry     = load_registry()
    valid_groups = discover_interaction_groups()
    if not valid_groups:
        valid_groups = {"init", "nav", "overlay", "selection", "data", "content", "gesture", "dismiss"}

    total_ok          = 0
    failed_components = 0
    total_violations  = 0

    for path in sorted(files):
        cid = os.path.basename(path).replace("_boundary.rs", "")
        if target and cid != target: continue
        errs, btype = check_file(path, cid, registry, valid_groups)
        if errs:
            print(f"\n[ERRO] {cid.upper()} (type:{btype})")
            for e in errs: print(f"   {e}")
            failed_components += 1
            total_violations  += len(errs)
        else:
            total_ok += 1
            if target: print(f"\n[OK] {cid.upper()} (type:{btype}) — clean")

    print(f"\n{'='*50}")
    print(f"[OK] {total_ok} boundaries clean")
    if total_violations:
        print(f"[FAIL] {failed_components} components failed — {total_violations} violations found")
        return 1
    print("[OK] All boundaries canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run(sys.argv[1] if len(sys.argv) > 1 else None))