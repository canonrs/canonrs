use leptos::prelude::*;
use super::input_group_ui::*;
use crate::ui::input::Input;

pub fn basic_example() -> impl IntoView {
    view! {
        <InputGroup>
            <span>"$"</span>
            <Input placeholder="Amount".to_string() />
        </InputGroup>
    }
}
