//! DocProgress Examples

use leptos::prelude::*;
use super::DocProgress;

#[component]
pub fn DocProgressExamples() -> impl IntoView {
    view! {
        <div style="padding: var(--space-xl);">
            <h2>"DocProgress - Reading Progress Bar"</h2>
            
            <section style="margin-top: var(--space-lg);">
                <h3>"Basic Usage"</h3>
                <p>"Fixed progress bar at top of page. Scroll to see it update."</p>
                
                <DocProgress />
                
                // Fake content to enable scrolling
                <div style="height: 200vh; padding: var(--space-xl); background: linear-gradient(to bottom, #f0f0f0, #ffffff);">
                    <h4>"Scroll down to see progress"</h4>
                    {(0..50).map(|i| view! {
                        <p style="margin: var(--space-md) 0;">
                            "Content paragraph " {i + 1} " - Keep scrolling to see the progress bar fill up at the top of the page."
                        </p>
                    }).collect::<Vec<_>>()}
                </div>
            </section>
        </div>
    }
}
