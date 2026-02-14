use leptos::prelude::*;
use super::{Callout, CalloutTitle, CalloutDescription, CalloutVariant};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <Callout variant=CalloutVariant::Default>
                <CalloutTitle>"Default"</CalloutTitle>
                <CalloutDescription>"This is a default callout for general information."</CalloutDescription>
            </Callout>

            <Callout variant=CalloutVariant::Info>
                <CalloutTitle>"Information"</CalloutTitle>
                <CalloutDescription>"New features are available in the latest release."</CalloutDescription>
            </Callout>

            <Callout variant=CalloutVariant::Success>
                <CalloutTitle>"Success"</CalloutTitle>
                <CalloutDescription>"Your changes have been saved successfully."</CalloutDescription>
            </Callout>

            <Callout variant=CalloutVariant::Warning>
                <CalloutTitle>"Warning"</CalloutTitle>
                <CalloutDescription>"This action cannot be undone. Please review before proceeding."</CalloutDescription>
            </Callout>

            <Callout variant=CalloutVariant::Error>
                <CalloutTitle>"Error"</CalloutTitle>
                <CalloutDescription>"Unable to process request. Please check your connection."</CalloutDescription>
            </Callout>
        </div>
    }
}
