//! Banner Island — Canon Rule #341
//! DOM-driven, zero state. Lógica via web_sys + Effect.

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum BannerIslandVariant {
    #[default] Info,
    Success, Warning, Error,
}

impl BannerIslandVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Info    => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error   => "error",
        }
    }
}

#[island]
pub fn BannerIsland(
    #[prop(optional, into)] content:                         Option<String>,
    #[prop(optional)] variant:    Option<BannerIslandVariant>,
    #[prop(optional)] dismissible: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let variant     = variant.unwrap_or_default();
    let dismissible = dismissible.unwrap_or(true);
    let class       = class.unwrap_or_default();

    let node_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::Element = (*root_html).clone().unchecked_into();

        if root.has_attribute("data-rs-attached") { return; }
        let _ = root.set_attribute("data-rs-attached", "1");

        if let Ok(Some(btn)) = root.query_selector("[data-rs-banner-close]") {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
                let _ = root_cb.set_attribute("data-rs-state", "closed");
                if let Ok(el) = root_cb.clone().dyn_into::<web_sys::HtmlElement>() {
                    el.set_hidden(true);
                }
            }));
            let _ = btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    });

    view! {
        <div
            data-rs-banner=""
            data-rs-component="Banner"
            data-rs-variant=variant.as_str()
            data-rs-state="open"
            role="status"
            aria-live="polite"
            class=class
            node_ref=node_ref
        >
            <div data-rs-banner-content="">{content}</div>
            {dismissible.then(|| view! {
                <button
                    type="button"
                    data-rs-banner-close=""
                    aria-label="Close banner"
                >"×"</button>
            })}
        </div>
    }
}
