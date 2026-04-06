use leptos::prelude::*;
use canonrs_core::primitives::TextareaPrimitive;
use canonrs_core::meta::DisabledState;

#[island]
pub fn TextareaIsland(
    #[prop(optional, into)] value: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] readonly: Option<bool>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional, into)] aria_labelledby: Option<String>,
    #[prop(optional, into)] aria_describedby: Option<String>,
    #[prop(optional)] rows: Option<u32>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let is_disabled = disabled.unwrap_or(false);
    let disabled_state = if is_disabled { DisabledState::Disabled } else { DisabledState::Enabled };

    let focused = RwSignal::new(false);

    let wrapper_state = move || {
        let mut s = vec![];
        if focused.get() { s.push("focus"); }
        if is_disabled   { s.push("disabled"); }
        s.join(" ")
    };

    view! {
        <div
            data-rs-textarea-island=""
            data-rs-state=wrapper_state
        >
            <TextareaPrimitive
                value=value.unwrap_or_default()
                placeholder=placeholder.unwrap_or_default()
                name=name.unwrap_or_default()
                disabled=disabled_state
                readonly=readonly.unwrap_or(false)
                required=required.unwrap_or(false)
                aria_labelledby=aria_labelledby.unwrap_or_default()
                aria_describedby=aria_describedby.unwrap_or_default()
                rows=rows.unwrap_or(3)
                class=class.unwrap_or_default()
                on:focus=move |_: leptos::ev::FocusEvent| { focused.set(true); }
                on:blur=move |_: leptos::ev::FocusEvent| { focused.set(false); }
            />
        </div>
    }
}
