#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""
check_interactions_overlay.py — Overlay Runtime Guarantees

Valida GARANTIAS arquiteturais do grupo overlay.
Nao valida implementacao especifica — valida contratos de comportamento.
"""
import re, glob, os, sys

OVERLAY_DIR = _CANONRS_ROOT + "/canonrs-interactions-overlay/src"

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

    # CR-OVL-100: SSR safety — Portal do Leptos proibido
    # Garantia: overlay DEVE renderizar no SSR
    if "leptos::portal::Portal" in nc:
        errors.append(
            f"[CR-OVL-100] {cid} — leptos::portal::Portal detectado\n"
            f"              garantia: overlay DEVE ser SSR-safe"
        )

    # CR-OVL-101: Replay safety — init sem idempotencia
    # Garantia: init DEVE sobreviver a remount sem duplicar listeners
    if "pub fn init" in nc:
        m = re.search(r"\bpub fn init\b", nc)
        if m:
            body = nc[nc.find("{", m.start()):nc.find("{", m.start())+500]
            has_uid_listeners = (
                "use canonrs_interactions_core::runtime::listeners" in nc or
                "use crate::runtime::listeners" in nc or
                bool(__import__("re").search(r"listeners::\w+\s*\(\s*&uid", nc))
            )
            is_state_only = "listeners::" not in nc and "add_event_listener" not in nc
            if "init_guard" not in body and "is_initialized" not in body and not has_uid_listeners and not is_state_only:
                errors.append(
                    f"[CR-OVL-101] {cid} — init() sem replay safety\n"
                    f"              garantia: init DEVE ser idempotente (remount duplica listeners)"
                )

    # CR-OVL-102: Closure permanente sem ownership
    # Garantia: listeners permanentes DEVEM ter cleanup path
    forget_count   = len(re.findall(r"\.forget\(\)", nc))
    listeners_uses = len(re.findall(r"listeners::", nc))
    timers_uses    = len(re.findall(r"timers::", nc))
    if forget_count > 0 and listeners_uses == 0 and timers_uses == 0:
        errors.append(
            f"[CR-OVL-102] {cid} — {forget_count} closure(s) sem ownership namespace\n"
            f"              garantia: listeners permanentes DEVEM ter cleanup path"
        )

    # CR-OVL-103: Scroll lock sem release
    # Garantia: se abre scroll lock, DEVE fechar
    opens_lock  = "set_scroll_lock(true)" in nc
    # stack::pop libera scroll_lock quando stack vazio — release valido
    closes_lock = "set_scroll_lock(false)" in nc or "stack::pop" in nc
    if opens_lock and not closes_lock:
        errors.append(
            f"[CR-OVL-103] {cid} — scroll lock aberto sem release\n"
            f"              garantia: scroll lock DEVE ser liberado no close"
        )

    # CR-OVL-104: Overlay com portal DEVE mover para body
    # Garantia: portal DEVE estar no body para z-index correto
    has_portal_attr = re.search(r'data-rs-\w+-portal', nc)
    has_move_to_body = "move_to_body" in nc or "append_child" in nc
    if has_portal_attr and not has_move_to_body:
        errors.append(
            f"[CR-OVL-104] {cid} — portal declarado sem move_to_body\n"
            f"              garantia: portal DEVE ser movido para body (z-index e stacking context)"
        )

    return errors


def score_file(path, cid):
    src = open(path).read()
    nc  = strip_comments(src)

    checks = {
        "SSR safe":         "leptos::portal::Portal" not in nc,
        "Replay safe":      "init_guard" in nc or "is_initialized" in nc or "pub fn init" not in nc or "listeners::" in nc,
        "Ownership safe":   len(re.findall(r"\.forget\(\)", nc)) == 0 or "listeners::" in nc or "timers::" in nc,
        "Scroll balanced":  not ("set_scroll_lock(true)" in nc and "set_scroll_lock(false)" not in nc and "stack::pop" not in nc),
        "Portal safe":      not (re.search(r'data-rs-\w+-portal', nc) and "move_to_body" not in nc and "append_child" not in nc),
    }
    passed = sum(1 for v in checks.values() if v)
    return checks, passed, len(checks)


def run(target=None):
    files = [f for f in glob.glob(f"{OVERLAY_DIR}/**/*.rs", recursive=True)
             if "/runtime/" not in f]

    if not files:
        print(f"[FAIL] 0 files found")
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
            print(f"\n[ERRO] {cid.upper()}")
            for e in errs: print(f"   {e}")
            failed += 1
            total_err += len(errs)
        else:
            total_ok += 1
            if target:
                print(f"\n[OK] {cid.upper()} — {passed}/{total}")
                for k, v in checks.items():
                    print(f"   {'✅' if v else '❌'} {k}")

    pct = int(score_passed / score_total * 100) if score_total else 0
    print(f"\n{'='*50}")
    print(f"[OK] {total_ok} modules clean")
    print(f"Runtime Score: {score_passed}/{score_total} ({pct}%)")
    if total_err:
        print(f"[FAIL] {failed} modules — {total_err} guarantee violations")
        return 1
    print("[OK] Overlay runtime guarantees met")
    return 0

if __name__ == "__main__":
    sys.exit(run(sys.argv[1] if len(sys.argv) > 1 else None))