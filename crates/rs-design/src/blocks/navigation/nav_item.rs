use crate::ui::{SidebarMenuButton};
use leptos::prelude::*;

#[component]
pub fn NavItem(
    #[prop(into)] href: String,
    #[prop(into)] label: String,
    #[prop(default = false)] active: bool,
    #[prop(optional)] icon: Option<ChildrenFn>,
) -> impl IntoView {
    let href_val = StoredValue::new(href);
    let label_val = StoredValue::new(label);
    let icon_stored = StoredValue::new(icon);
    
    view! {
        <a href=href_val.get_value()>
            <SidebarMenuButton is_active=active>
                {move || icon_stored.get_value().map(|icon_fn| icon_fn())}
                {label_val.get_value()}
            </SidebarMenuButton>
        </a>
    }
}
