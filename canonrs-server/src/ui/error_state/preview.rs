use leptos::prelude::*;
use super::error_state_island::{
    ErrorStateIsland, ErrorStateIconIsland, ErrorStateTitleIsland,
    ErrorStateDescriptionIsland, ErrorStateActionsIsland,
};

#[component]
pub fn ErrorStateShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ErrorStateIsland>
                    <ErrorStateIconIsland>"⚠️"</ErrorStateIconIsland>
                    <ErrorStateTitleIsland>"Something went wrong"</ErrorStateTitleIsland>
                    <ErrorStateDescriptionIsland>"We encountered an unexpected error. Please try again."</ErrorStateDescriptionIsland>
                    <ErrorStateActionsIsland>"Retry"</ErrorStateActionsIsland>
                </ErrorStateIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Error feedback always announced and structurally consistent."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Examples"</span>
                <div data-rs-showcase-preview-row="">
                    <ErrorStateIsland>
                        <ErrorStateIconIsland>"🔌"</ErrorStateIconIsland>
                        <ErrorStateTitleIsland>"Connection failed"</ErrorStateTitleIsland>
                        <ErrorStateDescriptionIsland>"Check your network and try again."</ErrorStateDescriptionIsland>
                        <ErrorStateActionsIsland>"Reconnect"</ErrorStateActionsIsland>
                    </ErrorStateIsland>
                    <ErrorStateIsland>
                        <ErrorStateIconIsland>"🔒"</ErrorStateIconIsland>
                        <ErrorStateTitleIsland>"Access denied"</ErrorStateTitleIsland>
                        <ErrorStateDescriptionIsland>"You do not have permission to view this page."</ErrorStateDescriptionIsland>
                        <ErrorStateActionsIsland>"Go back"</ErrorStateActionsIsland>
                    </ErrorStateIsland>
                    <ErrorStateIsland>
                        <ErrorStateIconIsland>"🗄️"</ErrorStateIconIsland>
                        <ErrorStateTitleIsland>"Failed to load data"</ErrorStateTitleIsland>
                        <ErrorStateDescriptionIsland>"The requested data could not be fetched."</ErrorStateDescriptionIsland>
                        <ErrorStateActionsIsland>"Try again"</ErrorStateActionsIsland>
                    </ErrorStateIsland>
                </div>
            </div>
        </div>
    }
}
