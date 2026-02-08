//! # DashboardLayout
//! **Canon Rule:** Layout fornece semântica + CSS, composição fica no app

use leptos::prelude::*;

#[component]
pub fn DashboardLayout(
    children: Children,
) -> impl IntoView {
    view! {
        <div data-layout="dashboard" class="layout-dashboard">
            {children()}
        </div>
    }
}
