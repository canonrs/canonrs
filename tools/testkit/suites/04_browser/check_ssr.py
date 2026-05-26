#!/usr/bin/env python3
"""
check_ssr.py — SSR DOM completeness governance

Valida que o HTML renderizado pelo servidor contem:
1. Todos os data-rs-interaction no SSR HTML
2. Todos os portals de overlay como div inline (nao vazios)
3. Nenhum componente interaction depende de hydrate-only DOM
4. data-rs-uid presente em todos os roots interativos
"""
import re, sys, urllib.request, urllib.error

BASE_URL = "http://localhost:3000"

SHOWCASE_ROUTES = [
    "/showcase/dialog",
    "/showcase/modal",
    "/showcase/drawer",
    "/showcase/popover",
    "/showcase/tooltip",
    "/showcase/dropdown-menu",
    "/showcase/accordion",
    "/showcase/tabs",
    "/showcase/select",
]

PORTAL_ATTRS = [
    "data-rs-dialog-portal",
    "data-rs-modal-portal",
    "data-rs-drawer-portal",
    "data-rs-sheet-portal",
    "data-rs-popover-portal",
    "data-rs-confirm-dialog-portal",
]


def fetch_html(url):
    try:
        with urllib.request.urlopen(url, timeout=5) as r:
            return r.read().decode("utf-8")
    except urllib.error.URLError as e:
        return None


def check_route(route):
    errors = []
    url = BASE_URL + route
    html = fetch_html(url)

    if html is None:
        return [f"[CR-SSR-000] {route} — servidor nao responde em {url}\n"
                f"             iniciar: cargo leptos serve"]

    # CR-SSR-100: data-rs-interaction deve existir no SSR
    interactions = re.findall(r'data-rs-interaction="([^"]+)"', html)
    if not interactions:
        errors.append(
            f"[CR-SSR-100] {route} — nenhum data-rs-interaction no SSR HTML\n"
            f"             componentes interativos devem ser renderizados no servidor"
        )

    # CR-SSR-101: data-rs-uid deve existir em roots interativos
    uids = re.findall(r'data-rs-uid="([^"]+)"', html)
    if interactions and not uids:
        errors.append(
            f"[CR-SSR-101] {route} — data-rs-interaction presente mas data-rs-uid ausente\n"
            f"             todo root interativo DEVE ter data-rs-uid no SSR"
        )

    # CR-SSR-102: portals de overlay devem ser div inline no SSR (nao vazios)
    for attr in PORTAL_ATTRS:
        if attr in html:
            # verifica que o portal tem conteudo (nao e comentario vazio do leptos Portal)
            pattern = rf'{attr}[^>]*>(\s*<!--.*?-->\s*)</div>'
            empty_portals = re.findall(pattern, html, re.DOTALL)
            if empty_portals:
                errors.append(
                    f"[CR-SSR-102] {route} — {attr} vazio no SSR\n"
                    f"             portal renderiza como comentario — leptos::portal::Portal proibido\n"
                    f"             usar div inline: <div {attr}=\"\">..."
                )

    # CR-SSR-103: data-rs-state deve existir em roots interativos com estado
    # apenas roots (tem data-rs-interaction) precisam de state inicial
    stateful = re.findall(r'<[^>]*data-rs-interaction="overlay"[^>]*>', html)
    for elem in stateful:
        if "data-rs-state" not in elem:
            tag_name = re.search(r'data-rs-(\w+)=""', elem)
            name = tag_name.group(1) if tag_name else "unknown"
            errors.append(
                f"[CR-SSR-103] {route} — {name} (interaction=overlay) sem data-rs-state no SSR\n"
                f"             estado inicial deve ser emitido no servidor"
            )
            break

    return errors


def run():
    errors = []
    ok_routes = 0

    for route in SHOWCASE_ROUTES:
        errs = check_route(route)
        if errs:
            print(f"\n[ERRO] {route}")
            for e in errs: print(f"   {e}")
            errors.extend(errs)
        else:
            ok_routes += 1
            print(f"[OK] {route}")

    print(f"\n{'='*50}")
    print(f"[OK] {ok_routes} routes clean")
    if errors:
        print(f"[FAIL] {len(errors)} SSR violations found")
        return 1
    print("[OK] SSR architecture canonical")
    return 0


if __name__ == "__main__":
    sys.exit(run())
