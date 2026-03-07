use leptos::prelude::*;
use super::ButtonGroup;
use crate::ui::button::Button;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <ButtonGroup aria_label="Actions">
            <Button>"First"</Button>
            <Button>"Second"</Button>
            <Button>"Third"</Button>
        </ButtonGroup>
    }
}
