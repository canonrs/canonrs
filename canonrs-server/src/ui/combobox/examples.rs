use leptos::prelude::*;
use super::combobox_ui::{Combobox, ComboboxTrigger, ComboboxList, ComboboxItem};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Combobox>
            <ComboboxTrigger>"Select option..."</ComboboxTrigger>
            <ComboboxList>
                <ComboboxItem>"Option 1"</ComboboxItem>
                <ComboboxItem>"Option 2"</ComboboxItem>
                <ComboboxItem>"Option 3"</ComboboxItem>
                <ComboboxItem disabled=true>"Option 4"</ComboboxItem>
            </ComboboxList>
        </Combobox>
    }
}
