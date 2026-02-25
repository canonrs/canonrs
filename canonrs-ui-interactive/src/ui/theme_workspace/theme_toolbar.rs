use leptos::prelude::*;
use super::viewport::Viewport;

#[component]
pub fn ThemeToolbar(
    viewport: RwSignal<Viewport>,
    on_generate: impl Fn(leptos::ev::MouseEvent) + 'static,
) -> impl IntoView {
    let btn = |label: &'static str, vp: Viewport, current: RwSignal<Viewport>| {
        let is_active = move || current.get() == vp;
        view! {
            <button
                on:click=move |_| current.set(vp)
                style=move || format!(
                    "padding:0.25rem 0.6rem;border-radius:4px;border:1px solid var(--theme-surface-border);cursor:pointer;font-size:0.75rem;background:{};color:{};",
                    if is_active() {"var(--theme-primary-bg)"} else {"var(--theme-surface-bg)"},
                    if is_active() {"var(--theme-primary-fg)"} else {"var(--theme-surface-fg)"}
                )
            >{label}</button>
        }
    };

    view! {
        <div style="display:flex;align-items:center;gap:0.5rem;padding:0.5rem 1rem;border-bottom:1px solid var(--theme-surface-border);background:var(--theme-surface-bg);flex-shrink:0;">
            <span style="font-size:0.75rem;font-weight:700;color:var(--theme-surface-fg-muted);margin-right:0.5rem;">"🎨 Theme Workspace"</span>
            {btn("🖥 Desktop", Viewport::desktop(), viewport)}
            {btn("📱 Tablet",  Viewport::tablet(),  viewport)}
            {btn("📲 Mobile",  Viewport::mobile(),  viewport)}
            <div style="flex:1;" />
            <button
                on:click=on_generate
                style="padding:0.3rem 0.8rem;border-radius:4px;border:none;cursor:pointer;font-size:0.8rem;font-weight:600;background:var(--theme-primary-bg);color:var(--theme-primary-fg);"
            >"⚡ Generate Document"</button>
        </div>
    }
}
