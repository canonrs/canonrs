#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""
check_interactions_core.py — Runtime Architecture Guarantees

Valida GARANTIAS, nao implementacao especifica.
Cada regra responde: "o sistema sobrevive ao lifecycle real?"
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
    "canonrs-interactions-core",
]

def is_runtime_kernel(path):
    """Arquivos do kernel que implementam as APIs oficiais — excluidos de algumas regras."""
    return "canonrs-interactions-core/src/runtime/" in path

# Files to skip — core infrastructure, not interaction components
SKIP_PATHS = [
    "/canonrs-interactions-core/src/behavior/",
    "/canonrs-interactions-core/src/dom/",
    "/canonrs-interactions-core/src/runtime/",
    "/canonrs-interactions-core/src/integration/",
    # Global singleton listeners — intentional cb.forget() (live forever)
    "/src/runtime/stack.rs",
    "/src/runtime/popup.rs",
    # Observer runtime infrastructure — intentional observer_cb.forget()
    "/canonrs-interactions-init/src/runtime/observer.rs",
]

def is_skip(path):
    return any(s in path for s in SKIP_PATHS)

def is_test_or_doc(path):
    return "/tests/" in path or path.endswith("_test.rs")

def strip_comments(src):
    return re.sub(r"//[^\n]*", "", src)

