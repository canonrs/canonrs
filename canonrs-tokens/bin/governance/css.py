"""
css.py — CSS governance layer
Validações: CR-340 a CR-348 (quality, pseudo, hardcode, display, aria, state)
"""

import re
import os

CSS_FORBIDDEN_PSEUDOCLASS = [
    (r':checked\b',       'checked',       'usar [data-rs-state~="checked"]'),
    (r'aria-selected',    'aria-selected', 'usar [data-rs-state~="selected"]'),
    (r'aria-expanded',    'aria-expanded', 'usar [data-rs-state~="open"]'),
    (r'aria-checked',     'aria-checked',  'usar [data-rs-state~="checked"]'),
    (r':hover\b',         'hover',         'usar [data-rs-state~="hover"]'),
    (r':focus\b',         'focus',         'usar [data-rs-state~="focus"]'),
    (r':disabled\b',      'disabled',      'usar [data-rs-state~="disabled"]'),
]

CSS_FORBIDDEN_HARDCODE = [
    (r':\s*rgba?\s*\(',                      'rgba/rgb hardcoded', 'usar token de cor'),
    (r':\s*#[0-9a-fA-F]{3,8}\b',            'hex hardcoded',      'usar token de cor'),
    (r':\s*\d+(\.\d+)?s\b',                  'tempo hardcoded',    'usar var(--motion-duration-)'),
    (r':\s*\d+(\.\d+)?ms\b',                 'tempo ms hardcoded', 'usar var(--motion-duration-)'),
    (r'cubic-bezier\s*\(',                   'easing hardcoded',   'usar var(--motion-ease-)'),
    (r':\s*[0-9]+px\s+[0-9]+px\s+[0-9]+px', 'shadow hardcoded',   'usar token de sombra'),
    (r':\s*(?!(-1px|1px|0px))\d+(\.\d+)?px\b', 'px hardcoded',    'usar var(--space-* ou size-*)'),
    (r':\s*\d+(\.\d+)?rem\b',                'rem hardcoded',      'usar var(--size-* ou font-size-*)'),
    (r'box-shadow\s*:\s*(?!var\()[^;]*?\d+px','box-shadow hardcoded','usar var(--shadow-*)'),
    (r'calc\([^)]*\d+(\.\d+)?(px|rem)\b[^)]*\)', 'calc com hardcode', 'usar token precomputado'),
]

CSS_FORBIDDEN_DISPLAY = [
    r'display\s*:\s*none\s*;',
    r'display\s*:\s*block\s*;',
]

CSS_FORBIDDEN_ARIA_SELECTORS = [
    r'\[aria-selected\]', r'\[aria-expanded\]',
    r'\[aria-checked\]',  r'\[aria-disabled\]',
]

CSS_PSEUDOCLASS_WHITELIST = [
    "::webkit-scrollbar", "::placeholder", ":root", ":not(",
    "focus-visible", "::before", "::after", "::marker",
    ":first-child", ":last-child", ":nth-child", ":empty", "scrollbar",
]

CSS_DISPLAY_NONE_WHITELIST = [
    "-list]", "-content]", "-overlay]", "-dropdown]", "-menu]",
    "-popover]", "-panel]", "-collapse", "combobox-item",
]

