use leptos::prelude::*;
use super::drawer_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div>
            <DrawerTrigger target_drawer_id="drawer-ex">
                <button data-button data-ui-variant="solid">"Open Drawer"</button>
            </DrawerTrigger>
            <Drawer id="drawer-ex".to_string()>
                <DrawerOverlay />
                <DrawerContent>
                    <h2>"Drawer Title"</h2>
                    <p>"Drawer content"</p>
                    <button data-button data-ui-variant="outline" onclick="document.getElementById('drawer-ex').setAttribute('data-state', 'closed')">"Close"</button>
                </DrawerContent>
            </Drawer>
        </div>
    }
}
