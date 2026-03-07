use leptos::prelude::*;
use super::combobox_ui::Combobox;
use super::types::ComboboxOption;

pub fn basic_example() -> impl IntoView {
    let options = vec![
        ComboboxOption::new("option-1", "Option 1"),
        ComboboxOption::new("option-2", "Option 2"),
        ComboboxOption::new("option-3", "Option 3"),
        ComboboxOption::new("option-4", "Option 4").disabled(),
    ];

    view! {
        <Combobox
            id="combobox-example-1".to_string()
            options=options
            placeholder="Select option...".to_string()
        />
    }
}

#[component]
pub fn BasicExample() -> impl IntoView {
    basic_example()
}
