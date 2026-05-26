#!/usr/bin/env python3
import re, glob, os, sys, yaml
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')


PRIMITIVES_DIR = _CANONRS_ROOT + "/canonrs-core/src/primitives"
UI_DIR         = _CANONRS_ROOT + "/canonrs-server/src/ui"
INT_BASE       = _CANONRS_ROOT + ""

FORBIDDEN_ATTRS = [
    ("data-rs-open",       "CR-352: usar data-rs-state"),
    ("data-rs-visible",    "CR-353: usar data-rs-state"),
    ("data-rs-component",  "CR-355: redundante — remover"),
    ("data-rs-behavior",   "CR-356: usar data-rs-interaction"),
    ("data-rs-visibility", "CR-357: usar data-rs-state"),
]

LAYOUT_PASSIVES = ["stack", "flex", "grid", "container", "center", "spacer"]
EXTRA_VALID_INTERACTIONS = {"dismiss"}


def strip_comments_and_strings(src):
    src = re.sub(r"//[^\n]*", "", src)
    src = re.sub(r'"[^"\\]*(?:\\.[^"\\]*)*"', '""', src)
    return src


def discover_interaction_groups():
    dirs = glob.glob(f"{INT_BASE}/canonrs-interactions-*")
    groups = set()
    for d in dirs:
        name = os.path.basename(d).replace("canonrs-interactions-", "")
        groups.add(name)
    return groups | EXTRA_VALID_INTERACTIONS


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


def is_interactive(cid, registry):
    entry = registry.get(cid, {})
    btype = entry.get("boundary_type", "") or ""
    # apenas init e interaction precisam de data-rs-interaction
    # passthrough com states = display component, nao precisa de interaction group
    return btype in ("init", "interaction")


def extract_view_blocks(src_clean):
    """Extrai blocos view!{} balanceando chaves."""
    blocks = []
    for vm in re.finditer(r"view!\s*\{", src_clean):
        start = vm.start()
        depth = 0
        end = start
        for i, ch in enumerate(src_clean[start:], start):
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    end = i
                    break
        blocks.append(src_clean[start:end])
    return blocks


