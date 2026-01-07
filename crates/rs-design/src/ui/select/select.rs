use leptos::prelude::*;
use super::variants::{SelectSize, SelectValidation};

const BASE_CLASSES: &str = "\
    w-full \
    appearance-none \
    rounded-[var(--radius-md)] \
    border-[length:var(--border-width-thin)] \
    bg-background \
    font-family-[var(--font-family-sans)] \
    font-[number:var(--font-weight-medium)] \
    text-foreground \
    line-height-[var(--line-height-normal)] \
    shadow-[var(--shadow-sm)] \
    z-[var(--z-base)] \
    transition-all duration-[var(--motion-duration-fast)] ease-[var(--motion-ease-standard)] \
    focus:outline-none \
    focus:ring-2 \
    focus:border-primary \
    hover:opacity-[var(--state-hover-opacity)] \
    active:opacity-[var(--state-active-opacity)] \
    disabled:cursor-not-allowed \
    disabled:opacity-[var(--state-disabled-opacity)] \
    disabled:text-muted-foreground \
    [&>option]:rounded-[var(--radius-sm)] \
    [&>option]:line-height-[var(--line-height-tight)] \
    [&>option:disabled]:text-muted-foreground";

const OPTION_BASE_CLASSES: &str = "\
    font-family-[var(--font-family-sans)] \
    font-[number:var(--font-weight-regular)] \
    text-foreground \
    bg-background \
    py-[var(--space-sm)] \
    px-[var(--space-xs)] \
    line-height-[var(--line-height-tight)]";

#[component]
pub fn Select(
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] name: String,
    value: RwSignal<String>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional)] size: Option<SelectSize>,
    #[prop(default = SelectValidation::None)] validation: SelectValidation,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] required: bool,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let size = size.unwrap_or_default();
    let classes = format!(
        "{} {} {} {} {}",
        BASE_CLASSES,
        size.classes(),
        validation.border_classes(),
        validation.ring_classes(),
        class
    );

    view! {
        <select
            id=id
            name=name
            prop:value=move || value.get()
            on:change=move |ev| {
                let val = event_target_value(&ev);
                value.set(val.clone());
                if let Some(cb) = on_change {
                    cb.run(val);
                }
            }
            disabled=disabled
            required=required
            class=classes
        >
            {if !placeholder.is_empty() {
                Some(view! {
                    <option value="" disabled=true class="text-muted-foreground">
                        {placeholder}
                    </option>
                })
            } else {
                None
            }}
            {children()}
        </select>
    }
}

#[component]
pub fn SelectOption(
    #[prop(into)] value: String,
    #[prop(default = false)] disabled: bool,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <option
            value=value
            disabled=disabled
            class=format!("{} {}", OPTION_BASE_CLASSES, class)
        >
            {children()}
        </option>
    }
}
