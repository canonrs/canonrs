use leptos::prelude::*;

#[derive(Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct ComboboxOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

#[island]
pub fn ComboboxIsland(
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] selected_value: Option<String>,
    options: Vec<ComboboxOption>,
) -> impl IntoView {
    let placeholder  = placeholder.unwrap_or_else(|| "Search...".to_string());
    let disabled     = disabled.unwrap_or(false);
    let class        = class.unwrap_or_default();

    let (is_open,     set_open)     = signal(false);
    let (query,       set_query)    = signal(String::new());
    let (selected,    set_selected) = signal(selected_value.unwrap_or_default());
    let (focused_idx, set_focused)  = signal::<Option<usize>>(None);

    let initial_state = if disabled { "disabled closed".to_string() } else { "closed".to_string() };

    let root_state = move || {
        let mut s = vec![];
        if disabled      { s.push("disabled"); }
        if is_open.get() { s.push("open"); } else { s.push("closed"); }
        s.join(" ")
    };

    let input_value = {
        let options = options.clone();
        move || {
            let val = selected.get();
            if val.is_empty() { return String::new(); }
            options.iter().find(|o| o.value == val).map(|o| o.label.clone()).unwrap_or_default()
        }
    };

    let _filtered_options = {
        let options = options.clone();
        move || {
            let q = query.get().to_lowercase();
            options.iter().enumerate()
                .filter(|(_, o)| q.is_empty() || o.label.to_lowercase().contains(&q))
                .map(|(i, o)| (i, o.clone()))
                .collect::<Vec<_>>()
        }
    };

    #[cfg(feature = "hydrate")]
    let on_input = move |e: leptos::ev::Event| {
        use leptos::wasm_bindgen::JsCast;
        if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
            set_query.set(input.value());
            set_open.set(true);
            set_focused.set(None);
        }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_input = move |_: leptos::ev::Event| { let _ = (set_query, set_open, set_focused); };

    #[cfg(feature = "hydrate")]
    let on_focusin = move |_: leptos::ev::FocusEvent| { if !disabled { set_open.set(true); } };
    #[cfg(not(feature = "hydrate"))]
    let on_focusin = move |_: leptos::ev::FocusEvent| { let _ = set_open; };

    #[cfg(feature = "hydrate")]
    let on_keydown = {
        let options = options.clone();
        move |e: leptos::ev::KeyboardEvent| {
            let key = e.key();
            let visible: Vec<(usize, ComboboxOption)> = {
                let q = query.get().to_lowercase();
                options.iter().enumerate()
                    .filter(|(_, o)| !o.disabled && (q.is_empty() || o.label.to_lowercase().contains(&q)))
                    .map(|(i, o)| (i, o.clone()))
                    .collect()
            };
            match key.as_str() {
                "Escape" | "Tab" => { set_open.set(false); set_focused.set(None); }
                "Enter" => {
                    e.prevent_default();
                    if let Some(idx) = focused_idx.get() {
                        if let Some((_, opt)) = visible.iter().find(|(i, _)| *i == idx) {
                            set_selected.set(opt.value.clone());
                            set_query.set(String::new());
                            set_open.set(false);
                            set_focused.set(None);
                        }
                    }
                }
                "ArrowDown" | "ArrowUp" => {
                    e.prevent_default();
                    if !is_open.get() { set_open.set(true); }
                    let len = visible.len();
                    if len == 0 { return; }
                    let current = focused_idx.get();
                    let current_pos = current.and_then(|idx| visible.iter().position(|(i, _)| *i == idx));
                    let next_pos = match (key.as_str(), current_pos) {
                        ("ArrowDown", None)    => 0,
                        ("ArrowDown", Some(i)) => (i + 1).min(len - 1),
                        ("ArrowUp",   None)    => len - 1,
                        ("ArrowUp",   Some(i)) => if i == 0 { 0 } else { i - 1 },
                        _ => 0,
                    };
                    set_focused.set(Some(visible[next_pos].0));
                }
                _ => {}
            }
        }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_keydown = move |_: leptos::ev::KeyboardEvent| { let _ = (set_open, set_focused, focused_idx, set_selected, set_query); };

    let options_view = {
        let options = options.clone();
        move || {
            let q = query.get().to_lowercase();
            options.iter().enumerate()
                .filter(|(_, o)| q.is_empty() || o.label.to_lowercase().contains(&q))
                .map(|(i, opt)| {
                    let value = opt.value.clone();
                    let label = opt.label.clone();
                    let is_disabled = opt.disabled;
                    let on_click = {
                        let value = value.clone();
                        move |e: leptos::ev::MouseEvent| {
                            e.stop_propagation();
                            if !is_disabled {
                                set_selected.set(value.clone());
                                set_query.set(String::new());
                                set_open.set(false);
                                set_focused.set(None);
                            }
                        }
                    };
                    let on_mouseenter = move |_: leptos::ev::MouseEvent| {
                        if !is_disabled { set_focused.set(Some(i)); }
                    };
                    let value_for_state = value.clone();
                    let item_state = move || {
                        let mut s = vec![];
                        if selected.get() == value_for_state { s.push("selected"); }
                        if is_disabled { s.push("disabled"); }
                        if focused_idx.get() == Some(i) { s.push("focus"); }
                        s.join(" ")
                    };
                    view! {
                        <div
                            data-rs-combobox-item=""
                            data-rs-component="ComboboxItem"
                            data-rs-value=value.clone()
                            data-rs-state=item_state
                            role="option"
                            tabindex="-1"
                            aria-selected=move || (selected.get() == value).to_string()
                            aria-disabled=is_disabled.to_string()
                            on:click=on_click
                            on:mouseenter=on_mouseenter
                        >
                            {label}
                        </div>
                    }
                }).collect::<Vec<_>>()
        }
    };

    view! {
        <div
            data-rs-combobox=""
            data-rs-component="Combobox"
            data-rs-role="root"
            data-rs-state=move || { let s = root_state(); if s.is_empty() { initial_state.clone() } else { s } }
            role="combobox"
            aria-expanded=move || is_open.get().to_string()
            aria-haspopup="listbox"
            class=class
            on:focusin=on_focusin
        >
            <input
                data-rs-combobox-input=""
                data-rs-component="ComboboxInput"
                type="text"
                placeholder=placeholder
                value=input_value
                disabled=disabled
                aria-autocomplete="list"
                autocomplete="off"
                on:input=on_input
                on:keydown=on_keydown
            />
            <div
                data-rs-combobox-list=""
                data-rs-component="ComboboxList"
                data-rs-state=move || if is_open.get() { "open" } else { "closed" }
                role="listbox"
            >
                {options_view}
            </div>
        </div>
    }
}
