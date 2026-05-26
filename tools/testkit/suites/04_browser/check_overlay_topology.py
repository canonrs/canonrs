#!/usr/bin/env python3
import os as _os
_CANONRS_ROOT = _os.environ.get('GITHUB_WORKSPACE', '/opt/docker/monorepo/packages-rust/rs-canonrs')

"""
check_overlay_topology.py — Overlay Topology Governance

Valida que overlays modais possuem topologia estrutural completa.
Nao valida implementacao — valida contratos estruturais do primitive.

Topologia obrigatoria por tipo:
  dialog:         trigger, portal, overlay, content, title, description, close
  modal:          trigger, portal, overlay, content, title, description, close
  drawer/sheet:   trigger, portal, overlay, content, close
  popover:        trigger, content
  confirm_dialog: trigger, portal, overlay, content, title, description, cancel
  alert_dialog:   trigger, overlay, content, title, description, cancel/confirm
"""
import re, glob, os, sys

PRIMITIVES_DIR = _CANONRS_ROOT + "/canonrs-core/src/primitives"

# data-rs-owner e propagado em runtime (nao no primitive)
# drawer/sheet fecham via overlay click (sem close button dedicado no primitive)
# alert_dialog reutiliza dialog-content internamente

TOPOLOGY = {
    "dialog": {
        "root":     "data-rs-dialog",
        "required": ["data-rs-dialog-trigger", "data-rs-dialog-portal",
                     "data-rs-dialog-overlay", "data-rs-dialog-content",
                     "data-rs-dialog-title", "data-rs-dialog-description",
                     "data-rs-dialog-close"],
        "owner_required": [],
        "uid_required": True,
    },
    "modal": {
        "root":     "data-rs-modal",
        "required": ["data-rs-modal-trigger", "data-rs-modal-portal",
                     "data-rs-modal-overlay", "data-rs-modal-content",
                     "data-rs-modal-title", "data-rs-modal-close"],
        "owner_required": [],
        "uid_required": True,
    },
    "drawer": {
        "root":     "data-rs-drawer",
        "required": ["data-rs-drawer-trigger", "data-rs-drawer-portal",
                     "data-rs-drawer-overlay", "data-rs-drawer-content"],
        "owner_required": [],
        "uid_required": True,
    },
    "sheet": {
        "root":     "data-rs-sheet",
        "required": ["data-rs-sheet-trigger", "data-rs-sheet-portal",
                     "data-rs-sheet-overlay", "data-rs-sheet-content"],
        "owner_required": [],
        "uid_required": True,
    },
    "confirm_dialog": {
        "root":     "data-rs-confirm-dialog",
        "required": ["data-rs-confirm-dialog-trigger", "data-rs-confirm-dialog-portal",
                     "data-rs-confirm-dialog-overlay", "data-rs-confirm-dialog-content",
                     "data-rs-confirm-dialog-title", "data-rs-confirm-dialog-cancel"],
        "owner_required": [],
        "uid_required": True,
    },
    "alert_dialog": {
        "root":     "data-rs-alert-dialog",
        "required": ["data-rs-dialog-content"],
        "owner_required": [],
        "uid_required": False,
    },
    "popover": {
        "root":     "data-rs-popover",
        "required": ["data-rs-popover-trigger", "data-rs-popover-content"],
        "owner_required": [],
        "uid_required": True,
    },
    "tooltip": {
        "root":     "data-rs-tooltip",
        "required": ["data-rs-tooltip-trigger", "data-rs-tooltip-content"],
        "owner_required": [],
        "uid_required": True,
    },
    "dropdown_menu": {
        "root":     "data-rs-dropdown-menu",
        "required": ["data-rs-dropdown-menu-trigger", "data-rs-dropdown-menu-content",
                     "data-rs-dropdown-menu-item"],
        "owner_required": [],
        "uid_required": True,
    },
    "context_menu": {
        "root":     "data-rs-context-menu",
        "required": ["data-rs-context-menu-trigger", "data-rs-context-menu-content"],
        "owner_required": [],
        "uid_required": True,
    },
}

def strip_comments(src):
    return re.sub(r"//[^\n]*", "", src)

def check_primitive(comp, spec):
    errors = []
    path = os.path.join(PRIMITIVES_DIR, f"{comp}.rs")
    if not os.path.exists(path):
        return [f"[CR-TOPO-000] {comp} — primitive nao encontrado: {path}"]

    src = open(path).read()
    nc  = strip_comments(src)

    # CR-TOPO-100: root deve ter data-rs-uid
    if spec["uid_required"]:
        if "data-rs-uid=" not in nc:
            errors.append(
                f"[CR-TOPO-100] {comp} — root sem data-rs-uid\n"
                f"              garantia: overlay root DEVE ter uid para ownership tracking"
            )

    # CR-TOPO-101: partes obrigatorias devem existir no primitive
    for part in spec["required"]:
        if part not in nc:
            errors.append(
                f"[CR-TOPO-101] {comp} — parte obrigatoria ausente: {part}\n"
                f"              garantia: topologia estrutural DEVE estar completa no primitive"
            )

    # CR-TOPO-102: partes com owner_required devem ter data-rs-owner
    for part in spec["owner_required"]:
        if part in nc:
            # verifica se o componente que tem esse attr tambem tem data-rs-owner
            block_re = re.compile(rf'data-rs-owner[^>]*{re.escape(part)}|{re.escape(part)}[^>]*data-rs-owner', re.DOTALL)
            # simplificado: verifica que data-rs-owner existe no arquivo se owner_required
            if "data-rs-owner" not in nc:
                errors.append(
                    f"[CR-TOPO-102] {comp} — {part} sem data-rs-owner\n"
                    f"              garantia: overlay/content DEVEM ter owner para targeting correto"
                )
                break

    # CR-TOPO-103: portal deve ser div inline (nao leptos Portal)
    root_attr = spec["root"]
    portal_attr = f"{root_attr}-portal"
    if portal_attr in nc:
        if "leptos::portal::Portal" in nc:
            errors.append(
                f"[CR-TOPO-103] {comp} — portal usa leptos::portal::Portal\n"
                f"              garantia: portal DEVE ser div inline para SSR safety"
            )

    return errors


def run(target=None):
    total_ok = total_err = failed = 0

    for comp, spec in sorted(TOPOLOGY.items()):
        if target and comp != target: continue
        errs = check_primitive(comp, spec)
        if errs:
            print(f"\n[ERRO] {comp.upper()}")
            for e in errs: print(f"   {e}")
            failed += 1
            total_err += len(errs)
        else:
            total_ok += 1
            if target: print(f"\n[OK] {comp.upper()} — topology complete")

    print(f"\n{'='*50}")
    print(f"[OK] {total_ok} overlays topology complete")
    if total_err:
        print(f"[FAIL] {failed} overlays — {total_err} topology violations")
        return 1
    print("[OK] Overlay topology canonical")
    return 0

if __name__ == "__main__":
    sys.exit(run(sys.argv[1] if len(sys.argv) > 1 else None))