use leptos::callback::Callback;
use leptos::prelude::*;
use leptos::ev;
use canonrs_ui::ui::nav_item::NavItem;

#[component]
pub fn NavItemInteractive(
    #[prop(into)] label: String,
    #[prop(optional, into)] href: Option<String>,
    #[prop(default = false)] active: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] on_click: Option<Callback<()>>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <NavItem
            label=label
            href=href.unwrap_or_else(|| "#".to_string())
            active=active
            disabled=disabled
            class=class
            on:click=move |_ev: ev::MouseEvent| {
                if !disabled {
                    if let Some(ref cb) = on_click {
                        cb.run(());
                    }
                }
            }
        />
    }
}
