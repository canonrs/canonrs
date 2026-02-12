use leptos::prelude::*;
use super::link_ui::*;
use crate::primitives::LinkVariant;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1.5rem;">
            <div>
                <h4>"Default Link"</h4>
                <p>
                    "Visit our "
                    <Link href="/docs".to_string()>"documentation"</Link>
                    " to learn more."
                </p>
            </div>

            <div>
                <h4>"Muted Link"</h4>
                <p>
                    "Read the "
                    <Link href="/privacy".to_string() variant=LinkVariant::Muted>"privacy policy"</Link>
                    " for details."
                </p>
            </div>

            <div>
                <h4>"Underline Link"</h4>
                <p>
                    "Check our "
                    <Link href="/terms".to_string() variant=LinkVariant::Underline>"terms of service"</Link>
                    "."
                </p>
            </div>

            <div>
                <h4>"External Link"</h4>
                <p>
                    "Follow us on "
                    <Link href="https://github.com".to_string() external=true>"GitHub"</Link>
                    "."
                </p>
            </div>

            <div>
                <h4>"Disabled Link"</h4>
                <p>
                    "This feature is "
                    <Link href="#".to_string() disabled=true>"coming soon"</Link>
                    "."
                </p>
            </div>
        </div>
    }
}
