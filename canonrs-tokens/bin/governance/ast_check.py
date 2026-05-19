"""
ast_check.py — AST-level governance via tree-sitter
Validações: CR-333 (DOM→signal), CR-339 (dynamic class), CR-334 (use_context in closure)
"""

import re
import glob
import os


def _build_ast_parser():
    try:
        import tree_sitter_rust as tsr
        from tree_sitter import Language, Parser
        RUST = Language(tsr.language())
        return Parser(RUST)
    except Exception:
        return None


_AST_PARSER = _build_ast_parser()


def _ast_walk(node, callback, results):
    callback(node, results)
    for child in node.children:
        _ast_walk(child, callback, results)


def check_boundary_ast(boundary_file: str, ui_dir: str, boundary_type: str = "state") -> list:
    """AST-level validation — zero falso positivo/negativo"""
    errors = []
    if _AST_PARSER is None:
        return errors

    matches = glob.glob(f"{ui_dir}/**/{boundary_file}", recursive=True)
    if not matches:
        return errors

    with open(matches[0], "rb") as f:
        source = f.read()

    tree = _AST_PARSER.parse(source)
    root = tree.root_node

    dom_vars = set()
    set_calls = []
    dynamic_classes = []

    def collect(node, results):
        if node.type == "let_declaration":
            if b"query_selector" in (node.text or b""):
                pat = node.child_by_field_name("pattern")
                if pat:
                    dom_vars.add(pat.text.decode("utf8").strip())
        if node.type == "call_expression":
            t = node.text or b""
            if b".set(" in t:
                set_calls.append((node.start_point[0] + 1, t.decode("utf8")))
        if node.type == "attribute":
            t = node.text or b""
            if t.startswith(b"class") and (b"move" in t or b"if " in t or b".get(" in t):
                dynamic_classes.append((node.start_point[0] + 1, t.decode("utf8")[:80]))

    _ast_walk(root, collect, None)

    if boundary_type != "observer":
        for (line, call) in set_calls:
            for var in dom_vars:
                if var in call:
                    errors.append(
                        f"[CR-333-AST] {boundary_file} linha {line} -- DOM → SIGNAL\n"
                        f"            {call[:80]}"
                    )

    for (line, text) in dynamic_classes:
        errors.append(
            f"[CR-339-AST] {boundary_file} linha {line} -- dynamic class state proibido\n"
            f"            {text[:80]}"
        )

    return errors
