//! Disclosure — open/close com suporte a single/multiple e collapsible
use web_sys::Element;
use crate::dom::{state, query};
use crate::integration::aria;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SelectionMode {
    Single,
    Multiple,
}

#[derive(Debug)]
pub struct DisclosureConfig {
    /// Selector dos itens disclosure dentro do root
    pub item_selector: &'static str,
    /// Selector do trigger dentro do item
    pub trigger_selector: &'static str,
    /// Modo de seleção
    pub mode: SelectionMode,
    /// Se false, item aberto não pode ser fechado clicando nele mesmo
    pub collapsible: bool,
}

pub fn open_item(item: &Element, trigger_selector: &str) {
    state::open(item);
    if let Some(trigger) = query::first(item, trigger_selector) {
        aria::set_expanded(&trigger, true);
    }
}

pub fn close_item(item: &Element, trigger_selector: &str) {
    state::close(item);
    if let Some(trigger) = query::first(item, trigger_selector) {
        aria::set_expanded(&trigger, false);
    }
}

pub fn toggle(root: &Element, item: &Element, config: &DisclosureConfig) {
    let currently_open = state::is_open(item);

    if currently_open {
        if config.collapsible {
            close_item(item, config.trigger_selector);
        }
    } else {
        if config.mode == SelectionMode::Single {
            for other in query::all(root, config.item_selector) {
                if state::is_open(&other) {
                    close_item(&other, config.trigger_selector);
                }
            }
        }
        open_item(item, config.trigger_selector);
    }
}

/// Retorna triggers ativos (não-disabled) na ordem DOM
pub fn active_triggers(root: &Element, config: &DisclosureConfig) -> Vec<Element> {
    query::all(root, config.item_selector)
        .into_iter()
        .filter(|item| !state::has(item, "disabled"))
        .filter_map(|item| query::first(&item, config.trigger_selector))
        .collect()
}

/// Inicializa estado: garante aria-expanded correto em todos os triggers
pub fn init_state(root: &Element, config: &DisclosureConfig) {
    for item in query::all(root, config.item_selector) {
        let is_open = state::is_open(&item);
        if let Some(trigger) = query::first(&item, config.trigger_selector) {
            aria::set_expanded(&trigger, is_open);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SelectionMode, DisclosureConfig};

    #[test]
    fn disclosure_config_single() {
        let config = DisclosureConfig {
            item_selector:    "[data-rs-accordion-item]",
            trigger_selector: "[data-rs-accordion-trigger]",
            mode:             SelectionMode::Single,
            collapsible:      true,
        };
        assert_eq!(config.mode, SelectionMode::Single);
        assert!(config.collapsible);
        assert_eq!(config.item_selector, "[data-rs-accordion-item]");
    }

    #[test]
    fn disclosure_config_multiple() {
        let config = DisclosureConfig {
            item_selector:    "[data-rs-accordion-item]",
            trigger_selector: "[data-rs-accordion-trigger]",
            mode:             SelectionMode::Multiple,
            collapsible:      false,
        };
        assert_eq!(config.mode, SelectionMode::Multiple);
        assert!(!config.collapsible);
    }

    #[test]
    fn selection_mode_equality() {
        assert_eq!(SelectionMode::Single,   SelectionMode::Single);
        assert_eq!(SelectionMode::Multiple, SelectionMode::Multiple);
        assert_ne!(SelectionMode::Single,   SelectionMode::Multiple);
    }
}
