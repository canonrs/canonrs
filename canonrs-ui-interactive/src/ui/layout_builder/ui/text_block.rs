use leptos::prelude::*;
use crate::ui::layout_builder::types::{Node, NodeKind, CanvasMode, TextVariant};
use crate::ui::layout_builder::state::builder_engine::BuilderEngine;

#[component]
pub fn TextBlock(
    node: Node,
    engine: RwSignal<BuilderEngine>,
    tree: RwSignal<Vec<Node>>,
    canvas_mode: RwSignal<CanvasMode>,
    variant: TextVariant,
) -> impl IntoView {
    let node_id = node.id;
    let is_builder = move || canvas_mode.get() == CanvasMode::Builder;
    let init = match &node.kind { NodeKind::Text { content, .. } => content.clone(), _ => String::new() };
    let text_signal: RwSignal<String> = RwSignal::new(init);
    let placeholder = variant.placeholder();
    let font_style = match variant {
        TextVariant::Heading1  => "font-size:1.5rem;font-weight:700;line-height:1.2;",
        TextVariant::Heading2  => "font-size:1.25rem;font-weight:600;line-height:1.3;",
        TextVariant::Heading3  => "font-size:1.1rem;font-weight:600;line-height:1.4;",
        TextVariant::Paragraph => "font-size:0.9rem;line-height:1.6;",
        TextVariant::Label     => "font-size:0.8rem;font-weight:500;",
        TextVariant::Caption   => "font-size:0.75rem;color:var(--theme-surface-fg-muted);",
    };
    let style_str = format!("{font_style} min-width:40px;outline:none;cursor:text;padding:0.1rem 0.25rem;border-radius:2px;");

    view! {
        <div
            data-text-block=""
            contenteditable=move || if is_builder() { "true" } else { "false" }
            attr:data-placeholder=placeholder
            style=style_str
            on:input=move |ev| {
                use leptos::wasm_bindgen::JsCast;
                if let Some(el) = ev.target().and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok()) {
                    let val = el.inner_text();
                    text_signal.set(val.clone());
                    engine.update(|e| {
                        let _ = e.execute(rs_canonrs::application::Command::UpdateProps {
                            node_id,
                            key: "content".to_string(),
                            value: serde_json::Value::String(val.clone()),
                        });
                    });
                    tree.update(|t| {
                        if let Some(n) = t.iter_mut().find(|n| n.id == node_id) {
                            n.kind = NodeKind::Text { content: val, variant };
                        }
                    });
                }
            }
            on:click=move |ev| ev.stop_propagation()
            on:pointerdown=move |ev| ev.stop_propagation()
        >
            {move || { let t = text_signal.get(); if t.is_empty() { String::new() } else { t } }}
        </div>
    }
}
