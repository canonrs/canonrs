use leptos::prelude::*;
use super::Sidebar;
use crate::shared::DrawerVariant;

#[component]
pub fn basic_example() -> impl IntoView {
    let open = RwSignal::new(false);
    let collapsed = RwSignal::new(false);

    view! {
        <div>
            <button on:click=move |_| open.set(!open.get())>
                "Toggle Sidebar"
            </button>
            <Sidebar open=open.into() collapsed=collapsed.into()>
                <nav>
                    <a href="/">"Home"</a>
                    <a href="/about">"About"</a>
                    <a href="/contact">"Contact"</a>
                </nav>
            </Sidebar>
        </div>
    }
}

#[component]
pub fn persistent_example() -> impl IntoView {
    let open = RwSignal::new(true);
    let collapsed = RwSignal::new(false);

    view! {
        <Sidebar open=open.into() collapsed=collapsed.into() variant=DrawerVariant::Persistent>
            <nav>
                <a href="/dashboard">"Dashboard"</a>
                <a href="/settings">"Settings"</a>
            </nav>
        </Sidebar>
    }
}

#[component]
pub fn collapsed_example() -> impl IntoView {
    let open = RwSignal::new(true);
    let collapsed = RwSignal::new(true);

    view! {
        <Sidebar open=open.into() collapsed=collapsed.into()>
            <nav>
                <a href="/" title="Home">"🏠"</a>
                <a href="/settings" title="Settings">"⚙️"</a>
            </nav>
        </Sidebar>
    }
}
