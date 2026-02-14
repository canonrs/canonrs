use leptos::prelude::*;
use super::alert_ui::*;
use crate::primitives::AlertVariant;

pub fn basic_example() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <Alert variant=AlertVariant::Default>
                <AlertTitle>"Info"</AlertTitle>
                <AlertDescription>"This is a default alert message."</AlertDescription>
            </Alert>
            <Alert variant=AlertVariant::Success>
                <AlertTitle>"Success"</AlertTitle>
                <AlertDescription>"Your action was completed successfully."</AlertDescription>
            </Alert>
            <Alert variant=AlertVariant::Warning>
                <AlertTitle>"Warning"</AlertTitle>
                <AlertDescription>"Please review before continuing."</AlertDescription>
            </Alert>
            <Alert variant=AlertVariant::Destructive>
                <AlertTitle>"Error"</AlertTitle>
                <AlertDescription>"Something went wrong. Please try again."</AlertDescription>
            </Alert>
        </div>
    }
}
