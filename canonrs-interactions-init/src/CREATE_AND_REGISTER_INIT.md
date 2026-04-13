# Como Criar e Registrar um Init

Guia para adicionar comportamento WASM a um novo componente.

---

## Passo 1 — Primitive define o contrato

O primitive em canonrs-core/src/primitives/{componente}.rs deve ter:

    data-rs-{componente}=""
    data-rs-interaction="init"
    data-rs-uid=uid

---

## Passo 2 — Criar o módulo

Crie o arquivo:

    /opt/docker/monorepo/packages-rust/rs-canonrs/canonrs-interactions-init/src/{componente}.rs

Estrutura mínima:

    use web_sys::Element;
    use crate::runtime::{lifecycle, state};

    pub fn init(root: Element) {
        if !lifecycle::init_guard(&root) { return; }

        // adiciona event listeners, observers, etc
    }

Helpers disponíveis em crate::runtime:

    lifecycle::init_guard(&el)     → evita double init (data-rs-initialized)
    state::add_state(&el, "open")  → adiciona data-rs-state
    state::remove_state(&el, "x")  → remove do data-rs-state
    query::all(&el, "[data-rs-x]") → querySelectorAll dentro do elemento
    query::first(&el, "[sel]")     → querySelector
    aria::set_expanded(&el, true)  → aria-expanded
    aria::set_hidden(&el, true)    → aria-hidden
    keyboard::focus_at(&el, 0)     → foca item por index
    dismiss::bind(&el, &btn)       → bind botao fechar
    interactive::bind(&el)         → hover/focus/active states
    focus::bind_within(&el)        → focus-within state

---

## Passo 3 — Registrar no lib.rs

Em canonrs-interactions-init/src/lib.rs:

1. Adiciona o mod:

    pub mod {componente};

2. Adiciona o dispatch dentro de init_init(el):

    if el.has_attribute("data-rs-{componente}") { {componente}::init(el.clone()); }

---

## Passo 4 — Recompilar o WASM

    cargo run -p canonrs-orchestrator

---

## Passo 5 — Verificar no browser

No console do DevTools:

    // verifica se o elemento existe no DOM
    document.querySelectorAll('[data-rs-{componente}]')

    // verifica se o init rodou
    document.querySelectorAll('[data-rs-{componente}][data-rs-initialized]')

Se tiver data-rs-initialized → init rodou com sucesso.

---

## Estrutura do lib.rs (referencia)

    pub mod runtime;
    pub mod animate;      // ← novo modulo aqui
    pub mod button;
    pub mod checkbox;
    // ...

    pub fn init_init(el: web_sys::Element) {
        if el.has_attribute("data-rs-animate")   { animate::init(el.clone()); }   // ← dispatch aqui
        if el.has_attribute("data-rs-button")    { button::init(el.clone()); }
        if el.has_attribute("data-rs-checkbox")  { checkbox::init(el.clone()); }
        // ...
    }
