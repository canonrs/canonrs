//! @canon-id: command-panel
//! @canon-type: block
//! @canon-category: overlay
//! @canon-variant: overlay
//! @canon-container: false
//! @canon-regions: search, results, footer
//! @canon-label: Command Panel
//! @canon-description: Command palette overlay block
//! @canon-tags: command,comando,palette,busca,spotlight,atalho
use leptos::prelude::*;

#[component]
pub fn CommandPanelBlock(
    #[prop(optional)] search: Option<ChildrenFn>,
    #[prop(optional)] results: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] style: String,
) -> impl IntoView {
    view! {
        <div
            data-block="command-panel"
            data-block-version="1"
            style=style
            class=class
        >
            {search.map(|s| view! { <div data-block-region="search">{s()}</div> })}
            {results.map(|r| view! { <div data-block-region="results">{r()}</div> })}
            {footer.map(|f| view! { <div data-block-region="footer">{f()}</div> })}
        </div>
    }
}
