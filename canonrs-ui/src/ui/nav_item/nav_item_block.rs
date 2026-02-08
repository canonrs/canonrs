use leptos::prelude::*;

#[component]
pub fn NavItem(
    #[prop(into)] label: String,
    #[prop(optional, into)] href: Option<String>,
    #[prop(default = false)] active: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(optional, into)] icon: Option<Children>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <a 
            href=href.unwrap_or_else(|| "#".to_string()) 
            class=move || format!(
                "canon-nav-item {} {} {}", 
                if active { "canon-nav-item--active" } else { "" }, 
                if disabled { "canon-nav-item--disabled" } else { "" }, 
                class
            ) 
            data-block="nav-item"
        >
            {icon.map(|i| view! { <span class="canon-nav-item__icon">{i()}</span> })}
            <span class="canon-nav-item__label">{label}</span>
        </a>
    }
}
