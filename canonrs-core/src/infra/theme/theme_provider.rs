use leptos::prelude::*;
use super::theme_types::{ThemeContext, ThemeMode};

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let mode = RwSignal::new(ThemeMode::Dark);
    let preset = RwSignal::new("canonrs".to_string());

    provide_context(ThemeContext { mode, preset });

    #[cfg(feature = "hydrate")]
    {
        use leptos::leptos_dom::helpers::document;

        Effect::new(move |_| {
            if let Some(html) = document().document_element() {
                match mode.get() {
                    ThemeMode::Dark => {
                        let _ = html.class_list().add_1("dark");
                    }
                    ThemeMode::Light | ThemeMode::System => {
                        let _ = html.class_list().remove_1("dark");
                    }
                }
            }
        });
    }

    children()
}

#[component]
pub fn CanonRSRoot(
    #[prop(default = "canonrs".to_string())] _theme: String,
    children: Children,
) -> impl IntoView {
    view! {
        <ThemeProvider>
            {children()}
        </ThemeProvider>
    }
}

pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>()
        .expect("use_theme must be used within CanonRSRoot")
}

pub fn canonrs_theme_script(theme: &str) -> String {
    format!(
        r#"(function(){{const r=document.documentElement;const s=localStorage.getItem("{}-theme");const m=s==="dark"||s==="light"?s:window.matchMedia("(prefers-color-scheme: dark)").matches?"dark":"light";r.setAttribute("data-theme","{}");if(m==="dark"){{r.classList.add("dark")}}else{{r.classList.remove("dark")}}}})();"#,
        theme, theme
    )
}
