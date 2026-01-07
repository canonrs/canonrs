use crate::tokens::{SEMANTIC, SPACING};
use leptos::prelude::*;

#[derive(Clone)]
pub struct FooterLink {
    pub label: String,
    pub href: String,
}

impl FooterLink {
    pub fn new(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: href.into(),
        }
    }
}

#[derive(Clone)]
pub struct FooterSection {
    pub title: String,
    pub links: Vec<FooterLink>,
}

impl FooterSection {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            links: vec![],
        }
    }

    pub fn link(mut self, label: impl Into<String>, href: impl Into<String>) -> Self {
        self.links.push(FooterLink::new(label, href));
        self
    }
}

#[component]
pub fn Footer(
    #[prop(default = vec![])] sections: Vec<FooterSection>,
    #[prop(optional)] copyright: Option<String>,
    #[prop(optional)] logo: Option<Children>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let footer_class = format!("border-t mt-auto {}", class);

    view! {
        <footer
            class=footer_class
            style=format!("background: {}; border-color: {};", SEMANTIC.background, SEMANTIC.border)
        >
            <div class=format!("container mx-auto px-[{}] py-12", SPACING.lg)>
                <div class="grid grid-cols-1 md:grid-cols-4 gap-8">
                    {logo.map(|logo_fn| view! {
                        <div class="col-span-1">
                            {logo_fn()}
                        </div>
                    })}

                    {sections.into_iter().map(|section| view! {
                        <div>
                            <h3
                                class="font-semibold mb-4"
                                style=format!("color: {};", SEMANTIC.foreground)
                            >
                                {section.title}
                            </h3>
                            <ul class="space-y-2">
                                {section.links.into_iter().map(|link| view! {
                                    <li>

                                        <a
                                            href=link.href
                                            class="text-sm hover:underline"
                                            style=format!("color: {};", SEMANTIC.muted_foreground)
                                        >
                                            {link.label}
                                        </a>
                                    </li>
                                }).collect::<Vec<_>>()}
                            </ul>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>

                {copyright.map(|text| view! {
                    <div
                        class="border-t mt-8 pt-8 text-center text-sm"
                        style=format!("color: {}; border-color: {};", SEMANTIC.muted_foreground, SEMANTIC.border)
                    >
                        {text}
                    </div>
                })}
            </div>
        </footer>
    }
}
