//! PageLayout Block
//! Padrões de layout enterprise (sidebar + content + aside)

use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PageLayoutVariant {
    Single,
    WithSidebar,
    WithAside,
    SidebarAndAside,
}

impl PageLayoutVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::WithSidebar => "with-sidebar",
            Self::WithAside => "with-aside",
            Self::SidebarAndAside => "sidebar-and-aside",
        }
    }
}

#[component]
pub fn PageLayout(
    children: Children,
    #[prop(optional)] sidebar: Option<ChildrenFn>,
    #[prop(optional)] aside: Option<ChildrenFn>,
    #[prop(default = PageLayoutVariant::Single)] variant: PageLayoutVariant,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            class={format!("page-layout {}", class)}
            id={id}
            data-block="page-layout"
            data-variant={variant.as_str()}
        >
            {sidebar.map(|sb| view! {
                <aside class="page-layout__sidebar">
                    {sb()}
                </aside>
            })}

            <main class="page-layout__content">
                {children()}
            </main>

            {aside.map(|a| view! {
                <aside class="page-layout__aside">
                    {a()}
                </aside>
            })}
        </div>
    }
}
