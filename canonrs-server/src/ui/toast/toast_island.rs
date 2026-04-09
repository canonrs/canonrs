//! Toast Island — Canon Rule #341 DOM-driven
use leptos::prelude::*;
use canonrs_core::primitives::ToastVariant;

#[island]
pub fn ToastIsland(
    #[prop(optional, into)] title:       Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] variant: Option<ToastVariant>,
    #[prop(optional)] duration_ms: Option<u32>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let variant     = variant.unwrap_or_default();
    let duration_ms = duration_ms.unwrap_or(5000);
    let class       = class.unwrap_or_default();
    let node_ref    = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::Element = (*root_html).clone().unchecked_into();
        if root.has_attribute("data-rs-attached") { return; }
        let _ = root.set_attribute("data-rs-attached", "1");

        if let Ok(Some(btn)) = root.query_selector("[data-rs-toast-close]") {
            let rc = root.clone();
            let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
                let _ = rc.set_attribute("data-rs-state", "closed");
                if let Ok(el) = rc.clone().dyn_into::<web_sys::HtmlElement>() { el.set_hidden(true); }
            }));
            let _ = btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
            cb.forget();
        }

        {
            let rc = root.clone();
            let cb = Closure::once_into_js(move || {
                let _ = rc.set_attribute("data-rs-state", "closed");
                if let Ok(el) = rc.clone().dyn_into::<web_sys::HtmlElement>() { el.set_hidden(true); }
            });
            web_sys::window().unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(cb.unchecked_ref(), duration_ms as i32).ok();
        }
    });

    view! {
        <div
            data-rs-toast=""
            data-rs-component="Toast"
            data-rs-variant=variant.as_str()
            data-rs-state="open"
            role="status"
            aria-live="polite"
            class=class
            node_ref=node_ref
        >
            <div data-rs-toast-content="">
                {title.map(|t| view! { <p data-rs-toast-title="">{t}</p> })}
                {description.map(|d| view! { <p data-rs-toast-description="">{d}</p> })}
            </div>
            <button type="button" data-rs-toast-close="" aria-label="Close">"×"</button>
        </div>
    }
}

#[island]
pub fn ToastViewportIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    view! {
        <div data-rs-toast-viewport="" aria-label="Notifications" class=class>
            {children()}
        </div>
    }
}
