use leptos::prelude::*;
use canonrs_ui::blocks::{Card, AlertBlock, AlertVariant};

#[component]
pub fn BlockInner(
    block_id: &'static str,
    block_label: &'static str,
    block_icon: &'static str,
) -> impl IntoView {
    match block_id {
        "card" => view! {
            <Card>
                <div style="padding:0.5rem;font-size:0.8rem;color:var(--theme-surface-fg-muted);">
                    "Card content"
                </div>
            </Card>
        }.into_any(),
        "alert" => view! {
            <AlertBlock variant=AlertVariant::Info>"Alert message"</AlertBlock>
        }.into_any(),
        _ => view! {
            <div data-block-inner="">
                <span>{block_icon}</span>
                <span>{block_label}</span>
            </div>
        }.into_any(),
    }
}
