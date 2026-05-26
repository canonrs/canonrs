#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""
check_runtime.py — Interaction Engine runtime safety governance

Valida:
1. init_guard DOM-driven (data-rs-initialized) — nao HashSet global
2. Sem global DOM registry (HashSet<Element>)
3. Sem thread_local com Element — stale references
4. Listeners adicionados no document sao cleanup-safe
5. Closures com forget() tem justificativa
6. Sem duplicate listener patterns
"""
import re, glob, os, sys

INT_BASE  = _CANONRS_ROOT + ""
INT_GROUPS = [
    "canonrs-interactions-nav",
    "canonrs-interactions-overlay",
    "canonrs-interactions-init",
    "canonrs-interactions-selection",
    "canonrs-interactions-gesture",
    "canonrs-interactions-content",
    "canonrs-interactions-data",
]


def check_file(path, cid, group):
    errors = []
    src = open(path).read()
    src_no_comments = re.sub(r"//[^\n]*", "", src)
    is_runtime = "/runtime/" in path

    # CR-RT-100: HashSet<Element> proibido — stale references, nao GC-safe
    if re.search(r"HashSet\s*<\s*(web_sys::)?Element\s*>", src_no_comments):
        errors.append(
            f"[CR-RT-100] {cid} ({group}) — HashSet<Element> proibido\n"
            f"            Element referencias ficam stale apos re-render\n"
            f"            usar HashSet<String> com data-rs-uid"
        )

    # CR-RT-101: thread_local com Element proibido
    if re.search(r"thread_local\s*!\s*\{[^}]*Element[^}]*\}", src_no_comments, re.DOTALL):
        errors.append(
            f"[CR-RT-101] {cid} ({group}) — thread_local com Element proibido\n"
            f"            usar thread_local com String (uid) ou DOM attrs como state"
        )

    # CR-RT-102: Vec<Element> global proibido
    if re.search(r"(?:static|thread_local)[^;]*Vec\s*<[^>]*Element", src_no_comments):
        errors.append(
            f"[CR-RT-102] {cid} ({group}) — Vec<Element> global proibido\n"
            f"            usar DOM attributes como source of truth"
        )

    # CR-RT-103: add_event_listener sem forget() ou stored closure — memory leak
    listener_count = len(re.findall(r"add_event_listener_with_callback", src_no_comments))
    forget_count   = len(re.findall(r"\.forget\(\)", src_no_comments))
    if listener_count > 0 and forget_count == 0 and not is_runtime:
        errors.append(
            f"[CR-RT-103] {cid} ({group}) — {listener_count} listeners sem .forget()\n"
            f"            Closure deve ter .forget() ou ser armazenada"
        )

    # CR-RT-104: document.add_event_listener em fn init() — pode duplicar em re-init
    if not is_runtime:
        doc_listeners = re.findall(
            r"doc(?:ument)?\.add_event_listener_with_callback\s*\(",
            src_no_comments
        )
        if doc_listeners and "lifecycle::init_guard" not in src_no_comments:
            errors.append(
                f"[CR-RT-104] {cid} ({group}) — {len(doc_listeners)} document listeners sem init_guard\n"
                f"            listeners globais duplicam em re-init sem guard"
            )

    # CR-RT-105: Rc<RefCell> em closures — nao replay-safe
    if re.search(r"Rc\s*<\s*RefCell\s*<", src_no_comments) and not is_runtime:
        errors.append(
            f"[CR-RT-105] {cid} ({group}) — Rc<RefCell> em closure de interacao\n"
            f"            usar DOM attrs como state — Rc nao sobrevive a re-render"
        )

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
        print(f"[FAIL] 0 files found — base: {INT_BASE}")
        return 1

    total_ok = total_err = failed = 0
    for path, group in sorted(all_files):
        cid = os.path.basename(path).replace(".rs", "")
        if target and cid != target: continue
        errs = check_file(path, cid, group)
        if errs:
            print(f"\n[ERRO] {cid.upper()} ({group})")
            for e in errs: print(f"   {e}")
            failed += 1
            total_err += len(errs)
        else:
            total_ok += 1
            if target: print(f"\n[OK] {cid.upper()} — clean")

    print(f"\n{'='*50}")
    print(f"[OK] {total_ok} files clean")
    if total_err:
        print(f"[FAIL] {failed} components failed — {total_err} violations found")
        return 1
    print("[OK] Runtime architecture canonical")
    return 0


if __name__ == "__main__":
    sys.exit(run(sys.argv[1] if len(sys.argv) > 1 else None))