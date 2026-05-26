#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""
check_css.py — CSS layer governance
Critérios: CANONRS CSS (1 pt)
Localização: canonrs-server/styles/ui/*.css

1. Token Purity (0.25pt) — 100% var(--*), zero hardcode
2. State Integrity (0.25pt) — data-rs-state~=, sem pseudo-class como source of truth
3. Layout Responsibility (0.25pt) — sem lógica hardcoded
4. Attribute Contract (0.25pt) — apenas atributos canonicos data-rs-*
"""
import re, glob, os, sys

CSS_DIR = _CANONRS_ROOT + "/canonrs-server/styles/ui"

# CR-340: pseudo-classes proibidas como source of truth
CSS_FORBIDDEN_PSEUDOCLASS = [
    (r':checked\b',    'checked',       'usar [data-rs-state~="checked"]'),
    (r':hover\b',      'hover',         'usar [data-rs-state~="hover"]'),
    (r':focus\b(?!-)',  'focus',        'usar [data-rs-state~="focus"]'),
    (r':disabled\b',   'disabled',      'usar [data-rs-state~="disabled"]'),
    (r':active\b',     'active',        'usar [data-rs-state~="active"]'),
]

CSS_FORBIDDEN_ARIA_SELECTORS = [
    (r'\[aria-selected\b', 'aria-selected', 'usar [data-rs-state~="selected"]'),
    (r'\[aria-expanded\b', 'aria-expanded', 'usar [data-rs-state~="open"]'),
    (r'\[aria-checked\b',  'aria-checked',  'usar [data-rs-state~="checked"]'),
    (r'\[aria-disabled\b', 'aria-disabled', 'usar [data-rs-state~="disabled"]'),
]

# CR-341: hardcode proibido
CSS_FORBIDDEN_HARDCODE = [
    (r':\s*rgba?\s*\(',                         'rgba/rgb hardcoded',  'usar token de cor'),
    (r':\s*#[0-9a-fA-F]{3,8}\b',               'hex hardcoded',       'usar token de cor'),
    (r':\s*\d+(\.\d+)?s\b',                     'tempo hardcoded',     'usar var(--motion-duration-)'),
    (r':\s*\d+(\.\d+)?ms\b',                    'tempo ms hardcoded',  'usar var(--motion-duration-)'),
    (r'cubic-bezier\s*\(',                      'easing hardcoded',    'usar var(--motion-ease-)'),
    (r':\s*[0-9]+px\s+[0-9]+px\s+[0-9]+px',    'shadow hardcoded',    'usar token de sombra'),
    (r':\s*(?!(-1px|1px|0px|0))\d+(\.\d+)?px\b','px hardcoded',       'usar var(--space-* ou size-*)'),
    (r':\s*\d+(\.\d+)?rem\b',                   'rem hardcoded',       'usar var(--size-* ou font-size-*)'),
    (r'box-shadow\s*:\s*(?!var\()[^;]*?\d+px',  'box-shadow hardcoded','usar var(--shadow-*)'),
]

CSS_PSEUDOCLASS_WHITELIST = [
    "::webkit-scrollbar", "::placeholder", ":root", ":not(", "focus-visible",
    "::before", "::after", "::marker", ":first-child", ":last-child",
    ":nth-child", ":empty", "scrollbar", ":is(", ":where(", ":has(",
]

CSS_HARDCODE_WHITELIST = [
    "content:", "width: 0", "height: 0", "opacity: 0", "opacity: 1",
    "z-index:", "0%", "100%", "transform:", "rotate(", "translateY(-50%)",
    "scaleY(", "scaleX(", "flex:", "order:", "grid-column", "grid-row",
    "color-mix(", "border-radius: 50%", "border-radius: 0",
    "top: 0", "left: 0", "right: 0", "bottom: 0",
    "padding: 0", "margin: 0", "gap: 0", "min-width: 0",
    "width: 100%", "height: 100%", "width: 3px", "width: 4px",
    "width: 8px", "height: 8px", "letter-spacing:", "line-height: 1",
    "line-height: 0", "border: none", "border: 0", "outline: none",
    "outline: 0", "pointer-events: none", "pointer-events: auto",
    "visibility: hidden", "visibility: visible", "appearance: none",
    "list-style: none", "list-style-type: none", "content: none",
    'content: ""', "content: '", "box-sizing:", "cursor:", "resize:",
    "overflow:", "white-space:", "text-overflow:", "text-decoration:",
    "text-transform:", "font-style:", "font-variant:",
    "display: flex", "display: grid", "display: inline",
    "display: inline-flex", "display: inline-block", "display: contents",
    "display: table", "display: table-cell", "display: table-row",
    "position:", "inset:", "aspect-ratio:", "object-fit:", "object-position:",
    "max-width:", "min-height:", "max-height:", "flex-direction:", "flex-wrap:",
    "flex-shrink:", "flex-grow:", "align-items:", "align-self:",
    "justify-content:", "justify-self:", "justify-items:", "place-items:",
    "place-content:", "columns:", "column-gap:", "row-gap:",
    "border-style:", "border-collapse:", "border-spacing:", "vertical-align:",
    "word-break:", "overflow-wrap:", "text-align:", "direction:",
    "unicode-bidi:", "writing-mode:", "overscroll-behavior:", "scroll-behavior:",
    "scrollbar-width:", "touch-action:", "user-select:", "will-change:",
    "isolation:", "mix-blend-mode:", "backdrop-filter:", "filter:",
    "clip-path:", "mask:", "background-clip:", "background-origin:",
    "background-size:", "background-repeat:", "background-position:",
    "background-attachment:", "counter-reset:", "counter-increment:",
    "90deg", "180deg", "270deg", "360deg", "-45deg",
    "translateX(-50%)", "translate(-50%", "translateY(-50%)",
    "border-radius: 50%", "transparent", "inherit", "initial", "unset",
    "auto", "none", "normal", "bold",
]

CSS_DISPLAY_WHITELIST = [
    "-list]", "-content]", "-overlay]", "-dropdown]", "-menu]",
    "-popover]", "-panel]", "-collapse", "combobox-item",
    "sidebar-toggle", "sidebar-overlay",
]

# atributos data-rs-* nao canonicos (sufixos proibidos)
FORBIDDEN_ATTR_SUFFIXES = [
    "-element]", "-slot]", "-wrapper-div]", "-inner-div]",
]


def get_block_selector(lines, line_idx):
    """Retorna o seletor do bloco CSS da linha atual."""
    for j in range(line_idx - 2, max(0, line_idx - 30), -1):
        l = lines[j].strip()
        if "{" in l:
            return l
    return ""


def check_file(path, cid):
    errors = []
    with open(path) as f:
        css = f.read()
    lines = css.splitlines()

    for i, line in enumerate(lines, 1):
        stripped = line.strip()

        # ignora comentarios e linhas vazias
        if not stripped:
            continue
        if stripped.startswith("/*") or stripped.startswith("*") or stripped.startswith("//"):
            continue

        # CR-348: data-rs-state deve usar ~= não =
        if '[data-rs-state="' in stripped and '~=' not in stripped \
                and 'data-rs-state=""' not in stripped:
            errors.append(f"[CR-348] {cid} linha {i} — data-rs-state usa = em vez de ~=\n"
                          f"         usar [data-rs-state~=\"X\"]\n"
                          f"         {stripped[:80]}")

        # CR-340: pseudo-classes como source of truth
        for pattern, name, fix in CSS_FORBIDDEN_PSEUDOCLASS:
            if re.search(pattern, stripped):
                if any(w in stripped for w in CSS_PSEUDOCLASS_WHITELIST):
                    continue
                # cursor, outline, box-shadow em pseudo-class sao complemento visual permitido
                context_next = lines[i] if i < len(lines) else ""
                block_lines = lines[i-1:min(len(lines), i+5)]
                block_props = " ".join(block_lines)
                if all(prop in block_props for prop in ["cursor"]) and "color" not in block_props and "background" not in block_props:
                    continue
                # focus-visible e permitido
                if "focus-visible" in stripped:
                    continue
                # permitido como complemento se ha data-rs-state no bloco
                block_sel = get_block_selector(lines, i)
                if "data-rs-state" in block_sel or "data-rs-state" in stripped:
                    continue
                errors.append(f"[CR-340] {cid} linha {i} — pseudo-state '{name}' sem data-rs-state\n"
                              f"         {fix}\n"
                              f"         {stripped[:80]}")

        # CR-345: aria attributes como seletores CSS — proibido como source of truth
        # permitido: aria em :not() como complemento defensivo
        for pattern, name, fix in CSS_FORBIDDEN_ARIA_SELECTORS:
            if re.search(pattern, stripped):
                # permitido em :not([aria-...]) — uso defensivo/complemento
                if ":not(" in stripped and re.search(pattern + r"[^]]*\]\)", stripped):
                    continue
                errors.append(f"[CR-345] {cid} linha {i} — aria attribute como seletor: {name}\n"
                              f"         {fix}\n"
                              f"         {stripped[:80]}")

        # CR-341: hardcode proibido
        for pattern, name, fix in CSS_FORBIDDEN_HARDCODE:
            if re.search(pattern, stripped):
                if any(w in stripped for w in CSS_HARDCODE_WHITELIST):
                    continue
                if "color-mix(" in stripped:
                    continue
                errors.append(f"[CR-341] {cid} linha {i} — {name}\n"
                              f"         {fix}\n"
                              f"         {stripped[:80]}")

        # CR-342: display:none/block fora de state
        if re.search(r'display\s*:\s*none\s*;', stripped):
            # pseudo-elementos sao sempre permitidos
            if "::-webkit-scrollbar" in stripped or "::marker" in stripped or "::before" in stripped or "::after" in stripped:
                pass
            else:
                block_sel = get_block_selector(lines, i)
                # pseudo-elementos no seletor do bloco
                if "::-webkit-scrollbar" in block_sel or "::marker" in block_sel:
                    pass
                # classe utilitaria .rs-* e permitida
                elif re.search(r'^\.rs-', block_sel.strip()):
                    pass
                # data-rs-mode e data-rs-* como state alternativo sao permitidos
                elif "data-rs-mode" in block_sel or "data-rs-" in block_sel:
                    pass
                elif not any(w in block_sel for w in CSS_DISPLAY_WHITELIST):
                    context = "\n".join(lines[max(0, i-8):i])
                    if "data-rs-state" not in context and "[hidden]" not in context:
                        errors.append(f"[CR-342] {cid} linha {i} — display:none fora de state\n"
                                      f"         usar [data-rs-state~=\"X\"] ou [hidden]\n"
                                      f"         {stripped[:80]}")

        # CR-346: atributos data-rs-* nao canonicos — apenas sufixos claramente errados
        if re.search(r'\[data-rs-[a-z-]+-(?:element|wrapper-div|inner-div)\]', stripped):
            errors.append(f"[CR-346] {cid} linha {i} — atributo data-rs-* nao canonico\n"
                          f"         usar apenas atributos declarados no primitive\n"
                          f"         {stripped[:80]}")

        # CR-347: [data-state] ou [data-selected] proibidos (nao canonicos)
        if re.search(r'\[data-state\b', stripped) or re.search(r'\[data-selected\b', stripped):
            errors.append(f"[CR-347] {cid} linha {i} — atributo nao canonico [data-state/data-selected]\n"
                          f"         usar [data-rs-state~=\"X\"]\n"
                          f"         {stripped[:80]}")

    return errors


def run(target=None):
    files = glob.glob(f"{CSS_DIR}/*.css")
    files += glob.glob(f"{CSS_DIR}/**/*.css", recursive=True)
    files = sorted(set(files))

    if not files:
        print(f"\n[FAIL] 0 files analyzed — path: {CSS_DIR}")
        return 1

    total_ok = total_err = failed = 0
    for path in sorted(files):
        cid = os.path.basename(path).replace(".css", "")
        if target and cid != target: continue
        errs = check_file(path, cid)
        if errs:
            print(f"\n[ERRO] {cid.upper()}")
            for e in errs: print(f"   {e}")
            failed += 1
            total_err += len(errs)
        else:
            total_ok += 1
            if target: print(f"\n[OK] {cid.upper()} — clean")

    print(f"\n{'='*50}")
    print(f"[OK] {total_ok} CSS files clean")
    if total_err:
        print(f"[FAIL] {failed} components failed — {total_err} violations found")
        return 1
    print("[OK] All CSS files canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run(sys.argv[1] if len(sys.argv) > 1 else None))