CSS_HARDCODE_WHITELIST = [
    "content:", "width: 0", "height: 0", "opacity: 0", "opacity: 1",
    "z-index:", "0%", "100%", "transform:", "rotate(", "translateY(-50%)",
    "scaleY(", "scaleX(", "flex:", "order:", "grid-column", "grid-row",
    "color-mix(", "border-radius: 50%", "border-radius: 0",
    "top: 0", "left: 0", "right: 0", "bottom: 0",
    "padding: 0", "margin: 0", "gap: 0", "min-width: 0",
    "width: 100%", "height: 100%",
    "width: 3px", "width: 4px", "width: 8px", "height: 8px",
    "letter-spacing:", "line-height: 1", "line-height: 0",
    "border: none", "border: 0", "outline: none", "outline: 0",
    "pointer-events: none", "pointer-events: auto",
    "visibility: hidden", "visibility: visible",
    "appearance: none", "list-style: none", "list-style-type: none",
    'content: ""', "content: '", "box-sizing:", "cursor:", "resize:",
    "overflow:", "white-space:", "text-overflow:", "text-decoration:",
    "text-transform:", "font-style:", "font-variant:",
    "display: flex", "display: grid", "display: inline",
    "display: inline-flex", "display: inline-block", "display: contents",
    "display: table", "display: table-cell", "display: table-row",
    "position:", "inset:", "aspect-ratio:", "object-fit:", "object-position:",
    "max-width:", "min-height:", "max-height:",
    "flex-direction:", "flex-wrap:", "flex-shrink:", "flex-grow:",
    "align-items:", "align-self:", "justify-content:", "justify-self:",
    "justify-items:", "place-items:", "place-content:",
    "columns:", "column-gap:", "row-gap:",
    "border-style:", "border-collapse:", "border-spacing:",
    "vertical-align:", "word-break:", "overflow-wrap:", "text-align:",
    "direction:", "unicode-bidi:", "writing-mode:",
    "overscroll-behavior:", "scroll-behavior:", "scrollbar-width:",
    "touch-action:", "user-select:", "will-change:", "isolation:",
    "mix-blend-mode:", "backdrop-filter:", "filter:", "clip-path:",
    "mask:", "background-clip:", "background-origin:", "background-size:",
    "background-repeat:", "background-position:", "background-attachment:",
    "counter-reset:", "counter-increment:",
    "90deg", "180deg", "270deg", "360deg", "-45deg",
    "translateX(-50%)", "translate(-50%", "translateY(-50%)",
]


def check_css_quality(css_file: str, component_id: str = "") -> list:
    """CR-340 a CR-348"""
    errors = []
    if not os.path.exists(css_file):
        return errors
    with open(css_file) as f:
        css = f.read()
    lines = css.splitlines()

    for i, line in enumerate(lines, 1):
        stripped = line.strip()
        if stripped.startswith("/*") or stripped.startswith("*") or stripped.startswith("//"):
            continue

        # CR-348: data-rs-state usa = ao invés de ~=
        if '[data-rs-state="' in stripped and '~=' not in stripped:
            if 'data-rs-state=""' not in stripped:
                errors.append(
                    f"[CR-348] {css_file}:{i} -- data-rs-state usa = ao invés de ~=\n"
                    f"            usar [data-rs-state~=\"X\"]"
                )

        # CR-340: pseudo-classes
        for (pattern, name, fix) in CSS_FORBIDDEN_PSEUDOCLASS:
            if re.search(pattern, stripped):
                if any(w in stripped for w in CSS_PSEUDOCLASS_WHITELIST):
                    continue
                errors.append(
                    f"[CR-340] {os.path.basename(css_file)} linha {i} -- pseudo-state incorreto '{name}'\n"
                    f"            {fix}\n"
                    f"            {stripped[:80]}"
                )

        # CR-341: hardcode proibido
        for (pattern, name, fix) in CSS_FORBIDDEN_HARDCODE:
            if re.search(pattern, stripped):
                if any(w in stripped for w in CSS_HARDCODE_WHITELIST):
                    continue
                if "color-mix(" in stripped:
                    continue
                errors.append(
                    f"[CR-341] {os.path.basename(css_file)} linha {i} -- valor hardcoded '{name}'\n"
                    f"            {fix}\n"
                    f"            {stripped[:80]}"
                )

        # CR-342: display sem state
        for pattern in CSS_FORBIDDEN_DISPLAY:
            if re.search(pattern, stripped):
                if any(w in stripped for w in ["::marker", "list-style", "content:", "::-webkit-scrollbar", "display: block", "display:block", "display: flex", "display:flex", "display: contents", "display:contents", "display: grid", "display:grid"]):
                    continue
                block_selector = ""
                for j in range(i-2, max(0, i-20), -1):
                    l = lines[j].strip()
                    if "{" in l:
                        block_selector = l
                        break
                skip_whitelist = any(w in block_selector for w in CSS_DISPLAY_NONE_WHITELIST)
                if not skip_whitelist and "data-rs-state" not in "\n".join(lines[max(0,i-5):i]):
                    errors.append(
                        f"[CR-342] {os.path.basename(css_file)} linha {i} -- display sem state\n"
                        f"            usar data-rs-state ou [hidden]\n"
                        f"            {stripped[:80]}"
                    )

        # CR-345: aria como seletor CSS
        for pattern in CSS_FORBIDDEN_ARIA_SELECTORS:
            if re.search(pattern, stripped):
                errors.append(
                    f"[CR-345] {os.path.basename(css_file)} linha {i} -- aria attribute como seletor CSS\n"
                    f"            usar [data-rs-state~=\"X\"]\n"
                    f"            {stripped[:80]}"
                )

    return errors


