use leptos::prelude::*;
use super::{ErrorState, ErrorStateTitle, ErrorStateDescription, ErrorStateActions};
use crate::ui::button::{Button, ButtonVariant};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <ErrorState>
            <ErrorStateTitle>"Something went wrong"</ErrorStateTitle>
            <ErrorStateDescription>"We encountered an error. Please try again."</ErrorStateDescription>
            <ErrorStateActions>
                <Button variant=ButtonVariant::Solid>"Retry"</Button>
            </ErrorStateActions>
        </ErrorState>
    }
}
