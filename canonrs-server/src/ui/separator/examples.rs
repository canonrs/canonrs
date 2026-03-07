use leptos::prelude::*;
use super::separator_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <div>
                <p>"Section A"</p>
                <Separator />
                <p>"Section B"</p>
            </div>
            <div>
                <p>"Horizontal (default)"</p>
                <Separator decorative=true />
            </div>
        </div>
    }
}
