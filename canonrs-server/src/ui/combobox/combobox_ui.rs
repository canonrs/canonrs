use leptos::prelude::*;
use canonrs_core::primitives::combobox::*;
use super::types::ComboboxOption;

#[component]
pub fn Combobox(
    id: String, // 👈 OBRIGATÓRIO (sem default, sem Option)
    options: Vec<ComboboxOption>,
    #[prop(default = "Select option...".to_string())] placeholder: String,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ComboboxPrimitive
            expanded=false
            disabled=disabled
            class=class
            id=id
        >
            <ComboboxTriggerPrimitive disabled=disabled>
                <span>{placeholder}</span>
            </ComboboxTriggerPrimitive>

            <ComboboxListPrimitive>
                {options.into_iter().map(|opt| view! {
                    <ComboboxItemPrimitive
                        selected=false
                        disabled=opt.disabled
                    >
                        <span>{opt.label}</span>
                    </ComboboxItemPrimitive>
                }).collect_view()}
            </ComboboxListPrimitive>
        </ComboboxPrimitive>
    }
}

#[component]
pub fn ComboboxPreview() -> impl IntoView {
    view! { <Combobox id="cb-preview".to_string() options=vec![] /> }
}
