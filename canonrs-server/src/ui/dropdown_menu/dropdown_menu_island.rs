//! DropdownMenu Island — Canon Rule init (DOM-driven)
use leptos::prelude::*;
use super::dropdown_menu_ui::{DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem};
use canonrs_core::meta::DisabledState;

#[island]
pub fn DropdownMenuIsland(
    children: Children,
    #[prop(optional, into)] trigger_label: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::Element = (*root_html).clone().unchecked_into();
        if root.has_attribute("data-rs-attached") { return; }
        let _ = root.set_attribute("data-rs-attached", "1");

        let add = |el: &web_sys::Element, t: &str| {
            let c = el.get_attribute("data-rs-state").unwrap_or_default();
            if c.split_whitespace().any(|s| s == t) { return; }
            let _ = el.set_attribute("data-rs-state", &format!("{} {}", c, t).trim().to_string());
        };
        let remove = |el: &web_sys::Element, t: &str| {
            let c = el.get_attribute("data-rs-state").unwrap_or_default();
            let _ = el.set_attribute("data-rs-state", &c.split_whitespace().filter(|s| *s != t).collect::<Vec<_>>().join(" "));
        };

        { let rc = root.clone();
          let add = add.clone(); let remove = remove.clone();
          let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
            if t.closest("[data-rs-dropdown-menu-trigger]").ok().flatten().is_some() {
                e.stop_propagation();
                let open = rc.get_attribute("data-rs-state").unwrap_or_default().contains("open");
                if open { remove(&rc, "open"); add(&rc, "closed"); } else { remove(&rc, "closed"); add(&rc, "open"); }
            }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget(); }

        { let rc = root.clone();
          let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
            if rc.contains(Some(&t)) { return; }
            let c = rc.get_attribute("data-rs-state").unwrap_or_default();
            let next = c.split_whitespace().filter(|s| *s != "open").collect::<Vec<_>>().join(" ");
            let _ = rc.set_attribute("data-rs-state", &format!("{} closed", next).trim().to_string());
        }));
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        }
        cb.forget(); }
    });

    view! {
        <DropdownMenu class=class.clone().unwrap_or_default() node_ref=node_ref>
            <DropdownMenuTrigger>{trigger_label.unwrap_or_else(|| "Options".to_string())}</DropdownMenuTrigger>
            <DropdownMenuContent>{children()}</DropdownMenuContent>
        </DropdownMenu>
    }
}

#[component]
pub fn DropdownMenuItemIsland(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
) -> impl IntoView {
    view! { <DropdownMenuItem disabled=disabled>{children()}</DropdownMenuItem> }
}