def check_file(path, cid, registry, valid_interactions):
    errors = []
    src = open(path).read()
    src_clean = strip_comments_and_strings(src)
    has_component = "#[component]" in src
    is_layout     = any(p in cid for p in LAYOUT_PASSIVES)
    interactive   = is_interactive(cid, registry)

    # CR-350: data-rs-uid obrigatorio no root de componentes interativos
    if has_component and not is_layout and interactive:
        if "data-rs-uid=" not in src_clean:
            errors.append(f"[CR-350] {cid} — data-rs-uid ausente no root\n"
                          f"         usar crate::infra::uid::generate(\"prefix\")")
        if "generate(" not in src_clean:
            errors.append(f"[CR-350] {cid} — generate() ausente\n"
                          f"         usar crate::infra::uid::generate()")

    # CR-351: fn *_uid() propria proibida
    if re.search(r"fn \w+_uid\(", src_clean):
        errors.append(f"[CR-351] {cid} — fn *_uid() propria proibida\n"
                      f"         usar crate::infra::uid::generate()")

    # CR-352..357: atributos proibidos
    for attr, msg in FORBIDDEN_ATTRS:
        if f"{attr}=" in src_clean or f"{attr} " in src_clean:
            errors.append(f"[{msg.split(':')[0]}] {cid} — {attr} proibido\n"
                          f"         {msg.split(':',1)[1].strip()}")

    # CR-353: data-rs-interaction obrigatorio apenas em componentes init/interaction
    if has_component and not is_layout and interactive:
        if "data-rs-interaction=" not in src_clean:
            errors.append(f"[CR-353] {cid} — data-rs-interaction ausente no root\n"
                          f"         grupos validos: {', '.join(sorted(valid_interactions))}")
        else:
            m = re.search(r'data-rs-interaction="([^"]+)"', src)
            if m and m.group(1) not in valid_interactions:
                errors.append(f"[CR-353] {cid} — data-rs-interaction=\"{m.group(1)}\" invalido\n"
                              f"         grupos validos: {', '.join(sorted(valid_interactions))}")

    # CR-354: data-rs-selected fora de state
    for i, line in enumerate(src.splitlines(), 1):
        line_clean = re.sub(r"//[^\n]*", "", line)
        line_clean = re.sub(r'"[^"]*"', '""', line_clean)
        if "data-rs-selected=" in line_clean and "data-rs-state" not in line_clean:
            errors.append(f"[CR-354] {cid} linha {i} — data-rs-selected fora de state\n"
                          f"         usar data-rs-state~=\"selected\"")

    # CR-355: identificador semantico data-rs-{name}="" obrigatorio
    if has_component and not is_layout and interactive:
        has_semantic = bool(re.search(r'data-rs-[a-z][a-z0-9-]+=""', src_clean))
        if not has_semantic:
            errors.append(f"[CR-355] {cid} — identificador semantico data-rs-{{name}}=\"\" ausente\n"
                          f"         ex: data-rs-button=\"\" data-rs-accordion=\"\"")

    # CR-360: leptos::portal::Portal proibido em primitives — nao funciona em SSR
    if "leptos::portal::Portal" in src:
        errors.append(f"[CR-360] {cid} — leptos::portal::Portal proibido\n"
                      f"         Portal do Leptos renderiza vazio no SSR\n"
                      f"         usar div inline + mover para body via JS (portal::move_to_body)")

    # CR-358: dois data-rs-state no mesmo elemento
    for i, line in enumerate(src.splitlines(), 1):
        line_clean = re.sub(r"//[^\n]*", "", line)
        if line_clean.count("data-rs-state") > 1:
            errors.append(f"[CR-358] {cid} linha {i} — dois data-rs-state no mesmo elemento\n"
                          f"         {line.strip()[:80]}")
            break

    # CR-359: unwrap_or_default() dentro de view! proibido exceto como prop passthrough
    for view_block in extract_view_blocks(src_clean):
        for m in re.finditer(r"unwrap_or_default\(\)", view_block):
            pre      = view_block[:m.start()].rfind("\n")
            line_ctx = view_block[pre+1:view_block.find("\n", m.start())].strip()
            is_prop  = bool(re.search(r"\w+\s*=\s*[\w.()\[\]]+\.unwrap_or_default\(\)", line_ctx))
            if not is_prop:
                errors.append(f"[CR-359] {cid} — unwrap_or_default() dentro do view! fora de prop passthrough\n"
                              f"         calcular antes do view!")
                break

    # CR-362: node_ref pattern no Primitive
    if re.search(r"node_ref", src_clean):
        has_prop = bool(re.search(
            r"#\[prop\(optional\)\]\s*node_ref\s*:\s*Option<NodeRef<",
            src_clean
        ))
        # aceita unwrap_or_default() direto ou via variavel intermediaria
        has_passthrough = bool(re.search(
            r"node_ref\s*=\s*node_ref\.unwrap_or_default\(\)",
            src_clean
        )) or bool(re.search(
            r"=\s*node_ref\.unwrap_or_default\(\)",
            src_clean
        ))
        if not has_prop:
            errors.append(
                f"[CR-362] {cid} — node_ref deve ser Option<NodeRef<T>> no primitive\n"
                f"         usar: #[prop(optional)] node_ref: Option<NodeRef<html::Input>>"
            )
        if not has_passthrough:
            errors.append(
                f"[CR-362] {cid} — node_ref deve usar passthrough adapter\n"
                f"         usar: node_ref=node_ref.unwrap_or_default()"
            )

    # CR-361: button como container de elementos interativos
    if "<button" in src_clean:
        for block in re.findall(r"<button[^>]*>(.*?)</button>", src_clean, re.DOTALL):
            if re.search(r"<(input|button|select|textarea|a\b)", block):
                errors.append(f"[CR-361] {cid} — <button> como container de elementos interativos\n"
                              f"         usar <div> como container")
                break

    return errors


def run(target=None):
    files = [f for f in glob.glob(f"{PRIMITIVES_DIR}/*.rs")
             if not os.path.basename(f).startswith("mod")]
    if not files:
        print(f"\n[FAIL] 0 files analyzed — path: {PRIMITIVES_DIR}")
        return 1

    registry           = load_registry()
    valid_interactions = discover_interaction_groups()

    total_ok          = 0
    failed_components = 0
    total_violations  = 0

    for path in sorted(files):
        cid = os.path.basename(path).replace(".rs", "")
        if target and cid != target: continue
        errs = check_file(path, cid, registry, valid_interactions)
        if errs:
            print(f"\n[ERRO] {cid.upper()}")
            for e in errs: print(f"   {e}")
            failed_components += 1
            total_violations  += len(errs)
        else:
            total_ok += 1
            if target: print(f"\n[OK] {cid.upper()} — clean")

    print(f"\n{'='*50}")
    print(f"[OK] {total_ok} primitives clean")
    if total_violations:
        print(f"[FAIL] {failed_components} components failed — {total_violations} violations found")
        return 1
    print("[OK] All primitives canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run(sys.argv[1] if len(sys.argv) > 1 else None))