def check_states_in_css(states: list, css: str) -> list:
    STATE_CSS_IMPLICIT = ["closed", "collapsed", "idle", "default", "normal", "hidden", "open", "error", "active", "inactive", "focus"]
    errors = []
    for state in states:
        selector = f'[data-rs-state~="{state}"]'
        if selector not in css:
            if state in STATE_CSS_IMPLICIT:
                continue
            errors.append(
                f"[STATE-CSS] estado '{state}' declarado mas sem seletor CSS correspondente\n"
                f"            adicione: [data-rs-X][data-rs-state~=\"{state}\"] {{ ... }}"
            )
    return errors


def check_hover_override_active(css_file: str) -> list:
    """CR-337: hover deve respeitar estado"""
    errors = []
    if not os.path.exists(css_file):
        return errors
    with open(css_file) as f:
        css_content = f.read()
    lines = css_content.splitlines()
    for i, line in enumerate(lines, 1):
        if ":hover" in line:
            if "data-rs-state" not in line and ":not(" not in line:
                continue
            if "::-webkit-scrollbar" in line or "scrollbar-color" in line:
                continue
            if not re.search(r':not\(\[data-rs-state~=', line):
                errors.append(
                    f"[CR-337] linha {i} -- hover sem guard :not([data-rs-state~=\"...\"])\n"
                    f"            {line.strip()[:80]}"
                )
    return errors


def check_child_combinator(css_file: str) -> list:
    """CR-333: CSS nao deve usar > combinator em island boundaries"""
    errors = []
    if not os.path.exists(css_file):
        return errors
    with open(css_file) as f:
        content = f.read()
    for i, line in enumerate(content.splitlines(), 1):
        if line.strip().startswith("//") or line.strip().startswith("/*"):
            continue
        if re.search(r"> *\[data-rs-", line):
            errors.append(
                f"[CR-333] linha {i} -- child combinator (>) proibido em seletor que cruza boundary\n"
                f"            usar descendant selector (espaco)\n"
                f"            {line.strip()[:80]}"
            )
    return errors


def check_layout_contract(layouts_dir: str) -> list:
    """CR-390: layout contract"""
    import glob
    errors = []
    EXCLUDED = {"page_layout_layout.css", "three_pane_layout.css", "split_view_layout.css", "layouts.css"}
    for css_file in sorted(glob.glob(f"{layouts_dir}/*.css")):
        filename = os.path.basename(css_file)
        if filename in EXCLUDED:
            continue
        with open(css_file) as f:
            css = f.read()
        layout_root = re.search(r'\[data-rs-layout="[^"]+"\]\s*\{([^}]+)\}', css, re.DOTALL)
        if layout_root and "display: grid" in layout_root.group(1):
            continue
        content_blocks = re.findall(
            r'\[data-rs-layout="[^"]+"\]\s*\[data-rs-region="content"\]\s*\{([^}]+)\}',
            css, re.DOTALL
        )
        if not content_blocks:
            errors.append(f"[CR-390] {filename} -- sem [data-rs-region=\"content\"] definido")
            continue
        for block in content_blocks:
            if "display: flex" not in block and "display:flex" not in block:
                errors.append(f"[CR-390] {filename} -- content region sem display:flex")
            if "flex-direction: column" not in block and "flex-direction:column" not in block:
                errors.append(f"[CR-390] {filename} -- content region sem flex-direction:column")
    return errors
