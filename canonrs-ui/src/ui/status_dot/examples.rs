use leptos::prelude::*;
use super::{StatusDot, StatusDotVariant};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1.5rem;">
            <div style="display: flex; align-items: center; gap: 0.75rem;">
                <StatusDot variant=StatusDotVariant::Online aria_label="Online".to_string() />
                <span>"Online"</span>
            </div>

            <div style="display: flex; align-items: center; gap: 0.75rem;">
                <StatusDot variant=StatusDotVariant::Offline aria_label="Offline".to_string() />
                <span>"Offline"</span>
            </div>

            <div style="display: flex; align-items: center; gap: 0.75rem;">
                <StatusDot variant=StatusDotVariant::Away aria_label="Away".to_string() />
                <span>"Away"</span>
            </div>

            <div style="display: flex; align-items: center; gap: 0.75rem;">
                <StatusDot variant=StatusDotVariant::Busy aria_label="Busy".to_string() />
                <span>"Busy"</span>
            </div>

            <div style="display: flex; align-items: center; gap: 0.75rem;">
                <StatusDot variant=StatusDotVariant::DoNotDisturb aria_label="Do Not Disturb".to_string() />
                <span>"Do Not Disturb"</span>
            </div>
        </div>
    }
}
