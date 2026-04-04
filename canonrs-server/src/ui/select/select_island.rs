use leptos::prelude::*;

#[derive(Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

#[island]
pub fn SelectIsland(
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] selected_value: Option<String>,
    options: Vec<SelectOption>,
) -> impl IntoView {
    let placeholder  = placeholder.unwrap_or_else(|| "Select...".to_string());
    let disabled     = disabled.unwrap_or(false);
    let class        = class.unwrap_or_default();

    let (is_open,    set_open)    = signal(false);
    let (selected,   set_selected) = signal(selected_value.clone().unwrap_or_default());
    let (focused_idx, set_focused) = signal::<Option<usize>>(None);

    let initial_state = if disabled { "disabled closed".to_string() } else { "closed".to_string() };

    let root_state = move || {
        let mut s = vec![];
        if disabled     { s.push("disabled"); }
        if is_open.get() { s.push("open"); } else { s.push("closed"); }
        s.join(" ")
    };

    let selected_label = {
        let options = options.clone();
        let _selected_value = selected_value.clone();
        move || {
            let val = selected.get();
            if val.is_empty() { return placeholder.clone(); }
            options.iter().find(|o| o.value == val).map(|o| o.label.clone()).unwrap_or(placeholder.clone())
        }
    };

    #[cfg(feature = "hydrate")]
    let on_trigger_click = move |e: leptos::ev::MouseEvent| {
        e.stop_propagation();
        if !disabled { set_open.update(|o| *o = !*o); }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_trigger_click = move |_: leptos::ev::MouseEvent| { let _ = set_open; };

    #[cfg(feature = "hydrate")]
    let on_keydown = {
        let navigable: Vec<usize> = options.iter().enumerate().filter(|(_, o)| !o.disabled).map(|(i, _)| i).collect();
        let options = options.clone();
        move |e: leptos::ev::KeyboardEvent| {
            let key = e.key();
            match key.as_str() {
                "Escape" | "Tab" => { set_open.set(false); set_focused.set(None); }
                " " | "Enter" => {
                    e.prevent_default();
                    if !is_open.get() {
                        set_open.set(true);
                    } else if let Some(idx) = focused_idx.get() {
                        if let Some(opt) = options.get(idx) {
                            if !opt.disabled {
                                set_selected.set(opt.value.clone());
                                set_open.set(false);
                                set_focused.set(None);
                            }
                        }
                    }
                }
                "ArrowDown" | "ArrowUp" => {
                    e.prevent_default();
                    if !is_open.get() { set_open.set(true); }
                    let len = navigable.len();
                    if len == 0 { return; }
                    let current = focused_idx.get();
                    let current_nav_pos = current.and_then(|idx| navigable.iter().position(|&i| i == idx));
                    let next_nav_pos = match (key.as_str(), current_nav_pos) {
                        ("ArrowDown", None)    => 0,
                        ("ArrowDown", Some(i)) => (i + 1).min(len - 1),
                        ("ArrowUp",   None)    => len - 1,
                        ("ArrowUp",   Some(i)) => if i == 0 { 0 } else { i - 1 },
                        _ => 0,
                    };
                    set_focused.set(Some(navigable[next_nav_pos]));
                }
                _ => {}
            }
        }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_keydown = move |_: leptos::ev::KeyboardEvent| { let _ = (set_open, set_focused, focused_idx); };

    let options_view = {
        let options = options.clone();

        options.into_iter().enumerate().map(move |(i, opt)| {
            let value = opt.value.clone();
            let label = opt.label.clone();
            let is_disabled = opt.disabled;
            let on_click = {
                let value = value.clone();
                move |e: leptos::ev::MouseEvent| {
                    e.stop_propagation();
                    if !is_disabled {
                        set_selected.set(value.clone());
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
                    data-rs-select-item=""
                    data-rs-component="SelectItem"
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
    };

    view! {
        <div
            data-rs-select=""
            data-rs-component="Select"
            data-rs-role="root"
            data-rs-state=move || { let s = root_state(); if s.is_empty() { initial_state.clone() } else { s } }
            class=class
        >
            <button
                type="button"
                data-rs-select-trigger=""
                data-rs-component="SelectTrigger"
                aria-haspopup="listbox"
                aria-expanded=move || is_open.get().to_string()
                aria-disabled=disabled.to_string()
                disabled=disabled
                on:click=on_trigger_click
                on:keydown=on_keydown
            >
                <span data-rs-select-value="" data-rs-component="SelectValue">
                    {selected_label}
                </span>
            </button>
            <div
                data-rs-select-content=""
                data-rs-component="SelectContent"
                data-rs-state=move || if is_open.get() { "open" } else { "closed" }
                role="listbox"
            >
                {options_view}
            </div>
        </div>
    }
}
