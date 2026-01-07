use crate::primitives::sidebar::{SidebarContext, CollapsibleMode, SidebarVariant, SidebarSide};
use leptos::prelude::*;

/// Sidebar Features - Canon Type 3 (Interactive)
///
/// Keyboard shortcuts e persistência

const SIDEBAR_KEYBOARD_SHORTCUT: &str = "b";
const SIDEBAR_STORAGE_KEY: &str = "sidebar:state";

// ═══════════════════════════════════════════════════════════
// ENHANCED PROVIDER (com keyboard + storage)
// ═══════════════════════════════════════════════════════════

#[component]
pub fn SidebarProviderEnhanced(
    children: ChildrenFn,
    #[prop(default = true)] default_open: bool,
    #[prop(default = false)] is_mobile: bool,
    #[prop(default = CollapsibleMode::Offcanvas)] collapsible: CollapsibleMode,
    #[prop(default = SidebarVariant::Sidebar)] variant: SidebarVariant,
    #[prop(default = SidebarSide::Left)] side: SidebarSide,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let wrapper_class = StoredValue::new(format!(
        "group/sidebar-wrapper flex min-h-svh w-full {}",
        class
    ));

    // Cria context
    let ctx = SidebarContext::new(default_open, is_mobile, collapsible, variant, side);

    // Carrega estado inicial do localStorage
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::window;

        if let Some(win) = window() {
            if let Some(storage) = win.local_storage().ok().flatten() {
                if let Ok(Some(saved_state)) = storage.get_item(SIDEBAR_STORAGE_KEY) {
                    let is_open = saved_state == "true";
                    ctx.open.set(is_open);
                }
            }
        }
    }

    // Keyboard shortcut (Cmd/Ctrl + B)
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        use wasm_bindgen::JsCast;
        use web_sys::KeyboardEvent;

        let handle_keydown = wasm_bindgen::closure::Closure::wrap(Box::new(move |event: KeyboardEvent| {
            if event.key() == SIDEBAR_KEYBOARD_SHORTCUT && (event.meta_key() || event.ctrl_key()) {
                event.prevent_default();
                ctx.toggle();
            }
        }) as Box<dyn Fn(_)>);

        if let Some(window) = web_sys::window() {
            let _ = window.add_event_listener_with_callback(
                "keydown",
                handle_keydown.as_ref().unchecked_ref()
            );
        }

        handle_keydown.forget();
    });

    // Persistência (localStorage)
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        use web_sys::window;

        if let Some(win) = window() {
            if let Some(storage) = win.local_storage().ok().flatten() {
                // Salva estado quando muda
                let state_str = if ctx.open.get() { "true" } else { "false" };
                let _ = storage.set_item(SIDEBAR_STORAGE_KEY, state_str);
            }
        }
    });

    provide_context(ctx);

    view! {
        <div
            class=wrapper_class.get_value()
            style="--sidebar-width: 16rem; --sidebar-width-icon: 3rem; --sidebar-width-mobile: 18rem;"
        >
            {children()}
        </div>
    }
}

// ═══════════════════════════════════════════════════════════
// HELPER: use_sidebar hook
// ═══════════════════════════════════════════════════════════

pub fn use_sidebar() -> SidebarContext {
    expect_context::<SidebarContext>()
}
