use leptos::prelude::*;
use canonrs_providers::theme::ThemeProvider;

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

pub fn use_theme() -> canonrs_providers::theme::ThemeContext {
    use_context::<canonrs_providers::theme::ThemeContext>()
        .expect("use_theme must be used within CanonRSRoot")
}

pub fn canonrs_theme_script(theme: &str) -> String {
    format!(
        r#"(function(){{const r=document.documentElement;const s=localStorage.getItem("{}-theme");const m=s==="dark"||s==="light"?s:window.matchMedia("(prefers-color-scheme: dark)").matches?"dark":"light";r.setAttribute("data-theme","{}");if(m==="dark"){{r.classList.add("dark")}}else{{r.classList.remove("dark")}}}})();"#,
        theme, theme
    )
}
