//! ToggleGroup Island — Canon Rule passthrough
use leptos::prelude::*;
use super::toggle_group_ui::ToggleGroup as ToggleGroupUi;

#[component]
pub fn ToggleGroup(
    children: Children,
    #[prop(default = false)] multiple: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let state = if disabled { "disabled"
} else { "" };
    view! {
        <ToggleGroupUi multiple=multiple class=class attr:data-rs-state=state>
            {children()}
        </ToggleGroupUi>
    }
}

#[component]
pub fn ToggleGroupItem(
    children: Children,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = false)] on: bool,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let state = if disabled {
        format!("{} disabled", if on { "on" } else { "off" })
    } else {
        if on { "on".into() } else { "off".into() }
    };
    view! {
        <button
            type="button"
            data-rs-toggle=""
            data-rs-component="Toggle"
            data-rs-value=value
            data-rs-state=state
            aria-pressed=if on { "true" } else { "false" }
            role="button"
        >
            {children()}
        </button>
    }
}
