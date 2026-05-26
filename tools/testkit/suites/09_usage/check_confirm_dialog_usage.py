#!/usr/bin/env python3
"""
check_confirm_dialog_usage.py — CR-HYD-105
Valida que ConfirmDialogPortal nao esta dentro de regioes reativas
que causam hydration mismatch no SSR.

Regra CR-HYD-105:
  ConfirmDialogPortal nao pode estar dentro de:
  - For
  - Suspense
  - slot!
  - closure reativa {move || ...}

Regra CR-HYD-106:
  ConfirmDialog deve ser declarado DEPOIS do conteudo principal
  no view! do Page — nao antes.
"""
import re, glob, os, sys

PRODUCTS_DIR = "/opt/docker/monorepo/products"

def strip_comments(src):
    return re.sub(r"//[^\n]*", "", src)

def find_pages(base):
    return glob.glob(f"{base}/**/pages/*.rs", recursive=True)

def check_file(path):
    errors = []
    src = open(path).read()
    nc  = strip_comments(src)

    if "ConfirmDialogPortal" not in nc:
        return errors

    cid = os.path.basename(path).replace(".rs", "")

    # CR-HYD-105: ConfirmDialogPortal dentro de slot!/For/Suspense/move ||
    forbidden_contexts = [
        (r"slot!\s*\(.*?ConfirmDialogPortal", "slot!"),
        (r"For\s*\{[^}]*ConfirmDialogPortal", "For"),
        (r"Suspense\s*[^{]*\{[^}]*ConfirmDialogPortal", "Suspense"),
        (r"move\s*\|\|[^{]*\{[^}]*ConfirmDialogPortal", "closure move ||"),
    ]
    for pattern, ctx in forbidden_contexts:
        if re.search(pattern, nc, re.DOTALL):
            errors.append(
                f"[CR-HYD-105] {cid} — ConfirmDialogPortal dentro de {ctx}\n"
                f"             garantia: ConfirmDialog DEVE ser declarado fora de regioes reativas"
            )

    # CR-HYD-106: ConfirmDialog deve vir DEPOIS do conteudo principal no view!
    # Detecta apenas dentro do view! do componente Page (ultimo view! do arquivo)
    # Ignora definicoes de funcao/componente
    page_view = re.findall(r"pub fn \w+Page[^{]*\{[\s\S]*?view!\s*\{([\s\S]*?)\}\s*\}", nc)
    for block in page_view:
        confirm_pos = block.find("<ConfirmDelete")
        list_pos    = [m.start() for m in re.finditer(r"<[A-Z][a-zA-Z]*List|<ProjectsList|<SourcesList", block)]
        if confirm_pos != -1 and list_pos and confirm_pos < min(list_pos):
            errors.append(
                f"[CR-HYD-106] {cid} — ConfirmDeleteProject declarado ANTES da lista\n"
                f"             garantia: ConfirmDialog DEVE vir depois do conteudo principal\n"
                f"             causa: SSR desalinha hidratacao quando dialog precede lista reativa"
            )

    return errors

def run(target=None):
    files = find_pages(PRODUCTS_DIR)
    if not files:
        print(f"[SKIP] nenhuma pagina encontrada em {PRODUCTS_DIR}")
        return 0

    total_ok = total_err = failed = 0

    for path in sorted(files):
        cid = os.path.basename(path).replace(".rs", "")
        if target and cid != target:
            continue
        errs = check_file(path)
        if errs:
            print(f"\n[ERRO] {cid.upper()}")
            for e in errs:
                print(f"   {e}")
            failed  += 1
            total_err += len(errs)
        else:
            total_ok += 1
            if target:
                print(f"\n[OK] {cid.upper()} — clean")

    print(f"\n{'='*50}")
    print(f"[OK] {total_ok} pages clean")
    if total_err:
        print(f"[FAIL] {failed} pages — {total_err} violations found")
        return 1
    print("[OK] ConfirmDialog usage canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run(sys.argv[1] if len(sys.argv) > 1 else None))
