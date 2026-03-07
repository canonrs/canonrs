use leptos::prelude::*;
use super::NavItem;

#[component]
pub fn basic_example() -> impl IntoView {
    view! {
        <nav data-nav-group="">
            <NavItem label="Home" href="/" />
            <NavItem label="About" href="/about" />
            <NavItem label="Contact" href="/contact" />
        </nav>
    }
}

#[component]
pub fn with_active_example() -> impl IntoView {
    view! {
        <nav data-nav-group="">
            <NavItem label="Dashboard" href="/dashboard" active=true />
            <NavItem label="Settings" href="/settings" />
            <NavItem label="Profile" href="/profile" />
        </nav>
    }
}

#[component]
pub fn disabled_example() -> impl IntoView {
    view! {
        <nav data-nav-group="">
            <NavItem label="Enabled" href="/enabled" />
            <NavItem label="Disabled" href="/disabled" disabled=true />
        </nav>
    }
}
