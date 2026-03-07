use leptos::prelude::*;
use super::CopyButton;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 2rem;">
            <div>
                <h4>"Copy Static Text"</h4>
                <CopyButton
                    text="Hello, World!".to_string()
                    id="copy-static".to_string()
                />
            </div>

            <div>
                <h4>"Copy From Element"</h4>
                <pre id="code-snippet" style="padding: 1rem; background: var(--theme-surface-muted); border-radius: var(--radius-sm);">
                    "const greeting = 'Hello, Canon!';"
                </pre>
                <CopyButton
                    target="code-snippet".to_string()
                    id="copy-target".to_string()
                />
            </div>

            <div>
                <h4>"Custom Reset Delay (5s)"</h4>
                <CopyButton
                    text="This stays 'Copied!' for 5 seconds".to_string()
                    reset_delay=5000
                    id="copy-delay".to_string()
                />
            </div>
        </div>
    }
}
