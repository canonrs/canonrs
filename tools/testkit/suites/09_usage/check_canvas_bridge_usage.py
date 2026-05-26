#!/usr/bin/env python3
import re, glob, os, sys

CANVAS_EDITOR = "/opt/docker/monorepo/products/canvas-editor"

# TODO: mover para runtime_bridge.rs
# inspector.rs: get_node_stroke, update_stroke
# center_panel.rs: export_png, export_webp
KNOWN_VIOLATIONS = [
    "canvas_editor/inspector.rs",
    "canvas_editor/center_panel.rs",
]

def strip_comments(src):
    return re.sub(r"//[^\n]*", "", src)

def check_file(path):
    errors = []
    src = strip_comments(open(path).read())
    cid = os.path.relpath(path, CANVAS_EDITOR)

    # CR-USG-CAN-100/101: overlay com style inline deve ter position:fixed + isolation:isolate
    # Apenas quando ActionOverlay/ContextMenuOverlay tem style= na mesma linha do componente
    import re as _re
    for line in open(path).readlines():
        line_nc = _re.sub(r"//.*", "", line)
        if ("ActionOverlay" in line_nc or "ContextMenuOverlay" in line_nc) and 'style="' in line_nc:
            if "position:fixed" not in line_nc:
                errors.append(f"[CR-USG-CAN-100] {cid} — overlay de canvas com style inline deve usar position:fixed")
            if "isolation:isolate" not in line_nc:
                errors.append(f"[CR-USG-CAN-101] {cid} — overlay de canvas com style inline deve usar isolation:isolate")

    # CR-USG-CAN-102: rs_canvas_runtime direto proibido exceto em lib.rs (bootstrap) e runtime_bridge.rs
    if "rs_canvas_runtime::" in src:
        is_lib      = path.endswith("/src/lib.rs")
        is_bridge   = "runtime_bridge.rs" in path
        is_known    = any(kv in path for kv in KNOWN_VIOLATIONS)
        # lib.rs pode chamar register() no bootstrap
        is_register_only = is_lib and all(
            call.strip().startswith("register")
            for call in src.split("rs_canvas_runtime::")[1:]
            if call.strip()
        )
        if not is_bridge and not is_register_only and not is_known:
            errors.append(
                f"[CR-USG-CAN-102] {cid} — frontend nao pode chamar rs_canvas_runtime direto\n"
                f"                  usar runtime_bridge.rs (excecao: lib.rs para register())"
            )

    # CR-USG-CAN-103: listeners do canvas devem ficar em canvas_events.rs
    if "add_event_listener" in src and "canvas_events.rs" not in path:
        errors.append(f"[CR-USG-CAN-103] {cid} — listeners do canvas devem ficar em canvas_events.rs")

    return errors

def run():
    files = glob.glob(f"{CANVAS_EDITOR}/**/*.rs", recursive=True)
    total_ok = failed = total_err = 0

    for path in sorted(files):
        errs = check_file(path)
        if errs:
            print(f"\n[ERRO] {os.path.relpath(path, CANVAS_EDITOR)}")
            for e in errs:
                print(f"   {e}")
            failed += 1
            total_err += len(errs)
        else:
            total_ok += 1

    print("\n" + "=" * 50)
    print(f"[OK] {total_ok} files clean")
    if total_err:
        print(f"[FAIL] {failed} files — {total_err} canvas bridge violations")
        return 1
    print("[OK] Canvas bridge usage canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run())
