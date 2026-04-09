//! NavigationMenu Island — Canon Rule init (DOM-driven)
use leptos::prelude::*;

#[island]
pub fn NavigationMenuIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Nav>::new();

    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::HtmlElement = (*root_html).clone().unchecked_into();
        if root.has_attribute("data-rs-attached") { return; }
        let _ = root.set_attribute("data-rs-attached", "1");

        { let rc = root.clone();
          let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            e.stop_propagation();
            let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
            let Some(trigger) = t.closest("[data-rs-navigation-menu-trigger]").ok().flatten() else { return };
            let Some(item) = trigger.closest("[data-rs-navigation-menu-item]").ok().flatten() else { return };
            let is_open = item.get_attribute("data-rs-state").as_deref() == Some("open");
            let mut idx = 0u32;
            loop {
                match rc.query_selector(&format!("[data-rs-navigation-menu-item]:nth-of-type({})", idx+1)) {
                    Ok(Some(el)) => {
                        let _ = el.set_attribute("data-rs-state", "closed");
                        if let Ok(Some(c)) = el.query_selector("[data-rs-navigation-menu-content]") {
                            if let Ok(h) = c.dyn_into::<web_sys::HtmlElement>() { h.set_hidden(true); }
                        }
                        idx += 1;
                    }
                    _ => break,
                }
            }
            if !is_open {
                let _ = item.set_attribute("data-rs-state", "open");
                if let Ok(Some(c)) = item.query_selector("[data-rs-navigation-menu-content]") {
                    if let Ok(h) = c.dyn_into::<web_sys::HtmlElement>() { h.set_hidden(false); }
                }
            }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget(); }

        { let rc = root.clone();
          let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
            if rc.contains(Some(&t)) { return; }
            let mut idx = 0u32;
            loop {
                match rc.query_selector(&format!("[data-rs-navigation-menu-item]:nth-of-type({})", idx+1)) {
                    Ok(Some(el)) => { let _ = el.set_attribute("data-rs-state", "closed"); idx += 1; }
                    _ => break,
                }
            }
        }));
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        }
        cb.forget(); }
    });

    view! {
        <nav
            data-rs-navigation-menu=""
            data-rs-component="NavigationMenu"
            class=class.unwrap_or_default()
            node_ref=node_ref
        >
            <ul data-rs-navigation-menu-list="">
                {children()}
            </ul>
        </nav>
    }
}

#[component]
pub fn NavigationMenuItemIsland(
    children: Children,
) -> impl IntoView {
    view! { <li data-rs-navigation-menu-item="" data-rs-state="closed">{children()}</li> }
}

#[component]
pub fn NavigationMenuTriggerIsland(
    children: Children,
) -> impl IntoView {
    view! {
        <button type="button" data-rs-navigation-menu-trigger="" aria-expanded="false" aria-haspopup="true">
            {children()}
        </button>
    }
}

#[component]
pub fn NavigationMenuContentIsland(
    children: Children,
) -> impl IntoView {
    view! {
        <div data-rs-navigation-menu-content="" data-rs-state="closed" hidden=true>
            <ul>{children()}</ul>
        </div>
    }
}

#[component]
pub fn NavigationMenuLinkIsland(
    children: Children,
    #[prop(into, default = String::new())] href: String,
) -> impl IntoView {
    view! { <li data-rs-navigation-menu-sub-item=""><a data-rs-navigation-menu-link="" href=href>{children()}</a></li> }
}
