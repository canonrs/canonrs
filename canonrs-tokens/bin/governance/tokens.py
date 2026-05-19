"""
tokens.py — Token governance layer
Validações: CR-347 (hardcode), token declared/allowed/unused
"""

import re
import glob
import os

FOUNDATION_PREFIXES = {
    "spacing":     ["--space-"],
    "size":        ["--size-"],
    "radius":      ["--radius-"],
    "motion":      ["--motion-"],
    "typography":  ["--font-", "--line-height-"],
    "shadow":      ["--shadow-"],
    "border":      ["--border-", "--border-thin", "--border-medium", "--border-thick"],
    "interaction": ["--state-", "--focus-ring-", "--opacity-", "--transform-", "--blur-", "--focus-ring-width"],
    "z":           ["--z-", "--layer-"],
    "color":       ["--color-"],
}

RUNTIME_ALLOWED = [
    "--theme-", "--primitive-", "--slider-fill", "--progress",
    "--layout-width-", "--opacity-", "--focus-ring-", "--border-thin",
    "--border-medium", "--border-thick", "--motion-duration-", "--motion-ease-",
    "--space-", "--size-", "--radius-", "--shadow-", "--font-", "--line-height-",
    "--layer-", "--link-group-", "--layout-sidebar-", "--color-overlay-",
]


def extract_declared_tokens(tokens_dir: str) -> set:
    declared = set()
    pattern = re.compile(r'FamilyToken::new\("([^"]+)"')
    for rs_file in glob.glob(f"{tokens_dir}/**/*.rs", recursive=True):
        with open(rs_file) as f:
            for match in pattern.finditer(f.read()):
                declared.add(f"--{match.group(1)}")
    return declared


def extract_vars(css: str) -> list:
    return re.findall(r"var\((--[a-zA-Z0-9-]+)", css)


def is_allowed(var: str, tokens: list, foundations: list, declared: set) -> tuple:
    for prefix in RUNTIME_ALLOWED:
        if var.startswith(prefix):
            return True, ""
    if var not in declared:
        return False, f"[INEXISTENTE] {var} -- token nao existe no sistema"
    for pattern in tokens:
        prefix = pattern.replace("*", "").replace(" ", "")
        if var.startswith(f"--{prefix}"):
            return True, ""
    for foundation in foundations:
        for prefix in FOUNDATION_PREFIXES.get(foundation, []):
            if var.startswith(prefix):
                return True, ""
    return False, f"[CONTRATO] {var} -- existe mas nao declarado em @tokens/@foundation"


def check_unused(tokens: list, vars_used: set, declared: set) -> list:
    warnings = []
    for pattern in tokens:
        if "*" not in pattern:
            continue
        prefix = f"--{pattern.replace('*', '').replace(' ', '')}"
        matching_declared = [t for t in declared if t.startswith(prefix)]
        for t in matching_declared:
            if t not in vars_used:
                warnings.append(f"[UNUSED] {t} -- declarado, nao usado no CSS")
    return warnings


def check_token_hardcode(tokens_dir: str) -> list:
    """CR-347: tokens com valores hardcoded rgba/hex"""
    errors = []
    pattern = re.compile(r'FamilyToken::new\("([^"]+)",\s*"([^"]+)"\)')
    ALLOWED_HARDCODE = [
        "0", "1", "0%", "100%", "none", "transparent", "inherit",
        "normal", "bold", "auto", "unset", "initial",
        "1px", "2px", "3px", "4px", "180deg", "90deg",
        "0.06em", "2.5rem", "0.5", "true", "false"
    ]
    for rs_file in glob.glob(f"{tokens_dir}/**/*.rs", recursive=True):
        with open(rs_file) as f:
            src = f.read()
        filename = os.path.basename(rs_file)
        for match in pattern.finditer(src):
            name  = match.group(1)
            value = match.group(2)
            if any(v == value.strip() for v in ALLOWED_HARDCODE):
                continue
            if value.startswith("var(") or value.startswith("color-mix("):
                continue
            if re.match(r'^rgba?\s*\(', value):
                errors.append(
                    f"[CR-347] {filename} -- {name}: {value}\n"
                    f"            rgba hardcoded proibido — usar color-mix ou token semantico"
                )
            elif re.match(r'^#[0-9a-fA-F]{3,8}$', value.strip()):
                errors.append(
                    f"[CR-347] {filename} -- {name}: {value}\n"
                    f"            hex hardcoded proibido — usar token semantico"
                )
    return errors
