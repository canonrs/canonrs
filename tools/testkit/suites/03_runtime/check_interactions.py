#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""
check_interactions.py — Interaction Engine governance
Critérios: CANONRS INTERACTION ENGINE (1 pt)
Padrao de referencia: canonrs-interactions-overlay
"""
import re, glob, os, sys

INT_BASE = _CANONRS_ROOT + ""
INT_GROUPS = [
    "canonrs-interactions-nav",
    "canonrs-interactions-overlay",
    "canonrs-interactions-init",
    "canonrs-interactions-selection",
    "canonrs-interactions-gesture",
    "canonrs-interactions-content",
    "canonrs-interactions-data",
]

CORE_MODULES = ["state", "query", "lifecycle", "attrs", "aria", "form"]

# tokens com funcoes tipadas no core — proibido usar como string literal em state::add/remove
# open -> state::open(), closed -> state::close()
# expanded -> state::expand(), collapsed -> state::collapse()
# outros (active, selected, disabled, loading) -> state::add(&str) e permitido
CANONICAL_TOKENS_WITH_TYPED_FN = [
    "open", "closed", "expanded", "collapsed",
]

# runtime modules locais permitidos por grupo — nao precisam de canonrs_interactions_core
LOCAL_RUNTIME = {
    "canonrs-interactions-gesture": ["drag", "context", "uid"],
    "canonrs-interactions-init":    ["interactive", "dismiss", "observer", "registry", "selection", "focus", "keyboard"],
    "canonrs-interactions-overlay": ["stack", "portal", "inert", "focus", "transition", "positioning", "events"],
    "canonrs-interactions-data":    ["context", "nav"],
    "canonrs-interactions-nav":     [],
    "canonrs-interactions-selection": [],
    "canonrs-interactions-content": [],
}


def is_runtime_file(path):
    return "/runtime/" in path or "\\runtime\\" in path


def check_file(path, cid, group):
    errors = []
    src = open(path).read()
    src_no_comments = re.sub(r"//[^\n]*", "", src)
    runtime = is_runtime_file(path)

    # CR-IE-100: doc comment //! obrigatorio — exceto runtime modules
    if not runtime:
        has_doc = bool(re.search(r"^//!", src, re.MULTILINE))
        if not has_doc:
            errors.append(f"[CR-IE-100] {cid} ({group}) — doc comment //! ausente\n"
                          f"            ex: //! ComponentName Interaction Engine\n"
                          f"            //! Core: dom/{{lifecycle, state, query}}")

    # CR-IE-101: lifecycle::init_guard obrigatorio em fn init() publica — exceto runtime
    if not runtime:
        for m in re.finditer(r"\bpub fn init\b", src):
            body_start = src.find("{", m.start())
            if body_start < 0:
                continue
            body_head = src[body_start:body_start+400]
            has_guard = (
                "lifecycle::init_guard" in body_head or
                "is_initialized" in body_head or
                "init_guard" in body_head
            )
            # uid-based listeners = bootstrap-dispatched, no guard needed
            has_uid_listeners = (
                "use crate::runtime::listeners" in src or
                "use canonrs_interactions_core::runtime::listeners" in src or
                bool(re.search(r"listeners::\w+\s*\(\s*&uid", src))
            )
            # pure state-only init (no listeners) is replay-safe — bootstrap governs
            is_state_only = "listeners::" not in src and "add_event_listener" not in src
            if not has_guard and not has_uid_listeners and not is_state_only:
                errors.append(f"[CR-IE-101] {cid} ({group}) — lifecycle::init_guard ausente em pub fn init()\n"
                              f"            primeira linha: if !lifecycle::init_guard(&root) {{ return; }}")
                break

    # CR-IE-102: importar canonrs_interactions_core quando usa modulos core
    has_core_import = "canonrs_interactions_core" in src
    uses_core = any(f"{mod}::" in src for mod in CORE_MODULES)
    allowed_local = LOCAL_RUNTIME.get(group, [])
    is_local_runtime = any(
        f"runtime/{mod}" in path or mod in cid
        for mod in allowed_local
    )
    if uses_core and not has_core_import and not is_local_runtime and not runtime:
        errors.append(f"[CR-IE-102] {cid} ({group}) — usa modulos core sem importar canonrs_interactions_core\n"
                      f"            use canonrs_interactions_core::dom::{{lifecycle, state, query}}")

    # CR-IE-103: tokens com funcoes tipadas nao devem ser strings literais
    # state::add(el, "open") -> state::open(el)
    # state::add(el, "expanded") -> state::expand(el)
    # state::add(el, "active") -> PERMITIDO (nao tem funcao tipada)
    for token in CANONICAL_TOKENS_WITH_TYPED_FN:
        pattern = rf'(?<!add_)(?<!remove_)state::(set|add|remove|unset)\s*\([^,]+,\s*"{token}"'
        if re.search(pattern, src):
            typed_fn = {"open": "state::open(el)", "closed": "state::close(el)",
                       "expanded": "state::expand(el)", "collapsed": "state::collapse(el)"}
            errors.append(f"[CR-IE-103] {cid} ({group}) — token \"{token}\" via state:: como string literal\n"
                          f"            usar {typed_fn.get(token, 'funcao tipada')} diretamente")

    # CR-IE-104: set_attribute("data-rs-state", literal_fixo) direto proibido
    # permitido: set_attribute com valor dinamico (&var, &format!(...), condicional)
    # proibido: set_attribute("data-rs-state", "open") — usar state::open()
    #           set_attribute("data-rs-state", "closed") — usar state::close()
    if not runtime and "transition" not in cid:
        # detecta apenas string literal fixa — nao dinamica
        for pat in [
            r'set_attribute\s*\(\s*"data-rs-state"\s*,\s*"open"\)',
            r'set_attribute\s*\(\s*"data-rs-state"\s*,\s*"closed"\)',
            r'set_attribute\s*\(\s*"data-rs-state"\s*,\s*"expanded"\)',
            r'set_attribute\s*\(\s*"data-rs-state"\s*,\s*"collapsed"\)',
        ]:
            if re.search(pat, src):
                errors.append(f"[CR-IE-104] {cid} ({group}) — set_attribute(\"data-rs-state\", literal) direto\n"
                              f"            usar state::open/close/expand/collapse do core")
                break

    # CR-IE-105: reimplementacao local de modulos core proibida
    for mod in CORE_MODULES:
        if re.search(rf"^pub fn {mod}\b", src, re.MULTILINE):
            errors.append(f"[CR-IE-105] {cid} ({group}) — reimplementacao local de '{mod}'\n"
                          f"            importar de canonrs_interactions_core")

    # CR-IE-106: keyboard navigation — nao verificado via regex
    # impossivel distinguir implementadores do padrao de quem deveria delegar
    # verificacao manual via code review

    return errors


def run(target=None):
    all_files = []
    for group_dir in INT_GROUPS:
        src_dir = os.path.join(INT_BASE, group_dir, "src")
        if not os.path.isdir(src_dir):
            continue
        for f in glob.glob(f"{src_dir}/**/*.rs", recursive=True):
            basename = os.path.basename(f)
            if basename in ("lib.rs", "mod.rs"):
                continue
            all_files.append((f, group_dir))

    if not all_files:
        print(f"\n[FAIL] 0 files analyzed — base: {INT_BASE}")
        return 1

    total_ok          = 0
    failed_components = 0
    total_violations  = 0

    for path, group in sorted(all_files):
        cid = os.path.basename(path).replace(".rs", "")
        if target and cid != target: continue
        errs = check_file(path, cid, group)
        if errs:
            print(f"\n[ERRO] {cid.upper()} ({group})")
            for e in errs: print(f"   {e}")
            failed_components += 1
            total_violations  += len(errs)
        else:
            total_ok += 1
            if target: print(f"\n[OK] {cid.upper()} ({group}) — clean")

    print(f"\n{'='*50}")
    print(f"[OK] {total_ok} interaction engines clean")
    if total_violations:
        print(f"[FAIL] {failed_components} components failed — {total_violations} violations found")
        return 1
    print("[OK] All interaction engines canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run(sys.argv[1] if len(sys.argv) > 1 else None))