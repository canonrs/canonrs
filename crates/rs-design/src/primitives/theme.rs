//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => "system",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "dark" => Theme::Dark,
            "system" => Theme::System,
            _ => Theme::Light,
        }
    }
}

use leptos::prelude::*;

#[component]
pub fn ThemePrimitive(
    children: Children,
    #[prop(optional)] initial_theme: Option<Theme>,
) -> impl IntoView {
    let theme = RwSignal::new(initial_theme.unwrap_or(Theme::System));

    provide_context(theme);

    // Effect APENAS no browser
    if leptos::prelude::is_browser() {
        leptos::prelude::Effect::new(move |_| {
            let theme_value = theme.get();

            // Salvar em localStorage
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item("app_theme", theme_value.as_str());
                }
            }

            // Aplicar no document
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(html) = document.document_element() {
                        let theme_str = theme_value.as_str();
                        let _ = html.set_attribute("data-theme", theme_str);

                        if matches!(theme_value, Theme::System) {
                            let prefers_dark = window
                                .match_media("(prefers-color-scheme: dark)")
                                .ok()
                                .flatten()
                                .map(|mql| mql.matches())
                                .unwrap_or(false);

                            let actual_theme = if prefers_dark { "dark" } else { "light" };
                            let _ = html.set_attribute("data-actual-theme", actual_theme);
                        } else {
                            let _ = html.set_attribute("data-actual-theme", theme_str);
                        }
                    }
                }
            }
        });
    }

    view! {
        {children()}
    }
}

pub fn use_theme() -> RwSignal<Theme> {
    expect_context::<RwSignal<Theme>>()
}

#[component]
pub fn ThemeTogglePrimitive(children: Children) -> impl IntoView {
    let theme = use_theme();

    let toggle_theme = move |_| {
        let current = theme.get();
        let new_theme = match current {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::System,
            Theme::System => Theme::Light,
        };
        theme.set(new_theme);
    };

    view! {
        <div on:click=toggle_theme>
            {children()}
        </div>
    }
}
