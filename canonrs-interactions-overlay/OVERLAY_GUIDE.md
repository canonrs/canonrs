# CanonRS Interaction Overlay — Guia de Referência

## Arquitetura

Todo overlay segue este fluxo:

    init(root)
      → lifecycle::init_guard
      → stack::push + stack::ensure_global_listeners
      → portal::move_to_body
      → transition::set_state_nodes("open")
      → focus::focus_first
      → inert::set_inert_background(true)
      → stack::register_click + stack::register_keydown

Fechar:
      → transition::set_state_nodes("exiting")
      → timeout(duration_ms)
      → transition::set_state_nodes("closed")
      → inert::set_inert_background(false)
      → focus restore
      → stack::pop + stack::unregister

---

## runtime/stack

    use crate::runtime::stack;

    stack::ensure_global_listeners();   // OBRIGATORIO — 1 vez por sessao
    stack::push(uid, "dialog");         // adiciona ao stack, retorna z-index
    stack::pop(uid);                    // remove do stack
    stack::is_top(uid);                 // true se este overlay esta no topo
    stack::z_for(uid);                  // z-index do overlay
    stack::has_modal_open();            // true se algum modal esta aberto
    stack::stack_empty();               // true se nenhum overlay aberto
    stack::register_click(uid, cb);     // registra handler de click
    stack::register_keydown(uid, cb);   // registra handler de keydown
    stack::unregister(uid);             // remove handlers

Kinds modais: dialog confirm-dialog modal sheet drawer alert-dialog
Kinds nao-modais: popover dropdown tooltip hover-card context-menu

---

## runtime/transition

    use crate::runtime::transition;

    transition::set_state_nodes(&overlay, &content, "open");
    transition::set_state_nodes(&overlay, &content, "exiting");
    transition::set_state_nodes(&overlay, &content, "closed");
    transition::duration_ms(&el, "--modal-transition-duration");

Estados: entering → open → exiting → closed
CSS deve reagir a data-rs-state~="open" e data-rs-state~="exiting".

---

## runtime/focus

    use crate::runtime::focus;

    focus::focus_first(&content);          // foca primeiro elemento focavel
    focus::focusable_elements(&content);   // lista elementos focaveis
    focus::active_element();               // elemento com foco atual
    focus::focus_escaped(&content);        // true se foco saiu do content
    focus::trap_tab(&e, &content);         // Tab/Shift+Tab dentro do content

Sequencia correta:
    let prev_focus = focus::active_element();
    focus::focus_first(&content);
    // ao fechar:
    if let Some(el) = prev_focus {
        el.dyn_into::<HtmlElement>().ok().map(|h| h.focus());
    }

---

## runtime/inert

    use crate::runtime::inert;

    inert::set_inert_background(true,  uid, "data-rs-modal-portal");
    inert::set_inert_background(false, uid, "data-rs-modal-portal");

Marca todos os filhos do body com inert exceto o portal do overlay.
Usa data-rs-inert-{uid} como marker para cleanup preciso.
NUNCA aplicar inert em overlays nao-modais (popover, dropdown, tooltip).

---

## runtime/portal

    use crate::runtime::portal;

    portal::portal_of(&root, "data-rs-modal-portal", uid);
    portal::move_to_body(&portal, uid);
    portal::propagate_owner(&portal, uid, "[data-rs-modal-overlay],[data-rs-modal-content]");
    portal::portal_nodes(uid, "data-rs-modal-overlay", "data-rs-modal-content");

Overlays com portal devem mover para body antes de abrir.
Owner propagado via data-rs-owner para queries subsequentes.

---

## runtime/positioning

    use crate::runtime::positioning;

    positioning::auto_side(&root, "[data-rs-popover-content]");

Le data-rs-side preferido do SSR.
Faz flip apenas se nao ha espaco suficiente.
Sides: top bottom left right.
Nao usar em overlays modais — apenas popover, tooltip, dropdown, hover-card.

---

## runtime/events

Re-exporta canonrs-interactions-core::behavior::events.
Usar para dispatch de CustomEvent nos overlays.

---

## Padrão de init — overlay modal

    pub fn init(root: web_sys::Element) {
        use canonrs_interactions_core::dom::lifecycle;
        use crate::runtime::{stack, transition, focus, inert, portal};

        if !lifecycle::init_guard(&root) { return; }

        let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

        stack::ensure_global_listeners();

        let portal_el = portal::portal_of(&root, "data-rs-modal-portal", &uid);
        if let Some(ref p) = portal_el { portal::move_to_body(p, &uid); }

        let (overlay, content) = portal::portal_nodes(&uid, "data-rs-modal-overlay", "data-rs-modal-content");

        stack::register_click(&uid, move |target| {
            // logica de open/close
        });

        stack::register_keydown(&uid, move |e| {
            if e.key() == "Escape" { /* fechar */ }
            focus::trap_tab(e, &content.as_ref().unwrap());
        });
    }

---

## Padrão de init — overlay nao-modal

    pub fn init(root: web_sys::Element) {
        use canonrs_interactions_core::dom::{lifecycle, state};
        use canonrs_interactions_core::behavior::outside;
        use crate::runtime::{stack, positioning};

        if !lifecycle::init_guard(&root) { return; }

        if stack::has_modal_open() { return; }

        outside::register_click_outside("[data-rs-dropdown-menu]", |el| {
            state::close(el);
        });

        positioning::auto_side(&root, "[data-rs-dropdown-menu-content]");
    }

---

## Proibido

- Aplicar inert em overlays nao-modais
- Registrar listeners de click/keydown fora do stack registry
- Abrir overlay nao-modal quando has_modal_open() == true
- Mover para body sem propagate_owner
- Fechar sem stack::pop e inert::set_inert_background(false)
- Focar elemento sem salvar e restaurar foco anterior
