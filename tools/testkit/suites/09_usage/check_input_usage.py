#!/usr/bin/env python3
import re, glob, os, sys

PRODUCTS_DIR = "/opt/docker/monorepo/products"

ALLOW_NATIVE_INPUT = [
    # canvas editor — input nativo justificado por rendering especializado
    "canvas-editor-frontend-leptos/src/pages/canvas_editor/left_panel.rs",
    "canvas-editor-frontend-leptos/src/pages/canvas_editor/inspector.rs",
    "canvas-editor-frontend-leptos/src/pages/project_detail.rs",
    "canvas-editor-frontend-leptos/src/pages/projects.rs",
    # canonrs-builder — builder interno usa input nativo por design
    "canonrs-builder/",
    # canonrs-site — site de documentacao, nao produto
    "canonrs-site/",
    # core-ai-assistant — chat usa input nativo por UX especifica
    "core-ai-assistant/",
    # backups e arquivos antigos — ignorar
    ".backup/",
    "_old-workbench/",
    "old1/",
    "frontend-leptos.backup/",
]

def strip_comments(src):
    return re.sub(r"//[^\n]*", "", src)

def allowed(path):
    return any(a in path for a in ALLOW_NATIVE_INPUT)

def check_file(path):
    errors = []
    src = strip_comments(open(path).read())
    cid = os.path.relpath(path, PRODUCTS_DIR)

    if allowed(path):
        return errors

    if re.search(r"<input\b", src):
        errors.append(
            f"[CR-USG-INP-100] {cid} — uso direto de <input> nativo proibido\n"
            f"                  usar <Input> CanonRS quando houver componente equivalente"
        )

    if re.search(r"<Input[^>]*node_ref=.*map\(Some\)", src, re.DOTALL):
        errors.append(
            f"[CR-USG-INP-101] {cid} — node_ref.map(Some) proibido no uso de Input\n"
            f"                  passar node_ref diretamente"
        )

    if re.search(r"<input[^>]*type=\"hidden\"", src):
        errors.append(
            f"[CR-USG-INP-102] {cid} — hidden input manual proibido para sincronizar componente CanonRS\n"
            f"                  sincronizacao deve ser feita pelo runtime do componente"
        )

    return errors

def run():
    files = glob.glob(f"{PRODUCTS_DIR}/**/*.rs", recursive=True)
    total_ok = failed = total_err = 0

    for path in sorted(files):
        errs = check_file(path)
        if errs:
            print(f"\n[ERRO] {os.path.relpath(path, PRODUCTS_DIR)}")
            for e in errs:
                print(f"   {e}")
            failed += 1
            total_err += len(errs)
        else:
            total_ok += 1

    print("\n" + "=" * 50)
    print(f"[OK] {total_ok} files clean")
    if total_err:
        print(f"[FAIL] {failed} files — {total_err} usage violations")
        return 1
    print("[OK] Input usage canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run())
