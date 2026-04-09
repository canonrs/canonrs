//! Combobox Island — Canon Rule passthrough
use leptos::prelude::*;
use super::combobox_ui::{Combobox, ComboboxInput, ComboboxList};

#[component]
pub fn ComboboxIsland(
    children: Children,
    #[prop(into, default = String::from("Search..."))] placeholder: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Combobox class=class>
            <ComboboxInput placeholder=placeholder />
            <ComboboxList>
                {children()}
            </ComboboxList>
        </Combobox>
    }
}
