use leptos::prelude::*;
use super::button_group_ui::*;
use crate::ui::button::Button;

pub fn basic_example() -> impl IntoView {
    view! {
        <ButtonGroup>
            <Button>"First"</Button>
            <Button>"Second"</Button>
            <Button>"Third"</Button>
        </ButtonGroup>
    }
}
