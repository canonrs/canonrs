//! # CommandPanel Block
//! Command palette / quick actions overlay

use leptos::prelude::*;

#[component]
pub fn CommandPanelBlock(
    #[prop(into)] open: MaybeSignal<bool>,
    #[prop(optional)] on_close: Option<Callback<()>>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    let search_input = NodeRef::<leptos::html::Input>::new();

    let handle_backdrop_click = move |_| {
        if let Some(cb) = on_close {
            cb.run(());
        }
    };

    view! {
        <div 
            class=move || format!(
                "canon-command-panel-overlay {}",
                if open.get() { "canon-command-panel-overlay--open" } else { "" }
            )
            attr:data-block="command-panel"
            attr:data-command-panel-action="close-backdrop" on:click=handle_backdrop_click
        >
            <div class="canon-command-panel__backdrop" />
            
            <div 
                class=format!("canon-command-panel {}", class)
                attr:data-command-panel-action="prevent-close" on:click=|e| e.stop_propagation()
            >
                <div class="canon-command-panel__search">
                    <input 
                        type="text"
                        class="canon-command-panel__input"
                        placeholder=placeholder.unwrap_or_else(|| "Search commands...".to_string())
                        node_ref=search_input
                        autofocus
                    />
                </div>
                
                <div class="canon-command-panel__content">
                    {children()}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn CommandPanelItem(
    #[prop(into)] label: String,
    #[prop(optional, into)] shortcut: Option<String>,
    #[prop(optional)] icon: Option<Children>,
    #[prop(optional)] on_select: Option<Callback<()>>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <button 
            class=format!("canon-command-panel__item {}", class)
            attr:data-command-panel-action="select-item" on:click=move |_| {
                if let Some(cb) = on_select {
                    cb.run(());
                }
            }
        >
            {icon.map(|i| view! {
                <span class="canon-command-panel__item-icon">{i()}</span>
            })}
            
            <span class="canon-command-panel__item-label">{label}</span>
            
            {shortcut.map(|s| view! {
                <span class="canon-command-panel__item-shortcut">{s}</span>
            })}
        </button>
    }
}
