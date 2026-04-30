//! Form helpers — native form integration for selection components
//!
//! Qualquer componente com data-rs-name usa estas funções
//! para manter um <input type="hidden"> sincronizado com o valor selecionado.
//! CR-XXX — Form Components Must Be Native-Compatible

use wasm_bindgen::JsCast;
use web_sys::Element;

/// Sincroniza o valor de um componente de seleção com um hidden input no DOM.
/// - Cria o input se não existir (1 por root, garantido por data-rs-hidden)
/// - Atualiza o value se já existir
/// - No-op se data-rs-name não estiver presente ou for vazio
pub fn sync_hidden_input(root: &Element, value: &str) {
    let name = match root.get_attribute("data-rs-name") {
        Some(n) if !n.is_empty() => n,
        _ => return,
    };
    let doc = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };
    // reutiliza se já existe
    if let Ok(Some(el)) = root.query_selector("input[data-rs-hidden]") {
        if let Ok(input) = el.dyn_into::<web_sys::HtmlInputElement>() {
            input.set_value(value);
            return;
        }
    }
    // cria novo
    if let Ok(el) = doc.create_element("input") {
        if let Ok(input) = el.dyn_into::<web_sys::HtmlInputElement>() {
            let _ = input.set_attribute("type", "hidden");
            let _ = input.set_attribute("data-rs-hidden", "");
            let _ = input.set_attribute("name", &name);
            input.set_value(value);
            let _ = root.append_child(input.unchecked_ref());
        }
    }
}

/// Remove o hidden input do root (cleanup no unmount).
pub fn remove_hidden_input(root: &Element) {
    if let Ok(Some(el)) = root.query_selector("input[data-rs-hidden]") {
        let _ = root.remove_child(&el);
    }
}
