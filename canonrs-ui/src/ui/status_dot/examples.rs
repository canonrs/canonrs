use leptos::prelude::*;
use super::{StatusDot, StatusDotVariant};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1.5rem;">
            <div style="display: flex; align-items: center; gap: 0.75rem;">
                <StatusDot variant=StatusDotVariant::Online />
                <span>"Online"</span>
            </div>

            <div style="display: flex; align-items: center; gap: 0.75rem;">
                <StatusDot variant=StatusDotVariant::Offline />
                <span>"Offline"</span>
            </div>

            <div style="display: flex; align-items: center; gap: 0.75rem;">
                <StatusDot variant=StatusDotVariant::Away />
                <span>"Away"</span>
            </div>

            <div style="display: flex; align-items: center; gap: 0.75rem;">
                <StatusDot variant=StatusDotVariant::Busy />
                <span>"Busy"</span>
            </div>

            <div style="display: flex; align-items: center; gap: 0.75rem;">
                <StatusDot variant=StatusDotVariant::DoNotDisturb />
                <span>"Do Not Disturb"</span>
            </div>
        </div>
    }
}
