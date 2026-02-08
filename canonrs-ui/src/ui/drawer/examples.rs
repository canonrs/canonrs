use leptos::prelude::*;
use super::drawer_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <div>
            <button id="open-drawer-ex" data-button data-ui-variant="default">"Open Drawer"</button>
            <Drawer open=Signal::from(false) collapsed=Signal::from(false) id="drawer-ex".to_string()>
                <DrawerOverlay />
                <DrawerContent>
                    <h2>"Drawer Title"</h2>
                    <p>"Drawer content from the left side."</p>
                    <button id="close-drawer-ex" data-button data-ui-variant="outline">"Close"</button>
                </DrawerContent>
            </Drawer>
        </div>
    }
}
