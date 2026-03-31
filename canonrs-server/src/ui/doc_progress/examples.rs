//! DocProgress Examples
//!
//! Two usage modes:
//!   1. Standalone — fixed bar at top of viewport (no sticky header)
//!   2. Portal     — bar anchored to bottom edge of sticky header

use leptos::prelude::*;
use super::{DocProgress, DocProgressSlot};
use canonrs::blocks::header::Header;

#[component]
pub fn DocProgressExamples() -> impl IntoView {
    view! {
        <div style="padding: var(--space-xl); max-width: 720px;">
            <h2>"DocProgress"</h2>
            <p>"Reading progress indicator. Two modes depending on whether the page has a sticky header."</p>

            // ── Mode 1: Standalone ────────────────────────────────────────────
            <section style="margin-top: var(--space-2xl);">
                <h3>"Mode 1 — Standalone"</h3>
                <p>
                    "Use when the page has NO sticky header. "
                    "The bar renders at " <code>"position: fixed; top: 0"</code> " automatically. "
                    "No configuration needed."
                </p>

                <pre style="background: var(--theme-surface-muted); padding: var(--space-md); border-radius: var(--radius-md); margin-top: var(--space-md); overflow-x: auto;">
                    <code>
"use canonrs::ui::doc_progress::DocProgress;

view! {
    <DocProgress/>
    // page content
}"
                    </code>
                </pre>

                <div style="margin-top: var(--space-md); padding: var(--space-sm) var(--space-md); background: var(--theme-surface-muted); border-radius: var(--radius-sm); font-size: var(--font-size-xs); color: var(--theme-surface-fg-muted);">
                    "↑ Live demo — scroll the page to see the bar fill at the top of the viewport"
                </div>

                <DocProgress />
            </section>

            // ── Mode 2: Portal ────────────────────────────────────────────────
            <section style="margin-top: var(--space-2xl);">
                <h3>"Mode 2 — Portal (sticky header)"</h3>
                <p>
                    "Use when the page HAS a sticky header. "
                    "Pass " <code>"DocProgressSlot"</code> " into the Header " <code>"bottom"</code> " slot. "
                    "The bar anchors to the bottom edge of the sticky header."
                </p>

                <pre style="background: var(--theme-surface-muted); padding: var(--space-md); border-radius: var(--radius-md); margin-top: var(--space-md); overflow-x: auto;">
                    <code>
"use canonrs::ui::doc_progress::DocProgressSlot;
use canonrs::blocks::header::Header;

view! {
    <Header
        bottom=leptos::children::ToChildren::to_children(|| view! {
            <DocProgressSlot/>
        })
        logo=...
        nav=...
        actions=...
    />
    // page content
}"
                    </code>
                </pre>

                // Live demo
                <div style="margin-top: var(--space-md); border: 1px solid var(--theme-surface-border); border-radius: var(--radius-md); overflow: hidden;">
                    <div style="padding: var(--space-xs) var(--space-md); background: var(--theme-surface-muted); font-size: var(--font-size-xs); color: var(--theme-surface-fg-muted);">
                        "↓ Live demo — sticky header with progress bar on its bottom edge"
                    </div>
                    <Header
                        bottom=leptos::children::ToChildren::to_children(|| view! {
                            <DocProgressSlot/>
                        })
                        logo=leptos::children::ToChildren::to_children(|| view! {
                            <span style="font-weight: var(--font-weight-bold); font-size: var(--font-size-sm);">"MyApp"</span>
                        })
                        nav=leptos::children::ToChildren::to_children(|| view! {
                            <span style="color: var(--theme-surface-fg-muted); font-size: var(--font-size-sm);">"Docs"</span>
                        })
                    />
                </div>
            </section>

            // ── Scroll content ────────────────────────────────────────────────
            <div style="height: 150vh; margin-top: var(--space-2xl);">
                {(0..30).map(|i| view! {
                    <p style="margin: var(--space-md) 0; color: var(--theme-surface-fg-muted); font-size: var(--font-size-sm);">
                        "Paragraph " {i + 1} " — scroll to see the progress bar update."
                    </p>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
