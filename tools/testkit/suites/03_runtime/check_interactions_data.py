#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""
check_interactions_data.py — Data Runtime Guarantees

Valida GARANTIAS arquiteturais do grupo data.
Nao valida implementacao especifica — valida contratos de comportamento.
"""
import re, glob, os, sys

DATA_DIR = _CANONRS_ROOT + "/canonrs-interactions-data/src"

def strip_comments(src):
    return re.sub(r"//[^\n]*", "", src)

def is_runtime(path):
    return "/runtime/" in path

def check_file(path, cid):
    errors = []
    src = open(path).read()
    nc  = strip_comments(src)
    if is_runtime(path): return errors
    if cid in ("lib", "mod"): return errors

    # CR-DATA-100: cb.forget() sem runtime ownership
    forget_count   = len(re.findall(r"\.forget\(\)", nc))
    listeners_uses = len(re.findall(r"listeners::", nc))
    timers_uses    = len(re.findall(r"timers::", nc))
    if forget_count > 0 and listeners_uses == 0 and timers_uses == 0:
        errors.append(
            f"[CR-DATA-100] {cid} — {forget_count} closure(s) permanente(s) sem ownership governado\n"
            f"              .forget() sem runtime::listeners/timers sugere leak\n"
            f"              garantia: closures permanentes DEVEM ter namespace de ownership"
        )

    # CR-DATA-101: Replay safety
    if "pub fn init" in nc:
        m = re.search(r"\bpub fn init\b", nc)
        if m:
            body = nc[nc.find("{", m.start()):nc.find("{", m.start())+500]
            has_local_guard = "init_guard" in body or "is_initialized" in body
            has_uid_listeners = bool(
                re.search(r"listeners::\w+\s*\(\s*&\w*uid", nc) or
                re.search(r"listeners::listen_uid", nc) or
                re.search(r"drag::start\s*\(\s*&\w*uid", nc) or
                (bool(re.search(r'get_attribute.*data-rs-uid', nc)) and "listeners::" in nc) or
                "use crate::runtime::listeners" in nc
            )
            if not has_local_guard and not has_uid_listeners:
                errors.append(
                    f"[CR-DATA-101] {cid} — init() sem replay safety\n"
                    f"              garantia: init DEVE ser idempotente"
                )

    # CR-DATA-102: window/document listeners sem namespace
    has_global_raw = (
        "add_event_listener_with_callback" in nc and
        ("window()" in nc or "document()" in nc) and
        "listeners::" not in nc
    )
    if has_global_raw:
        errors.append(
            f"[CR-DATA-102] {cid} — listener global sem runtime namespace\n"
            f"              garantia: DEVE usar listeners::listen_window/listen_document"
        )

    # CR-DATA-103: drag lifecycle sem runtime drag
    has_window_raw = "add_event_listener_with_callback" in nc and "window()" in nc
    has_drag_start = "drag::start" in nc
    if has_window_raw and not has_drag_start and listeners_uses == 0:
        errors.append(
            f"[CR-DATA-103] {cid} — drag lifecycle sem drag::start\n"
            f"              garantia: mousemove/mouseup window DEVEM usar drag::start"
        )

    # CR-DATA-104: Runtime ownership
    uses_listen_uid = bool(
        re.search(r"listeners::\w+\s*\(\s*&\w*uid", nc) or
        re.search(r"listeners::listen_uid", nc)
    )
    has_runtime_ownership = (
        "listeners::cleanup" in nc or "cleanup::" in nc or "cleanup_uid" in nc
        or "stack::register_click" in nc or "stack::register_keydown" in nc
        or "drag::start" in nc
        or "crate::runtime::listeners" in nc
        or uses_listen_uid
    )
    if listeners_uses > 0 and not has_runtime_ownership:
        errors.append(
            f"[CR-DATA-104] {cid} — listeners sem runtime ownership\n"
            f"              garantia: DEVE usar listeners::listen(&uid, ...) ou drag::start"
        )

    return errors


def score_file(path, cid):
    src = open(path).read()
    nc  = strip_comments(src)
    forget_count   = len(re.findall(r"\.forget\(\)", nc))
    listeners_uses = len(re.findall(r"listeners::", nc))
    timers_uses    = len(re.findall(r"timers::", nc))
    checks = {
        "No raw forget":        forget_count == 0 or listeners_uses > 0 or timers_uses > 0,
        "Replay safe":          "init_guard" in nc or "is_initialized" in nc or "pub fn init" not in nc or "use crate::runtime::listeners" in nc or bool(re.search(r'get_attribute.*data-rs-uid', nc)),
        "No raw global listen": not (
            "add_event_listener_with_callback" in nc and
            ("window()" in nc or "document()" in nc) and
            "listeners::" not in nc
        ),
        "Drag governed":        not (
            "add_event_listener_with_callback" in nc and
            "window()" in nc and
            "drag::start" not in nc and
            listeners_uses == 0
        ),
        "Runtime ownership":    not (listeners_uses > 0 and not (
            "listeners::cleanup" in nc or "cleanup::" in nc or
            "drag::start" in nc or "crate::runtime::listeners" in nc or
            bool(re.search(r"listeners::\w+\s*\(\s*&\w*uid", nc)) or
            bool(re.search(r"listeners::listen_uid", nc))
        )),
    }
    passed = sum(1 for v in checks.values() if v)
    return checks, passed, len(checks)


def run(target=None):
    files = [f for f in glob.glob(f"{DATA_DIR}/**/*.rs", recursive=True)
             if "/runtime/" not in f and ".bak" not in f and ".old" not in f]
    if not files:
        print("[FAIL] 0 files found")
        return 1
    total_ok = total_err = failed = 0
    score_passed = score_total = 0
    for path in sorted(files):
        cid = os.path.basename(path).replace(".rs", "")
        if target and cid != target: continue
        if cid in ("lib", "mod"): continue
        errs = check_file(path, cid)
        checks, passed, total = score_file(path, cid)
        score_passed += passed
        score_total  += total
        if errs:
            print(f"\n[ERRO] {os.path.relpath(path, DATA_DIR)}")
            for e in errs: print(f"   {e}")
            failed += 1
            total_err += len(errs)
        else:
            total_ok += 1
            if target:
                print(f"\n[OK] {cid.upper()} — {passed}/{total}")
                for k, v in checks.items():
                    print(f"   {chr(9989) if v else chr(10060)} {k}")
    pct = int(score_passed / score_total * 100) if score_total else 0
    print(f"\n" + "=" * 50)
    print(f"[OK] {total_ok} modules clean")
    print(f"Runtime Score: {score_passed}/{score_total} ({pct}%)")
    if total_err:
        print(f"[FAIL] {failed} modules — {total_err} guarantee violations")
        return 1
    print("[OK] Data runtime guarantees met")
    return 0


if __name__ == "__main__":
    sys.exit(run(sys.argv[1] if len(sys.argv) > 1 else None))