//! Dialog interaction helpers

/// Fecha um dialog pelo uid.
/// Chamado após ação async concluir com sucesso.
/// SSR-safe: no-op no servidor.
pub fn dialog_close(uid: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(win) = leptos::web_sys::window() {
            if let Some(doc) = win.document() {
                let sel = format!("[data-rs-dialog][data-rs-uid='{}']", uid);
                if let Ok(Some(root)) = doc.query_selector(&sel) {
                    if let Ok(event) = leptos::web_sys::CustomEvent::new("rs:dialog:close") {
                        let _ = root.dispatch_event(&event);
                    }
                }
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = uid;
}

/// Fecha um confirm dialog pelo uid.
/// SSR-safe: no-op no servidor.
pub fn confirm_dialog_close(uid: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(win) = leptos::web_sys::window() {
            if let Some(doc) = win.document() {
                let sel = format!("[data-rs-confirm-dialog][data-rs-uid='{}']", uid);
                if let Ok(Some(root)) = doc.query_selector(&sel) {
                    if let Ok(event) = leptos::web_sys::CustomEvent::new("rs:confirm-dialog:close") {
                        let _ = root.dispatch_event(&event);
                    }
                }
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = uid;
}

/// Reseta um input pelo name.
/// SSR-safe: no-op no servidor.
pub fn input_reset(name: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(win) = leptos::web_sys::window() {
            if let Some(doc) = win.document() {
                let sel = format!("[data-rs-input][name='{}']", name);
                if let Ok(Some(el)) = doc.query_selector(&sel) {
                    if let Ok(event) = leptos::web_sys::CustomEvent::new("rs:input:reset") {
                        let _ = el.dispatch_event(&event);
                    }
                }
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = name;
}
