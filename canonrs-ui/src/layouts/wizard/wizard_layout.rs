//! # WizardLayout
//! 
//! **Purpose:** Step-oriented processes, guided flows
//! **Coverage:** Setup, Onboarding, Multi-step forms, Configurations
//! 
//! **Structure:**
//! ```
//! Header (branding)
//! Stepper (progress indicator)
//! Content (current step)
//! Footer (navigation actions)
//! ```
//! 
//! **Token Family:** Layout (layout.wizard.*)

use leptos::prelude::*;

#[component]
pub fn WizardLayout(
    /// Header content (branding, title)
    header: Children,
    /// Stepper/progress indicator
    stepper: Children,
    /// Current step content
    children: Children,
    /// Footer actions (Back, Next, Cancel)
    footer: Children,
) -> impl IntoView {
    view! {
        <div
            class="layout-wizard"
            attr:data-layout="wizard"
        >
            <header
                class="layout-wizard-header"
                attr:data-layout-region="header"
            >
                {header()}
            </header>

            <div
                class="layout-wizard-stepper"
                attr:data-layout-region="stepper"
            >
                {stepper()}
            </div>

            <main
                class="layout-wizard-content"
                attr:data-layout-region="content"
            >
                {children()}
            </main>

            <footer
                class="layout-wizard-footer"
                attr:data-layout-region="footer"
            >
                {footer()}
            </footer>
        </div>
    }
}
