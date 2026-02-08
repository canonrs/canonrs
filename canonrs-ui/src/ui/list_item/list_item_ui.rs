use leptos::prelude::*;

#[component]
pub fn ListItem(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] id: String,
    #[prop(default = false)] selectable: bool,
    #[prop(default = false)] selected: bool,
) -> impl IntoView {
    view! {
        <div
            data-list-item
            data-list-item-selectable=selectable.to_string()
            id=id
            data-selected=selected.to_string()
            style="cursor: pointer; padding: 0.75rem; border-bottom: 1px solid var(--semantic-surface-border);"
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ListItemTitle(
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div style="font-weight: 600;">
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ListItemDescription(
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div style="font-size: 0.875rem; color: var(--semantic-text-secondary);">
            {children.map(|c| c())}
        </div>
    }
}
