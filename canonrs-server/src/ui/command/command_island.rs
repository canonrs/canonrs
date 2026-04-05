use leptos::prelude::*;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CommandIslandItem {
    pub label: String,
    pub value: String,
    pub group: Option<String>,
}

#[island]
pub fn CommandIsland(
    items: Vec<CommandIslandItem>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] empty_text: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let placeholder = placeholder.unwrap_or_else(|| "Search...".to_string());
    let empty_text  = empty_text.unwrap_or_else(|| "No results found.".to_string());
    let class       = class.unwrap_or_default();
    let (query, set_query) = signal(String::new());
    let _ = set_query;

    let filtered = move || {
        let q = query.get().to_lowercase();
        items.iter()
            .filter(|item| q.is_empty() || item.label.to_lowercase().contains(&q))
            .cloned()
            .collect::<Vec<_>>()
    };

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

    view! {
        <div
            data-rs-command=""
            data-rs-component="Command"
            class=class
        >
            <div data-rs-command-input-wrapper="">
                <input
                    data-rs-command-input=""
                    type="text"
                    placeholder=placeholder
                    on:input=on_input
                />
            </div>
            <div data-rs-command-list="" role="listbox">
                {move || {
                    let items = filtered();
                    if items.is_empty() {
                        view! {
                            <div data-rs-command-empty="">{empty_text.clone()}</div>
                        }.into_any()
                    } else {
                        items.into_iter().map(|item| {
                            view! {
                                <div
                                    data-rs-command-item=""
                                    role="option"
                                    data-rs-value=item.value.clone()
                                >
                                    {item.label}
                                </div>
                            }
                        }).collect_view().into_any()
                    }
                }}
            </div>
        </div>
    }
}