def check_file(path, cid):
    errors = []
    src = open(path).read()
    nc  = strip_comments(src)
    kernel = is_runtime_kernel(path)

    # CR-CORE-100: Closure permanente sem ownership explícito
    # O problema nao e .forget() — e closure permanente sem namespace de ownership
    # Heuristica: .forget() fora do kernel sem listeners:: sugere leak sem cleanup
    if not kernel:
        forget_count   = len(re.findall(r"\.forget\(\)", nc))
        listeners_uses = len(re.findall(r"listeners::", nc))
        timers_uses    = len(re.findall(r"timers::", nc))
        # se usa forget mas nao usa listeners:: nem timers:: — ownership nao governado
        if forget_count > 0 and listeners_uses == 0 and timers_uses == 0:
            errors.append(
                f"[CR-CORE-100] {cid} — {forget_count} closure(s) permanente(s) sem ownership governado\n"
                f"              .forget() sem runtime::listeners/timers sugere leak sem cleanup namespace\n"
                f"              garantia: closures permanentes DEVEM ter namespace de ownership"
            )

    # CR-CORE-101: DOM Element em estado global
    # Garantia: runtime nao pode possuir referencias a elementos DOM
    if re.search(r"HashSet\s*<\s*(?:web_sys::)?Element\s*>", nc):
        errors.append(
            f"[CR-CORE-101] {cid} — HashSet<Element> em estado global\n"
            f"              garantia: runtime nao pode possuir referencias DOM (stale apos re-render)\n"
            f"              usar HashSet<String> com data-rs-uid"
        )
    if re.search(r"thread_local\s*!\s*\{[^}}]*\bElement\b[^}}]*\}", nc, re.DOTALL):
        errors.append(
            f"[CR-CORE-101] {cid} — thread_local com Element\n"
            f"              garantia: estado global nao pode referenciar nos DOM"
        )

    # CR-CORE-102: Init sem idempotencia
    # Garantia: init DEVE ser replay-safe
    if not kernel:
        for m in re.finditer(r"\bpub fn init\b", nc):
            body_start = nc.find("{", m.start())
            if body_start < 0: continue
            body = nc[body_start:body_start+500]
            has_guard = "init_guard" in body or "is_initialized" in body
            # uid-based listeners = bootstrap-dispatched, replay-safe
            has_uid_listeners = (
                "use canonrs_interactions_core::runtime::listeners" in nc or
                "use crate::runtime::listeners" in nc or
                bool(re.search(r"listeners::\w+\s*\(\s*&uid", nc))
            )
            # state-only init (no listeners) is safe
            is_state_only = "listeners::" not in nc and "add_event_listener" not in nc
            if not has_guard and not has_uid_listeners and not is_state_only:
                errors.append(
                    f"[CR-CORE-102] {cid} — pub fn init() sem idempotencia guard\n"
                    f"              garantia: init DEVE ser replay-safe (double-init causa listeners duplicados)"
                )
                break

    # CR-CORE-103: Portal SSR-unsafe
    # Garantia: portais DEVEM renderizar no SSR
    if "leptos::portal::Portal" in nc:
        errors.append(
            f"[CR-CORE-103] {cid} — leptos::portal::Portal detectado\n"
            f"              garantia: portais DEVEM ser SSR-safe (Portal do Leptos renderiza vazio no servidor)"
        )

    # CR-CORE-104: static mut — unsafe global state
    # Garantia: estado global mutavel nao e thread-safe no WASM
    if re.search(r"\bstatic\s+mut\b", nc):
        errors.append(
            f"[CR-CORE-104] {cid} — static mut detectado\n"
            f"              garantia: estado global mutavel e unsafe em WASM single-threaded"
        )

    # CR-CORE-105: manual data-rs-initialized — bypassa lifecycle governance
    # Garantia: lifecycle tracking DEVE ser centralizado
    if re.search(r'set_attribute\s*\(\s*"data-rs-initialized"', nc):
        errors.append(
            f"[CR-CORE-105] {cid} — set_attribute(data-rs-initialized) manual\n"
            f"              garantia: lifecycle tracking DEVE passar por lifecycle::init_guard()\n"
            f"              set manual bypassa replay e reinit governance"
        )

    # CR-CORE-106: listeners::listen sem runtime ownership
    # Garantia: componente que registra listeners DEVE ter runtime ownership governado
    # Ownership pode ser: explicito (cleanup::) ou implicito via runtime central
    # (stack::register_*, drag::start, runtime::listeners namespace, listeners::listen com uid)
    if not kernel:
        uses_listen = "listeners::listen" in nc or "listeners::on_click" in nc or "listeners::on_keydown" in nc
        has_uid_ns  = re.search(r'listeners::\w+\s*\(\s*&uid', nc) or re.search(r'listeners::\w+\s*\(\s*&\w*uid', nc)
        # Runtime ownership: explicito ou implicito via runtime central
        has_runtime_ownership = (
            "listeners::cleanup" in nc
            or "cleanup::" in nc
            or "cleanup_uid" in nc
            or "stack::register_click" in nc
            or "stack::register_keydown" in nc
            or "drag::start" in nc
            or "runtime::listeners" in nc
            or "crate::runtime::listeners" in nc
            or "canonrs_interactions_core::runtime::listeners" in nc
            or "listeners::listen(" in nc
        )
        # so valida se usa listeners com namespace (nao one-shot sem uid)
        if uses_listen and has_uid_ns and not has_runtime_ownership:
            errors.append(
                f"[CR-CORE-106] {cid} — listeners registrados sem runtime ownership\n"
                f"              garantia: componente com listeners DEVE ter ownership governado\n"
                f"              via cleanup::, stack::register_*, drag::start, ou runtime::listeners"
            )

    return errors


def run(target=None):
    all_files = []
    for group in INT_GROUPS:
        src_dir = os.path.join(INT_BASE, group, "src")
        if not os.path.isdir(src_dir): continue
        for f in glob.glob(f"{src_dir}/**/*.rs", recursive=True):
            if not is_test_or_doc(f) and not is_skip(f):
                all_files.append((f, group))

    if not all_files:
        print(f"[FAIL] 0 files found")
        return 1

    total_ok = total_err = failed = 0
    for path, group in sorted(all_files):
        cid = os.path.basename(path).replace(".rs", "")
        if target and cid != target: continue
        errs = check_file(path, cid)
        if errs:
            rel = path.replace(INT_BASE + "/", "")
            print(f"\n[ERRO] {rel}")
            for e in errs: print(f"   {e}")
            failed += 1
            total_err += len(errs)
        else:
            total_ok += 1

    print(f"\n{'='*50}")
    print(f"[OK] {total_ok} files clean")
    if total_err:
        print(f"[FAIL] {failed} files — {total_err} guarantee violations")
        return 1
    print("[OK] Runtime architecture guarantees met")
    return 0

if __name__ == "__main__":
    sys.exit(run(sys.argv[1] if len(sys.argv) > 1 else None))