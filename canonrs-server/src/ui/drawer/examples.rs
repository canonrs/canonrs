use leptos::prelude::*;
use super::drawer_ui::*;
use crate::ui::button::button_ui::{Button, ButtonVariant};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Drawer>
            <Button variant=ButtonVariant::Primary attr:data-rs-drawer-trigger="">"Open Drawer"</Button>
            <DrawerOverlay />
            <DrawerContent aria_labelledby="drawer-example-title">
                <h2 id="drawer-example-title">"Drawer Title"</h2>
                <p>"Drawer content"</p>
                <Button variant=ButtonVariant::Outline attr:data-rs-drawer-close="">"Close"</Button>
            </DrawerContent>
        </Drawer>
    }
}
