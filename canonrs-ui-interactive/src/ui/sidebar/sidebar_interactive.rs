use leptos::callback::Callback;
use leptos::prelude::*;
use leptos::ev;
use canonrs_ui::ui::sidebar::SidebarMenuButton;

#[component]
pub fn SidebarMenuButtonInteractive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] is_active: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] on_click: Option<Callback<()>>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <SidebarMenuButton
            is_active=is_active
            disabled=disabled
            class=class
            id=id
            on:click=move |_ev: ev::MouseEvent| {
                if !disabled {
                    if let Some(ref handler) = on_click {
                        handler.run(());
                    }
                }
            }
        >
            {children.map(|c| c())}
        </SidebarMenuButton>
    }
}
