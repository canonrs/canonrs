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
    canonrs-interactions-init/      → lifecycle, focus, hover, click (micro-interactions)

---

## 2. Compilação

Cada grupo é compilado via wasm-pack:

    wasm-pack build canonrs-interactions-{group} --target web --out-dir dist/{group}

Output por grupo:

    dist/{group}/
      canonrs_interactions_{group}.js       <- JS glue (wasm-bindgen)
      canonrs_interactions_{group}_bg.wasm  <- binário WASM

---

## 3. Deploy para o Site

Após compilar, copiar para o site via script:

    bash scripts/core/copy-wasm.sh

O script copia cada grupo de dist/{group}/ para:

    products/canonrs-site/public/wasm/{group}/

O manifest public/js/canonrs-manifest.json já referencia esses paths.

ATENCAO: ao adicionar novo grupo, incluir no loop do copy-wasm.sh E no manifest.

---

## 4. Bundle

canonrs.bundle.js é o único script que o app inclui:

    <script src="/js/canonrs.bundle.js"></script>

Lê o manifest e carrega cada grupo dinamicamente via import().

---

## 5. Runtime no Browser

### Grupos interaction
O bundle detecta elementos com data-rs-interaction="{group}" via MutationObserver
e chama init_{group}(element) no WASM correspondente.

### Grupo init
Após DOMContentLoaded, o bundle chama init_all() que varre o DOM via scan_all()
e despacha para o handler correto via registry::dispatch.

---

## 6. Estado e CSS

Todo estado vive no DOM via atributos data-rs-state. O CSS reage diretamente:

    WASM -> data-rs-state="open" -> CSS [data-rs-state~="open"] -> visual

Zero signals Leptos. Zero JS manual. DOM como SSOT.

Regra CSS: nunca usar pseudo-classes de estado (:focus, :hover). Sempre data-rs-state.

---

## 7. Hot Reload (Dev)

O orchestrator (canonrs-orchestrator) monitora os crates via notify e recompila
o crate canonrs-interactions ao detectar mudanças. Para os crates separados,
recompilar manualmente e rodar copy-wasm.sh.
