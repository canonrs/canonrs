use leptos::prelude::*;
use super::NavItem;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <nav data-nav-group="" data-rs-direction="horizontal">
            <NavItem label="Home".to_string() href="/".to_string() />
            <NavItem label="About".to_string() href="/about".to_string() />
            <NavItem label="Contact".to_string() href="/contact".to_string() />
        </nav>
    }
}

#[component]
pub fn WithActiveExample() -> impl IntoView {
    view! {
        <nav data-nav-group="" data-rs-direction="horizontal">
            <NavItem label="Dashboard".to_string() href="/dashboard".to_string() active=true />
            <NavItem label="Settings".to_string() href="/settings".to_string() />
            <NavItem label="Profile".to_string() href="/profile".to_string() />
        </nav>
    }
}

#[component]
pub fn VerticalExample() -> impl IntoView {
    view! {
        <nav data-nav-group="">
            <NavItem label="Dashboard".to_string() href="/dashboard".to_string() active=true />
            <NavItem label="Settings".to_string() href="/settings".to_string() />
            <NavItem label="Profile".to_string() href="/profile".to_string() />
            <NavItem label="Disabled".to_string() href="/disabled".to_string() disabled=true />
        </nav>
    }
}

#[component]
pub fn DisabledExample() -> impl IntoView {
    view! {
        <nav data-nav-group="" data-rs-direction="horizontal">
            <NavItem label="Enabled".to_string() href="/enabled".to_string() />
            <NavItem label="Disabled".to_string() href="/disabled".to_string() disabled=true />
        </nav>
    }
}
