#!/usr/bin/env python3
"""
check_canvas_graph_contracts.py — CR-GRF-100..104
CR-GRF-102: hit_test deve usar geometry layer
CR-GRF-104: Node render proibido mutar engine state
"""
import re, glob, os, sys

CANVAS_DIR = "/opt/docker/monorepo/products/canvas-editor"

GEOMETRY_TERMS = ["geometry", "geom", "spatial", "bounds", "layout", "rect", "bbox"]

def strip_comments(src):
    return re.sub(r"//[^\n]*", "", src)

def extract_fn_body(nc, fn_name):
    """Extrai o corpo de uma funcao pelo nome."""
    m = re.search(rf"fn {fn_name}\b[^{{]*{{", nc)
    if not m:
        return ""
    start = m.end() - 1
    depth = 0
    end = start
    for i, ch in enumerate(nc[start:], start):
        if ch == "{": depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                end = i
                break
    return nc[start:end]

def extract_all_fn_bodies(nc, fn_prefix):
    """Extrai corpos de todas as funcoes que comecam com fn_prefix."""
    bodies = []
    for m in re.finditer(rf"(?:pub\s+)?(?:async\s+)?fn ({fn_prefix}\w*)\s*\(", nc):
        fn_start = nc.find("{", m.end())
        if fn_start == -1:
            continue
        depth = 0
        end = fn_start
        for i, ch in enumerate(nc[fn_start:], fn_start):
            if ch == "{": depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    end = i
                    break
        bodies.append((m.group(1), nc[fn_start:end]))
    return bodies

def check_file(path):
    errors = []
    src = open(path).read()
    nc  = strip_comments(src)
    cid = os.path.relpath(path, CANVAS_DIR)

    # CR-GRF-102: hit_test sem geometry layer
    if "hit_test" in nc:
        body = extract_fn_body(nc, "hit_test")
        if body and not any(term in body for term in GEOMETRY_TERMS):
            errors.append(
                f"[CR-GRF-102] {cid} — hit_test sem geometry layer\n"
                f"             usar geometry/bounds/rect para calculo espacial"
            )

    # CR-GRF-104: render mutando engine state — verifica TODAS as fn render*
    for fn_name, body in extract_all_fn_bodies(nc, "render"):
        if re.search(r"engine\.(?:set|update|push|insert|remove)\b", body):
            errors.append(
                f"[CR-GRF-104] {cid} — fn {fn_name} muta engine state\n"
                f"             render deve ser puro — sem mutacao de engine"
            )

    return errors

def run():
    if not os.path.exists(CANVAS_DIR):
        print("[SKIP] canvas-editor nao encontrado")
        return 0

    files = glob.glob(f"{CANVAS_DIR}/**/*.rs", recursive=True)
    total_ok = failed = total_err = 0
    for path in sorted(files):
        errs = check_file(path)
        if errs:
            print(f"\n[ERRO] {os.path.relpath(path, CANVAS_DIR)}")
            for e in errs: print(f"   {e}")
            failed += 1; total_err += len(errs)
        else:
            total_ok += 1
    print(f"\n{chr(61)*50}")
    print(f"[OK] {total_ok} files clean")
    if total_err:
        print(f"[FAIL] {failed} files — {total_err} violations")
        return 1
    print("[OK] Canvas graph contracts canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run())
