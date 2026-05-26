#!/usr/bin/env python3
import re, glob, os, sys, yaml
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')


UI_DIR = _CANONRS_ROOT + "/canonrs-server/src/ui"

SIGNAL_PATTERNS = ["create_signal", "signal(", "RwSignal::new", "create_rw_signal", "create_memo"]
HTML_RE = re.compile(r"<(div|span|button|input|textarea|select|form|ul|ol|li|table|tr|td|th|nav|header|footer|main|section|article|aside|h[1-6]|p|img)\b")


def load_registry():
    registry = {}
    for path in glob.glob(f"{UI_DIR}/**/builder.yaml", recursive=True):
        try:
            with open(path) as f:
                data = yaml.safe_load(f)
            if data and "id" in data:
                registry[data["id"]] = data
        except Exception:
            pass
    return registry


def strip_comments_and_strings(src):
    src = re.sub(r"//[^\n]*", "", src)
    src = re.sub(r'"[^"\\]*(?:\\.[^"\\]*)*"', '""', src)
    return src


def remove_preview_fns(src):
    return re.sub(r"#\[component\]\s*\npub fn \w+Preview\s*\([^)]*\)[^{]*\{[^}]*\}", "", src, flags=re.DOTALL)


def is_dynamic_renderer(src):
    """Rendering dinamico legitimo via inner_html."""
    return "inner_html" in src


def is_composite_exception(cid, registry):
    """Componente composto com rendering estrutural proprio — excecao para CR-360/CR-365."""
    entry = registry.get(cid, {})
    composable = entry.get("composable", False)
    btype      = entry.get("boundary_type", "") or ""
    return composable and btype == "interaction"


def extract_view_blocks(src_clean):
    blocks = []
    for vm in re.finditer(r"view!\s*\{", src_clean):
        start = vm.start()
        depth = 0
        end = start
        for i, ch in enumerate(src_clean[start:], start):
            if ch == "{": depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    end = i
                    break
        blocks.append(src_clean[start:end])
    return blocks


def check_file(path, cid, registry):
    errors = []
    src = open(path).read()
    src_no_preview  = remove_preview_fns(src)
    src_clean       = strip_comments_and_strings(src_no_preview)
    src_no_comments = re.sub(r"//[^\n]*", "", src)
    is_exc          = is_dynamic_renderer(src) or is_composite_exception(cid, registry)

    # CR-367: fn *Preview() dentro de _ui.rs = violacao sempre
    if re.search(r"pub fn \w+Preview\s*\(", src):
        errors.append(f"[CR-367] {cid}_ui — fn *Preview() dentro de _ui.rs\n"
                      f"         remover — preview pertence a preview.rs usando apenas Boundary")

    # CR-360: HTML direto proibido
    if not is_exc:
        tags = HTML_RE.findall(src_clean)
        if tags:
            errors.append(f"[CR-360] {cid}_ui — HTML direto proibido: <{set(tags).pop()}>\n"
                          f"         UI deve ser 100% proxy do Primitive\n"
                          f"         criar Primitive correspondente se necessario")

    # CR-361: signals proibidos
    for sig in SIGNAL_PATTERNS:
        if sig in src_no_comments:
            errors.append(f"[CR-361] {cid}_ui — signal proibido: {sig}\n"
                          f"         UI nao cria estado reativo")
            break

    # CR-362: provide_context proibido
    if "provide_context" in src_no_comments:
        errors.append(f"[CR-362] {cid}_ui — provide_context proibido")

    # CR-363: use_navigate proibido
    if "use_navigate" in src_no_comments:
        errors.append(f"[CR-363] {cid}_ui — use_navigate proibido")

    # CR-364: deve importar Primitive de canonrs_core
    if not is_exc and "canonrs_core" not in src:
        errors.append(f"[CR-364] {cid}_ui — nao importa canonrs_core\n"
                      f"         UI DEVE ser proxy de canonrs_core::primitives")

    # CR-365: unwrap_or_default() no view! so permitido como prop passthrough
    if not is_exc:
        for view_block in extract_view_blocks(src_clean):
            for m in re.finditer(r"unwrap_or_default\(\)", view_block):
                pre      = view_block[:m.start()].rfind("\n")
                line_ctx = view_block[pre+1:view_block.find("\n", m.start())].strip()
                is_prop  = bool(re.search(r"\w+\s*=\s*[\w.()\[\]]+\.unwrap_or_default\(\)", line_ctx))
                if not is_prop:
                    errors.append(f"[CR-365] {cid}_ui — unwrap_or_default() no view! fora de prop passthrough\n"
                                  f"         calcular antes do view!")
                    break

    # CR-368: node_ref em UI/Boundary nao pode usar map(Some)
    # unwrap_or_default() e CORRETO no UI — Primitive espera NodeRef<T> nao Option<NodeRef<T>>
    if re.search(r"node_ref", src_no_comments):
        if re.search(r"node_ref.*map\(Some\)", src_no_comments):
            errors.append(
                f"[CR-368] {cid}_ui — node_ref.map(Some) proibido em UI/Boundary\n"
                f"         usar: node_ref=node_ref.unwrap_or_default()"
            )

    # CR-366: sub-components do UI devem usar sub-primitives — via registry
    if not is_exc:
        entry          = registry.get(cid, {})
        required_parts = entry.get("required_parts", []) or []
        if required_parts:
            primitive_imports = re.findall(r"use canonrs_core::primitives::\{([^}]+)\}", src)
            if not primitive_imports:
                errors.append(f"[CR-366] {cid}_ui — tem {len(required_parts)} sub-components mas sem imports de sub-primitives\n"
                              f"         parts: {required_parts}")

    return errors


def run(target=None):
    files = glob.glob(f"{UI_DIR}/**/*_ui.rs", recursive=True)
    if not files:
        print(f"\n[FAIL] 0 files analyzed — path: {UI_DIR}")
        return 1

    registry = load_registry()

    total_ok          = 0
    failed_components = 0
    total_violations  = 0

    for path in sorted(files):
        cid = os.path.basename(path).replace("_ui.rs", "")
        if target and cid != target: continue
        errs = check_file(path, cid, registry)
        if errs:
            print(f"\n[ERRO] {cid.upper()}")
            for e in errs: print(f"   {e}")
            failed_components += 1
            total_violations  += len(errs)
        else:
            total_ok += 1
            if target: print(f"\n[OK] {cid.upper()} — clean")

    print(f"\n{'='*50}")
    print(f"[OK] {total_ok} UI files clean")
    if total_violations:
        print(f"[FAIL] {failed_components} components failed — {total_violations} violations found")
        return 1
    print("[OK] All UI files canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run(sys.argv[1] if len(sys.argv) > 1 else None))