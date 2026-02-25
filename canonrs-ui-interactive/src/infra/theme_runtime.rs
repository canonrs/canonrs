use leptos::prelude::*;
use super::super::ui::theme_engine::ThemeState;

#[cfg(target_arch = "wasm32")]
pub fn apply_theme_to(el: &leptos::web_sys::HtmlElement, theme: &ThemeState) {
    let style = el.style();
    for (key, val) in theme.current_css_vars() {
        let _ = style.set_property(&format!("--theme-{}", key), &val);
    }
    let _ = style.set_property("--preview-radius", &format!("{}rem", theme.radius.get()));
}

#[component]
pub fn ThemeRuntime(theme: ThemeState, children: Children) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();
    let t = theme.clone();

    Effect::new(move |_| {
        let _ = t.active.get();
        let _ = t.light.get();
        let _ = t.dark.get();
        let _ = t.radius.get();
        let Some(el) = node_ref.get() else { return };
        #[cfg(target_arch = "wasm32")]
        {
            use leptos::wasm_bindgen::JsCast;
            let html_el: &leptos::web_sys::HtmlElement = el.unchecked_ref();
            apply_theme_to(html_el, &t);
        }
    });

    view! {
        <div node_ref=node_ref id="theme-runtime-root" style="display:contents;">
            {children()}
        </div>
    }
}
