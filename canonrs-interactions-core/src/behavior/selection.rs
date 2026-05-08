//! Selection — active item management
//! Padrão: desativa todos, ativa um, sincroniza aria
use web_sys::Element;
use crate::dom::{state, query};
use crate::integration::aria;

#[derive(Debug)]
pub struct SelectionConfig {
    /// Selector dos itens selecionáveis dentro do root
    pub item_selector: &'static str,
    /// Atributo que identifica o valor do item
    pub value_attr: &'static str,
    /// Se true, sincroniza aria-selected nos triggers
    pub aria_selected: bool,
    /// Se true, sincroniza aria-current="page" nos itens
    pub aria_current: bool,
}

/// Desativa todos os itens e ativa o que tem data-rs-value == value
pub fn activate_by_value(root: &Element, value: &str, config: &SelectionConfig) {
    for item in query::all(root, config.item_selector) {
        let v = item.get_attribute(config.value_attr).unwrap_or_default();
        let is_active = v == value;
        state::remove(&item, "active");
        state::remove(&item, "inactive");
        if is_active {
            state::add(&item, "active");
            if config.aria_selected { aria::set_selected(&item, true); }
            if config.aria_current  { let _ = item.set_attribute("aria-current", "page"); }
        } else {
            state::add(&item, "inactive");
            if config.aria_selected { aria::set_selected(&item, false); }
            if config.aria_current  { let _ = item.remove_attribute("aria-current"); }
        }
    }
}

/// Desativa todos os itens e ativa o item diretamente
pub fn activate(root: &Element, target: &Element, config: &SelectionConfig) {
    for item in query::all(root, config.item_selector) {
        state::remove(&item, "active");
        state::remove(&item, "inactive");
        state::add(&item, "inactive");
        if config.aria_selected { aria::set_selected(&item, false); }
        if config.aria_current  { let _ = item.remove_attribute("aria-current"); }
    }
    state::remove(target, "inactive");
    state::add(target, "active");
    if config.aria_selected { aria::set_selected(target, true); }
    if config.aria_current  { let _ = target.set_attribute("aria-current", "page"); }
}

/// Retorna o valor do item atualmente ativo
pub fn active_value(root: &Element, config: &SelectionConfig) -> Option<String> {
    query::all(root, config.item_selector)
        .into_iter()
        .find(|el| state::has(el, "active"))
        .and_then(|el| el.get_attribute(config.value_attr))
}

/// Inicializa estado: garante que todos têm "inactive" se nenhum tem "active"
pub fn init_state(root: &Element, config: &SelectionConfig) {
    let items = query::all(root, config.item_selector);
    let has_active = items.iter().any(|el| state::has(el, "active"));
    if !has_active {
        if let Some(first) = items.first() {
            activate(root, first, config);
        }
    } else {
        for item in &items {
            if !state::has(item, "active") {
                state::remove(item, "active");
                state::add(item, "inactive");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SelectionConfig;

    #[test]
    fn selection_config_tabs() {
        let config = SelectionConfig {
            item_selector: "[data-rs-tabs-trigger]",
            value_attr:    "data-rs-value",
            aria_selected: true,
            aria_current:  false,
        };
        assert_eq!(config.item_selector, "[data-rs-tabs-trigger]");
        assert!(config.aria_selected);
        assert!(!config.aria_current);
    }

    #[test]
    fn selection_config_nav() {
        let config = SelectionConfig {
            item_selector: "[data-rs-nav-item]",
            value_attr:    "data-rs-value",
            aria_selected: false,
            aria_current:  true,
        };
        assert!(!config.aria_selected);
        assert!(config.aria_current);
    }
}
