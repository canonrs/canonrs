use leptos::prelude::*;
use super::InputGroup;
use crate::ui::input::Input;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <InputGroup merge_radius=true>
            <span data-input-group-addon="">"@"</span>
            <Input 
                placeholder="username".to_string()
            />
        </InputGroup>
    }
}
