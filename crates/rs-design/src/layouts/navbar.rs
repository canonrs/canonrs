use crate::tokens::{SEMANTIC, SPACING};
use leptos::prelude::*;

#[derive(Clone)]
pub struct NavConfig {
    pub label: String,
    pub href: String,
    pub active: bool,
}

impl NavConfig {
    pub fn new(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: href.into(),
            active: false,
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

#[component]
pub fn Navbar(
    #[prop(default = vec![])] items: Vec<NavConfig>,
    #[prop(optional)] logo: Option<Children>,
    #[prop(optional)] actions: Option<Children>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let navbar_class = format!(
        "flex h-16 items-center justify-between border-b px-[{}] {}",
        SPACING.lg, class
    );

    view! {
        <nav
            class=navbar_class
            style=format!("background: {}; border-color: {};", SEMANTIC.background, SEMANTIC.border)
        >
            <div class="flex items-center gap-6">
                {logo.map(|logo_fn| view! {
                    <div class="flex items-center">
                        {logo_fn()}
                    </div>
                })}

                <ul class="flex items-center gap-1">
                    {items.into_iter().map(|item| {
                        let item_class = format!(
                            "px-3 py-2 text-sm font-medium rounded-md transition-colors {}",
                            if item.active {
                                format!("bg-[{}] text-[{}]", SEMANTIC.accent, SEMANTIC.accent_foreground)
                            } else {
                                format!("text-[{}] hover:bg-[{}]", SEMANTIC.foreground, SEMANTIC.accent)
                            }
                        );

                        view! {
                            <li>
                                <a href=item.href class=item_class>
                                    {item.label}
                                </a>
                            </li>
                        }
                    }).collect::<Vec<_>>()}
                </ul>
            </div>

            {actions.map(|actions_fn| view! {
                <div class="flex items-center gap-2">
                    {actions_fn()}
                </div>
            })}
        </nav>
    }
}
