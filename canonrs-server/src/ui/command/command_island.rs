//! Command Island — Canon Rule init (DOM-driven)
use leptos::prelude::*;

#[island]
pub fn CommandIsland(
    children: Children,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] empty_text: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class       = class.unwrap_or_default();
    let placeholder = placeholder.unwrap_or_else(|| "Search...".to_string());
    let _empty_text = empty_text.unwrap_or_else(|| "No results found.".to_string());
    let (query, set_query) = signal(String::new());

    #[cfg(feature = "hydrate")]
    let on_input = move |e: leptos::ev::Event| {
        use leptos::wasm_bindgen::JsCast;
        let val = e.target()
            .and_then(|t| t.dyn_into::<leptos::web_sys::HtmlInputElement>().ok())
            .map(|el| el.value())
            .unwrap_or_default();
        set_query.set(val);
    };
    #[cfg(not(feature = "hydrate"))]
    let on_input = move |_: leptos::ev::Event| {};

    let _ = (query, set_query);

    view! {
        <div data-rs-command="" data-rs-component="Command" class=class>
            <div data-rs-command-input-wrapper="">
                <input
                    data-rs-command-input=""
                    type="text"
                    placeholder=placeholder
                    on:input=on_input
                />
            </div>
            <div data-rs-command-list="" role="listbox">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn CommandItemIsland(
    children: Children,
    #[prop(into, default = String::new())] value: String,
    #[prop(optional, into)] group: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-command-item=""
            role="option"
            data-rs-value=value
            data-rs-group=group.unwrap_or_default()
        >
            {children()}
        </div>
    }
}
