use leptos::prelude::*;

#[derive(Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct RadioOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

#[island]
pub fn RadioGroupIsland(
    #[prop(optional, into)] name: Option<String>,
    #[prop(into)] selected_value: String,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
    options: Vec<RadioOption>,
) -> impl IntoView {
    let name     = name.unwrap_or_else(|| "radio-group".to_string());
    let disabled = disabled.unwrap_or(false);
    let class    = class.unwrap_or_default();

    let (selected, set_selected) = signal(selected_value.clone());
    let (focused,  set_focused)  = signal::<Option<usize>>(None);

    let options_view = {
        let options = options.clone();
        let name = name.clone();
        options.into_iter().enumerate().map(move |(i, opt)| {
            let value      = opt.value.clone();
            let label      = opt.label.clone();
            let is_disabled = opt.disabled || disabled;
            let name       = name.clone();

            let on_change = {
                let value = value.clone();
                move |_: leptos::ev::Event| {
                    if !is_disabled {
                        set_selected.set(value.clone());
                    }
                }
            };

            let on_focus = move |_: leptos::ev::FocusEvent| {
                if !is_disabled { set_focused.set(Some(i)); }
            };

            let on_blur = move |_: leptos::ev::FocusEvent| {
                set_focused.set(None);
            };

            let item_state = {
                let value = value.clone();
                move || {
                    let mut s = vec![];
                    if selected.get() == value { s.push("selected"); }
                    if is_disabled             { s.push("disabled"); }
                    if focused.get() == Some(i) { s.push("focus");   }
                    s.join(" ")
                }
            };

            view! {
                <label
                    data-rs-radio=""
                    data-rs-component="Radio"
                    data-rs-state=item_state
                    class=""
                >
                    <input
                        type="radio"
                        data-rs-radio-input=""
                        name=name.clone()
                        value=value.clone()
                        checked=move || selected.get() == value
                        disabled=is_disabled
                        aria-disabled=is_disabled.to_string()
                        tabindex="-1"
                        on:change=on_change
                        on:focus=on_focus
                        on:blur=on_blur
                    />
                    <span data-rs-radio-indicator="" />
                    {label}
                </label>
            }
        }).collect::<Vec<_>>()
    };

    view! {
        <div
            data-rs-radio-group=""
            data-rs-component="RadioGroup"
            class=class
            role="radiogroup"
        >
            {options_view}
        </div>
    }
}
