use leptos::prelude::*;
use super::error_state_boundary::{
    ErrorState, ErrorStateIcon, ErrorStateTitle,
    ErrorStateDescription, ErrorStateActions,
};

#[component]
pub fn ErrorStateShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ErrorState>
                    <ErrorStateIcon>"⚠️"</ErrorStateIcon>
                    <ErrorStateTitle>"Something went wrong"</ErrorStateTitle>
                    <ErrorStateDescription>"We encountered an unexpected error. Please try again."</ErrorStateDescription>
                    <ErrorStateActions>"Retry"</ErrorStateActions>
                </ErrorState>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Error feedback always announced and structurally consistent."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Examples"</span>
                <div data-rs-showcase-preview-row="">
                    <ErrorState>
                        <ErrorStateIcon>"🔌"</ErrorStateIcon>
                        <ErrorStateTitle>"Connection failed"</ErrorStateTitle>
                        <ErrorStateDescription>"Check your network and try again."</ErrorStateDescription>
                        <ErrorStateActions>"Reconnect"</ErrorStateActions>
                    </ErrorState>
                    <ErrorState>
                        <ErrorStateIcon>"🔒"</ErrorStateIcon>
                        <ErrorStateTitle>"Access denied"</ErrorStateTitle>
                        <ErrorStateDescription>"You do not have permission to view this page."</ErrorStateDescription>
                        <ErrorStateActions>"Go back"</ErrorStateActions>
                    </ErrorState>
                    <ErrorState>
                        <ErrorStateIcon>"🗄️"</ErrorStateIcon>
                        <ErrorStateTitle>"Failed to load data"</ErrorStateTitle>
                        <ErrorStateDescription>"The requested data could not be fetched."</ErrorStateDescription>
                        <ErrorStateActions>"Try again"</ErrorStateActions>
                    </ErrorState>
                </div>
            </div>
        </div>
    }
}
