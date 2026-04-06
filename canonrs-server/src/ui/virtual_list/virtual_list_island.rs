use leptos::prelude::*;

#[island]
pub fn VirtualListIsland(
    items_count: usize,
    #[prop(optional)] item_height: Option<f64>,
    #[prop(optional, into)] item_template: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let item_height  = item_height.unwrap_or(40.0);
    let class        = class.unwrap_or_default();
    let template     = item_template.unwrap_or_else(|| "Item".to_string());
    let total_height = items_count as f64 * item_height;

    let scroll_top = RwSignal::new(0.0f64);
    let viewport_h = RwSignal::new(400.0f64);

    let on_scroll = move |e: leptos::ev::Event| {
        use leptos::wasm_bindgen::JsCast;
        if let Some(el) = e.target()
            .and_then(|t| t.dyn_into::<leptos::web_sys::HtmlElement>().ok())
        {
            scroll_top.set(el.scroll_top() as f64);
        }
    };

    let stored_template = StoredValue::new(template);

    view! {
        <div
            data-rs-virtual-list-viewport=""
            class=class
            style="height:400px;overflow-y:auto;position:relative"
            on:scroll=on_scroll
        >
            <div
                data-rs-virtual-list-content=""
                style=format!("height:{}px;position:relative", total_height)
            >
                {move || {
                    let st    = scroll_top.get();
                    let vh    = viewport_h.get();
                    let start = ((st / item_height).floor() as usize).saturating_sub(2);
                    let end   = (((st + vh) / item_height).ceil() as usize + 2).min(items_count);
                    let tmpl  = stored_template.get_value();
                    (start..end)
                        .map(|i| {
                            let top   = i as f64 * item_height;
                            let label = format!("{} {}", tmpl, i + 1);
                            view! {
                                <div
                                    data-rs-virtual-list-item=""
                                    style=format!(
                                        "position:absolute;top:{}px;height:{}px;width:100%;display:flex;align-items:center",
                                        top, item_height
                                    )
                                >
                                    {label}
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}
