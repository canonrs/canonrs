//! TableOfContents Init — scroll spy via IntersectionObserver, auto-expand nested

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    let mode = root.get_attribute("data-rs-mode").unwrap_or_default();

    // hover no root — mostra scrollbar
    {
        let r_enter = root.clone();
        let enter = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            state::add_state(&r_enter, "hover");
        });
        let r_leave = root.clone();
        let leave = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let link = if t.has_attribute("data-rs-toc-link") { Some(t) } else { t.closest("[data-rs-toc-link]").ok().flatten() };
            if let Some(l) = link { state::remove_state(&l, "hover"); }
            let _ = &r_leave;
        });
        let _ = root.add_event_listener_with_callback("mouseenter", enter.as_ref().unchecked_ref());
        let _ = root.add_event_listener_with_callback("mouseleave", leave.as_ref().unchecked_ref());
        enter.forget();
        leave.forget();
    }

    // hover nos links — delegado no root para funcionar com filhos dinâmicos (nested/subtree)
    {
        let r_enter = root.clone();
        let enter = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let link = if t.has_attribute("data-rs-toc-link") {
                Some(t.clone())
            } else {
                t.closest("[data-rs-toc-link]").ok().flatten()
                    .or_else(|| if t.has_attribute("data-rs-toc-link") { Some(t) } else { None })
            };
            if let Some(l) = link {
                for other in query::all(&r_enter, "[data-rs-toc-link]") {
                    state::remove_state(&other, "hover");
                }
                state::add_state(&l, "hover");
            }
        });
        let _r_leave = root.clone();
        let leave = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let link = if t.has_attribute("data-rs-toc-link") {
                Some(t.clone())
            } else {
                t.closest("[data-rs-toc-link]").ok().flatten()
                    .or_else(|| if t.has_attribute("data-rs-toc-link") { Some(t) } else { None })
            };
            if let Some(l) = link {
                state::remove_state(&l, "hover");
            }
        });
        let _ = root.add_event_listener_with_callback("mouseover", enter.as_ref().unchecked_ref());
        let _ = root.add_event_listener_with_callback("mouseout", leave.as_ref().unchecked_ref());
        enter.forget();
        leave.forget();
    }

    // click nos links — smooth scroll no ScrollArea viewport ou window
    let scroll_viewport = {
        let doc = web_sys::window().and_then(|w| w.document());
        doc.and_then(|d| d.query_selector("[data-rs-scroll-viewport]").ok().flatten())
    };
    for link in query::all(&root, "[data-rs-toc-link]") {
        let vp = scroll_viewport.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            e.prevent_default();
            let Some(t) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let href = t.get_attribute("href").unwrap_or_default();
            if href.starts_with('#') {
                let id = &href[1..];
                let win = match web_sys::window() { Some(w) => w, None => return };
                let doc = match win.document() { Some(d) => d, None => return };
                if let Ok(Some(target)) = doc.query_selector(&format!("#{}", id)) {
                    if let Some(ref viewport) = vp {
                        // scroll dentro do ScrollArea
                        let target_rect = target.get_bounding_client_rect();
                        let vp_rect = viewport.get_bounding_client_rect();
                        let current_scroll = viewport.scroll_top() as f64;
                        let top = current_scroll + target_rect.top() - vp_rect.top() - 80.0;
                        let opts = web_sys::ScrollToOptions::new();
                        opts.set_top(top);
                        opts.set_behavior(web_sys::ScrollBehavior::Smooth);
                        viewport.scroll_to_with_scroll_to_options(&opts);
                    } else {
                        // fallback window scroll
                        let rect = target.get_bounding_client_rect();
                        let scroll_y = win.scroll_y().unwrap_or(0.0);
                        let top = rect.top() + scroll_y - 80.0;
                        let opts = web_sys::ScrollToOptions::new();
                        opts.set_top(top);
                        opts.set_behavior(web_sys::ScrollBehavior::Smooth);
                        win.scroll_to_with_scroll_to_options(&opts);
                    }
                }
            }
        });
        let _ = link.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // expand mode — controlado pelo scroll spy, não por click
    // a lógica de expand é executada dentro do on_active callback

    // nested mode — puramente scroll-driven, chevron é decorativo (indicador de estado)
    // sem click manual — estado deriva do scroll

    // scroll spy via IntersectionObserver — arquitetura enterprise
    // TOC é projeção do estado do documento, não tem estado próprio
    {
        let heading_ids: Vec<String> = query::all(&root, "[data-rs-toc-item][data-rs-target]")
            .iter()
            .filter_map(|el| el.get_attribute("data-rs-target"))
            .filter(|id| !id.is_empty())
            .collect();

        if heading_ids.is_empty() { return; }

        let doc = match web_sys::window().and_then(|w| w.document()) {
            Some(d) => d, None => return,
        };

        // estado inicial — ativa o primeiro item imediatamente
        {
            let first_id = &heading_ids[0];
            let selector = format!("[data-rs-toc-item][data-rs-target='{}']", first_id);
            if let Some(item) = query::first(&root, &selector) {
                state::add_state(&item, "active");
            }
        }

        // mapa de visibilidade — mantido fora do closure
        // key: id do heading, value: boundingClientRect.top no momento que entrou
        let visible: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<String, f64>>> =
            std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashMap::new()));
        let visible_cb = visible.clone();

        let heading_ids_obs = heading_ids.clone();
        let root_spy = root.clone();
        let mode_spy = mode.clone();

        let on_active = Closure::<dyn Fn(js_sys::Array)>::new(move |entries: js_sys::Array| {
            // atualiza mapa de visibilidade com as entradas que mudaram
            for entry in entries.iter() {
                let entry = match entry.dyn_into::<web_sys::IntersectionObserverEntry>() {
                    Ok(e) => e, Err(_) => continue,
                };
                let id = match entry.target().get_attribute("id") {
                    Some(id) => id, None => continue,
                };
                if entry.is_intersecting() {
                    let top = entry.bounding_client_rect().top();
                    visible_cb.borrow_mut().insert(id, top);
                } else {
                    visible_cb.borrow_mut().remove(&id);
                }
            }

            // pega o heading visível mais próximo do topo da viewport
            // ordena pelos ids originais para manter ordem do documento
            let active_id = {
                let map = visible_cb.borrow();
                if map.is_empty() {
                    // fallback — nenhum visível, pega o último que passou pelo topo
                    let win = match web_sys::window() { Some(w) => w, None => return };
                    let doc = match win.document() { Some(d) => d, None => return };
                    let threshold = 120.0_f64;
                    let mut closest: Option<(f64, String)> = None;
                    for id in &heading_ids_obs {
                        if let Ok(Some(el)) = doc.query_selector(&format!("#{}", id)) {
                            let top = el.get_bounding_client_rect().top();
                            if top <= threshold {
                                match &closest {
                                    None => { closest = Some((top, id.clone())); }
                                    Some((ct, _)) => {
                                        if top > *ct { closest = Some((top, id.clone())); }
                                    }
                                }
                            }
                        }
                    }
                    // se nenhum passou pelo topo, assume o primeiro item
                    match closest.map(|(_, id)| id) {
                        Some(id) => Some(id),
                        None => heading_ids_obs.first().cloned(),
                    }
                } else {
                    // pega o visível com menor top (mais próximo do topo)
                    heading_ids_obs.iter()
                        .filter(|id| map.contains_key(*id))
                        .next()
                        .cloned()
                }
            };

            let Some(id) = active_id else { return };

            // reset todos os itens
            for item in query::all(&root_spy, "[data-rs-toc-item]") {
                item.set_attribute("data-rs-state", "idle").ok();
            }

            // marca ativo
            let selector = format!("[data-rs-toc-item][data-rs-target='{}']", id);
            let Some(active_item) = query::first(&root_spy, &selector) else { return };
            state::add_state(&active_item, "active");

            // expand — mostra filhos do item ativo, esconde filhos de outros pais
            if mode_spy == "expand" {
                let all_items = query::all(&root_spy, "[data-rs-toc-item]");
                // encontra o nível e target do item ativo
                let active_level = active_item.get_attribute("data-rs-level")
                    .unwrap_or_default().parse::<u32>().unwrap_or(1);
                let active_target = active_item.get_attribute("data-rs-target").unwrap_or_default();

                // sobe para encontrar o pai direto (level 1) do item ativo
                let parent_target = if active_level > 1 {
                    // acha o item de level 1 mais próximo antes do ativo na lista
                    let mut found_active = false;
                    let mut parent: Option<String> = None;
                    for item in all_items.iter().rev() {
                        if !found_active {
                            if item.get_attribute("data-rs-target").as_deref() == Some(&active_target) {
                                found_active = true;
                            }
                            continue;
                        }
                        let lvl = item.get_attribute("data-rs-level")
                            .unwrap_or_default().parse::<u32>().unwrap_or(1);
                        if lvl < active_level {
                            parent = item.get_attribute("data-rs-target");
                            break;
                        }
                    }
                    parent.unwrap_or(active_target.clone())
                } else {
                    active_target.clone()
                };

                // mostra filhos do pai ativo, esconde todos os outros filhos
                let mut in_parent = false;
                let mut parent_level = 1u32;
                for item in &all_items {
                    let target = item.get_attribute("data-rs-target").unwrap_or_default();
                    let level = item.get_attribute("data-rs-level")
                        .unwrap_or_default().parse::<u32>().unwrap_or(1);
                    let is_child = item.get_attribute("data-rs-child").as_deref() == Some("true");

                    if target == parent_target {
                        in_parent = true;
                        parent_level = level;
                        continue;
                    }

                    if is_child {
                        if in_parent && level > parent_level {
                            state::add_state(item, "visible");
                        } else {
                            state::remove_state(item, "visible");
                        }
                    } else if level <= parent_level && target != parent_target {
                        in_parent = false;
                    }
                }
            }

            // nested — abre ancestrais e colapsa outros
            if mode_spy == "nested" {
                // coleta todos os subtrees e fecha todos
                for subtree in query::all(&root_spy, "[data-rs-toc-subtree]") {
                    state::remove_state(&subtree, "open");
                    state::add_state(&subtree, "closed");
                }
                for btn in query::all(&root_spy, "[data-rs-toc-expand-btn]") {
                    state::remove_state(&btn, "expanded");
                }

                // sobe o DOM a partir do item ativo abrindo ancestrais
                let mut current = active_item.parent_element();
                while let Some(el) = current {
                    // se encontrou um subtree, abre ele
                    if el.has_attribute("data-rs-toc-subtree") {
                        state::remove_state(&el, "closed");
                        state::add_state(&el, "open");
                        // acha o botão irmão e marca como expanded
                        if let Some(parent_item) = el.parent_element() {
                            if let Some(btn) = query::first(&parent_item, "[data-rs-toc-expand-btn]") {
                                state::add_state(&btn, "expanded");
                            }
                            // marca o item pai como ancestor
                            if parent_item.has_attribute("data-rs-toc-item") {
                                state::add_state(&parent_item, "ancestor");
                            }
                        }
                    }
                    current = el.parent_element();
                    // para quando chega no root
                    if current.as_ref().map(|e| e == &root_spy).unwrap_or(false) {
                        break;
                    }
                }
            }
        });

        // IntersectionObserver options
        // busca explicitamente o center com scroll-viewport
        let scroll_root = doc
            .query_selector("[data-rs-main-viewport] [data-rs-scroll-viewport]")
            .ok()
            .flatten()
            .or_else(|| doc.query_selector("[data-rs-region='center'] [data-rs-scroll-viewport]").ok().flatten())
            .or_else(|| doc.query_selector("[data-rs-scroll-viewport]").ok().flatten());

        let options = web_sys::IntersectionObserverInit::new();
        options.set_threshold(&JsValue::from_f64(0.0));
        if let Some(ref sr) = scroll_root {
            options.set_root(Some(sr));
            options.set_root_margin("0px 0px -40% 0px");
        } else {
            options.set_root_margin("0px 0px -40% 0px");
        }

        let observer = match web_sys::IntersectionObserver::new_with_options(
            on_active.as_ref().unchecked_ref(),
            &options,
        ) {
            Ok(o) => o, Err(_) => return,
        };

        // observa todos os headings
        for id in &heading_ids {
            if let Ok(Some(el)) = doc.query_selector(&format!("#{}", id)) {
                observer.observe(&el);
            }
        }

        on_active.forget();
        // mantém observer vivo
        Box::leak(Box::new(observer));
    }
}
