# CanonRS WASM Flow — Do Código ao Browser

## Visão Geral

CanonRS usa WASM para toda interatividade. Não há JavaScript manual. Cada grupo de componentes tem seu próprio crate Rust compilado para WASM.

---

## 1. Estrutura de Grupos

Cada grupo é um crate independente:
canonrs-interactions-gesture/   → drag, resize, slider, carousel
canonrs-interactions-overlay/   → dropdown, tooltip, dialog, context menu
canonrs-interactions-selection/ → select, combobox, radio, toggle, tree
canonrs-interactions-nav/       → keyboard navigation, tabs
canonrs-interactions-data/      → datatable (filter, sort, pagination, selection)
canonrs-interactions-content/   → markdown, copy, highlight
canonrs-interactions-init/      → table, hover, clickable row (micro-interactions)

---

## 2. Compilação

`build.rs` do crate `canonrs` compila cada grupo com `wasm-pack`:
wasm-pack build canonrs-interactions-{group} --target web --out-dir dist/{group}

Output por grupo:
dist/{group}/
canonrs_interactions_{group}.js       ← JS glue (wasm-bindgen)
canonrs_interactions_{group}_bg.wasm  ← binário WASM

Os arquivos são copiados para:
canonrs-server/assets/wasm/{group}/

O Leptos serve esses assets em `/wasm/{group}/`.

---

## 3. Manifest

O `build.rs` gera automaticamente:
products/canonrs-site/public/canonrs-manifest.json

Com todos os grupos e paths dos arquivos WASM/JS. Versionado com `CARGO_PKG_VERSION`.

---

## 4. Bundle

`canonrs.bundle.js` é o único script que o app inclui:

```html
<script src="/js/canonrs.bundle.js"></script>
```

Ele lê o manifest e carrega cada grupo dinamicamente via `import()`.

---

## 5. Runtime no Browser

### Grupos interaction
O bundle detecta elementos com `data-rs-interaction="{group}"` via `MutationObserver` e chama `init_{group}(element)` no WASM correspondente.

### Grupos init
Após `DOMContentLoaded`, o bundle chama `init_all()` que varre o DOM por seletores como `[data-rs-table]` e registra listeners leves.

---

## 6. Estado e CSS

Todo estado vive no DOM via atributos `data-rs-*`. O CSS reage diretamente:
WASM → data-rs-state="open" → CSS [data-rs-state~="open"] → visual

Zero signals Leptos. Zero JS manual. DOM como SSOT.

---

## 7. Hot Reload (Dev)

O orchestrator monitora cada grupo com `cargo watch` e recompila automaticamente ao salvar qualquer arquivo em `canonrs-interactions-{group}/src/`.

