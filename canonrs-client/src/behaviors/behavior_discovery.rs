//! @canon-level: strict
//! BehaviorDiscovery — conecta data-rs-behavior (Rust core) ao behavior engine (JS)
//! Fecha o contrato entre BehaviorEngine (core) e os behaviors registrados (client)

#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;

/// Mapeia data-rs-behavior → seletor do behavior registrado
/// Sincronizado com canonrs_core::behavior_engine::component_behaviors()
#[cfg(feature = "hydrate")]
pub fn behavior_selector(behavior: &str) -> Option<&'static str> {
    match behavior {
        "overlay"    => Some("data-rs-dialog, data-rs-drawer, data-rs-sheet, data-rs-modal"),
        "selection"  => Some("data-rs-select, data-rs-combobox, data-rs-radio-group"),
        "navigation" => Some("data-rs-tabs, data-rs-accordion, data-rs-navigation-menu"),
        "resize"     => Some("data-rs-resizable"),
        "toggle"     => Some("data-rs-toggle, data-rs-switch"),
        "disclosure" => Some("data-rs-collapsible, data-rs-accordion"),
        _            => None,
    }
}

/// Escaneia DOM por data-rs-behavior e loga componentes não inicializados
/// Útil em dev para detectar components sem behavior registrado
#[cfg(feature = "hydrate")]
pub fn audit_behaviors() {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let document = match window.document() {
        Some(d) => d,
        None => return,
    };

    let behaviors = ["overlay", "selection", "navigation", "resize", "toggle", "disclosure"];

    for behavior in behaviors {
        let selector = format!("[data-rs-behavior='{}']", behavior);
        if let Ok(nodes) = document.query_selector_all(&selector) {
            for i in 0..nodes.length() {
                if let Some(node) = nodes.item(i) {
                    if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                        let initialized = el.has_attribute("data-rs-initialized");
                        if !initialized {
                            web_sys::console::warn_1(
                                &format!(
                                    "[CanonRS] Component with data-rs-behavior='{}' not initialized: {:?}",
                                    behavior,
                                    el.tag_name()
                                ).into()
                            );
                        }
                    }
                }
            }
        }
    }
}

#[cfg(not(feature = "hydrate"))]
pub fn audit_behaviors() {}
#[cfg(not(feature = "hydrate"))]
pub fn behavior_selector(_: &str) -> Option<&'static str> { None }

/// Escaneia DOM por data-rs-component e injeta data-rs-behavior automaticamente
/// Fecha o contrato: Rust define behaviors via BehaviorEngine, JS os lê via DOM
#[cfg(feature = "hydrate")]
pub fn inject_behaviors_from_meta() {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let document = match window.document() {
        Some(d) => d,
        None => return,
    };

    if let Ok(nodes) = document.query_selector_all("[data-rs-component]") {
        for i in 0..nodes.length() {
            if let Some(node) = nodes.item(i) {
                if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                    if el.has_attribute("data-rs-behavior") { continue; }
                    if let Some(name) = el.get_attribute("data-rs-component") {
                        let behavior = match name.as_str() {
                            "Dialog" | "Drawer" | "Sheet" | "Modal" => "overlay",
                            "Select" | "Combobox"                   => "selection",
                            "Tabs" | "Accordion" | "NavigationMenu" => "navigation",
                            "Resizable"                             => "resize",
                            "Toggle" | "Switch"                     => "toggle",
                            "Collapsible"                           => "disclosure",
                            _                                       => continue,
                        };
                        el.set_attribute("data-rs-behavior", behavior).ok();
                    }
                }
            }
        }
    }
}

#[cfg(not(feature = "hydrate"))]
pub fn inject_behaviors_from_meta() {}
