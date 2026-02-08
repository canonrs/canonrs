use leptos::prelude::*;
use super::alert_ui::*;
use crate::primitives::AlertVariant;

pub fn basic_example() -> impl IntoView {
    view! {
        <Alert variant=AlertVariant::Success>
            <AlertTitle>"Success"</AlertTitle>
            <AlertDescription>"Your action was completed successfully."</AlertDescription>
        </Alert>
    }
}
