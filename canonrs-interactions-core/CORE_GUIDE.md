# CanonRS Interaction Core — Guia de Referência

## Regra de Ouro

Todo engine começa com:

    if !lifecycle::init_guard(&root) { return; }

Nunca inicializar sem guard.
Nunca manipular class, style ou hidden sem sincronizar data-rs-state.

---

## dom/lifecycle

    use canonrs_interactions_core::dom::lifecycle;

    if !lifecycle::init_guard(&root) { return; }
    // Suporta data-rs-reinit para rebind forcado apos hydration

---

## dom/state

    use canonrs_interactions_core::dom::state;

    state::add(&el, "open");
    state::remove(&el, "closed");
    state::has(&el, "focused");
    state::open(&el);
    state::close(&el);
    state::toggle(&el);
    state::expand(&el);
    state::collapse(&el);
    state::is_open(&el);
    state::is_expanded(&el);
    state::set_scroll_lock(true);
    state::set(&el, State::Active);
    state::unset(&el, State::Inactive);
    state::is(&el, State::Open);

Tokens canonicos:
open closed hidden visible active inactive selected unselected
expanded collapsed focused hover disabled checked unchecked
on off loading idle error submitting copied paused entering exiting

---

## dom/query

    use canonrs_interactions_core::dom::query;

    query::safe_target(&e)
    query::safe_current(&e)
    query::closest_el(&el, "[data-rs-x]")
    query::first(&root, "[data-rs-item]")
    query::all(&root, "[data-rs-item]")
    query::root_of("data-rs-tabs", uid)
    query::each("[data-rs-x]", |el| {})
    query::closest_attr(&el, "data-rs-x")
    query::has_ancestor_attr(&el, "data-rs-x")

---

## dom/attrs

    use canonrs_interactions_core::dom::attrs;

    attrs::get_str(&el, "data-rs-value", "default")
    attrs::get_usize(&el, "data-rs-page-size", 10)
    attrs::get_bool(&el, "data-rs-disabled")
    attrs::get_f64(&el, "data-rs-progress", 0.0)
    attrs::get_i32(&el, "data-rs-index", 0)
    attrs::query_one(&root, "[data-rs-trigger]")

---

## behavior/disclosure

    use canonrs_interactions_core::behavior::disclosure;

    disclosure::toggle(&root, &item, &config);
    disclosure::open_item(&item, "[data-rs-trigger]");
    disclosure::close_item(&item, "[data-rs-trigger]");
    disclosure::init_state(&root, &config);
    disclosure::active_triggers(&root, &config);

    let config = DisclosureConfig {
        item_selector:    "[data-rs-accordion-item]",
        trigger_selector: "[data-rs-accordion-trigger]",
        mode:             SelectionMode::Single,
        collapsible:      true,
    };

---

## behavior/selection

    use canonrs_interactions_core::behavior::selection;

    selection::activate(&root, &item, &config);
    selection::activate_by_value(&root, "tab1", &config);
    selection::active_value(&root, &config);
    selection::init_state(&root, &config);

    let config = SelectionConfig {
        item_selector: "[data-rs-tabs-trigger]",
        value_attr:    "data-rs-value",
        aria_selected: true,
        aria_current:  false,
    };

---

## behavior/keyboard

    use canonrs_interactions_core::behavior::keyboard;

    keyboard::init_nav(
        &root,
        "[data-rs-item]",
        NavConfig {
            orientation:  Orientation::Vertical,
            element_type: ElementType::Button,
            wrap:         false,
            focus_state:  "focused",
        },
        on_enter,
        on_escape,
    );

    keyboard::focus_at(&items, index);
    keyboard::find_pos(&items, &target);

Cobre: Arrow Up/Down/Left/Right, Home, End, Enter, Space, Escape.
NUNCA reimplementar navegacao por teclado.

---

## behavior/outside

    use canonrs_interactions_core::behavior::outside;

    outside::register_click_outside("[data-rs-dropdown-menu]", close_fn);

1 listener global por selector — nao N por instancia.

---

## behavior/events

Re-exporta helpers de CustomEvent.
Usar para: rs-datatable-action, rs-selection-change, rs-datatable-bulk-action.

---

## Template minimo de engine

    //! MeuComponente Interaction Engine
    //! Core: dom/{lifecycle, state, query} + behavior/selection::activate_by_value

    use canonrs_interactions_core::dom::{lifecycle, state, query, attrs};
    use canonrs_interactions_core::behavior::{selection, keyboard, outside};

    pub fn init(root: web_sys::Element) {
        if !lifecycle::init_guard(&root) { return; }

        bind_events(&root);
    }

    fn bind_events(root: &web_sys::Element) {
        // usar query::safe_target — nunca e.target() direto
        // usar state::add/remove — nunca manipular class/style/hidden
        // usar selection::activate — nunca reimplementar loop de ativacao
        // usar keyboard::init_nav — nunca reimplementar Arrow/Home/End
    }

---

## Proibido

- Duplicar state.rs, query.rs, lifecycle.rs, attrs.rs localmente
- Reimplementar Arrow/Home/End manualmente
- Reimplementar loop de ativacao de item
- Operar em elementos desconectados do DOM
- Usar strings literais para tokens canonicos (usar State enum)
- Registrar multiplos listeners globais para o mesmo selector
