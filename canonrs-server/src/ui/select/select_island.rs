//! Select Island — Canon Rule passthrough
use leptos::prelude::*;
use super::select_ui::{Select, SelectTrigger, SelectValue, SelectContent};

#[component]
pub fn SelectIsland(
    children: Children,
    #[prop(into, default = String::from("Select..."))] placeholder: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Select class=class>
            <SelectTrigger>
                <SelectValue placeholder=placeholder>{""}</SelectValue>
            </SelectTrigger>
            <SelectContent>
                {children()}
            </SelectContent>
        </Select>
    }
}
