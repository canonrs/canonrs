//! MarkdownToolbar - Controls for breadcrumb, TOC and line numbers

use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct MarkdownToolbarState {
    pub show_breadcrumb: RwSignal<bool>,
}

impl Default for MarkdownToolbarState {
    fn default() -> Self {
        Self {
            show_breadcrumb: RwSignal::new(true),
        }
    }
}

#[component]
pub fn MarkdownToolbar(
    #[prop(into)] target_id: String,
) -> impl IntoView {
    let state = use_context::<MarkdownToolbarState>()
        .unwrap_or_else(|| {
            let s = MarkdownToolbarState::default();
            provide_context(s);
            s
        });
    
    let btn_toc_id = format!("{}-btn-toc", target_id);
    let btn_lines_id = format!("{}-btn-lines", target_id);
    
    view! {
        <div data-markdown-toolbar="">
            <button
                data-toolbar-toggle=""
                data-active={move || state.show_breadcrumb.get()}
                on:click=move |_| state.show_breadcrumb.update(|v| *v = !*v)
                title="Toggle Breadcrumb"
            >
                "🍞"
            </button>
            
            <button
                id={btn_toc_id}
                data-toolbar-toggle=""
                data-action="toggle-toc"
                data-target={target_id.clone()}
                data-active="true"
                title="Toggle TOC"
            >
                "📑"
            </button>
            
            <button
                id={btn_lines_id}
                data-toolbar-toggle=""
                data-action="toggle-line-numbers"
                data-target={target_id}
                data-active="false"
                title="Toggle Line Numbers"
            >
                "🔢"
            </button>
        </div>
    }
}
