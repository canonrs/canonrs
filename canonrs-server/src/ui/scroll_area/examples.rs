use leptos::prelude::*;
use super::scroll_area_ui::ScrollArea;
use super::scroll_area_primitive::ScrollOrientation;

pub fn basic_example() -> impl IntoView {
    view! {
        <div style="height:240px;">
            <ScrollArea id="scroll-basic".to_string() auto_hide=true>
                <div style="padding:1rem;width:800px;">
                    {(1..=30).map(|i| view! {
                        <p style="margin:0;padding:6px 0;border-bottom:1px solid var(--theme-surface-border);white-space:nowrap;">
                            {format!("Linha {:02} — Scrollable content enterprise token-based Canon Design System", i)}
                        </p>
                    }).collect::<Vec<_>>()}
                </div>
            </ScrollArea>
        </div>
    }
}

pub fn horizontal_example() -> impl IntoView {
    view! {
        <div style="height:60px;">
            <ScrollArea id="scroll-horizontal".to_string() orientation=ScrollOrientation::Horizontal auto_hide=false>
                <div style="display:flex;gap:1rem;padding:0.5rem;width:max-content;">
                    {(1..=20).map(|i| view! {
                        <div style="min-width:120px;padding:0.5rem 1rem;background:var(--theme-surface-muted);border-radius:var(--radius-md);white-space:nowrap;">
                            {format!("Item {:02}", i)}
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </ScrollArea>
        </div>
    }
}

pub fn both_example() -> impl IntoView {
    view! {
        <div style="height:240px;">
            <ScrollArea id="scroll-both".to_string() orientation=ScrollOrientation::Both auto_hide=true>
                <div style="padding:1rem;width:1200px;">
                    {(1..=30).map(|i| view! {
                        <p style="margin:0;padding:6px 0;border-bottom:1px solid var(--theme-surface-border);white-space:nowrap;">
                            {format!("Linha {:02} — Conteúdo muito largo que requer scroll horizontal E vertical simultaneamente — Canon Analytics Engine", i)}
                        </p>
                    }).collect::<Vec<_>>()}
                </div>
            </ScrollArea>
        </div>
    }
}
