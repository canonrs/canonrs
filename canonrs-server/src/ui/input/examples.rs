use leptos::prelude::*;
use super::{Input, InputVariant, InputSize};

#[component]
pub fn basic_example() -> impl IntoView {
    view! {
        <Input placeholder="Enter your name...".to_string() />
    }
}

#[component]
pub fn variants_example() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <Input variant=InputVariant::Default placeholder="Default input".to_string() />
            <Input variant=InputVariant::Success placeholder="Success input".to_string() />
            <Input variant=InputVariant::Error placeholder="Error input".to_string() />
        </div>
    }
}

#[component]
pub fn sizes_example() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <Input size=InputSize::Sm placeholder="Small input".to_string() />
            <Input size=InputSize::Md placeholder="Medium input".to_string() />
            <Input size=InputSize::Lg placeholder="Large input".to_string() />
        </div>
    }
}

#[component]
pub fn types_example() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <Input input_type="text".to_string() placeholder="Text input".to_string() />
            <Input input_type="email".to_string() placeholder="Email input".to_string() />
            <Input input_type="password".to_string() placeholder="Password input".to_string() />
            <Input input_type="number".to_string() placeholder="Number input".to_string() />
        </div>
    }
}

#[component]
pub fn disabled_example() -> impl IntoView {
    view! {
        <Input placeholder="Disabled input".to_string() disabled=true />
    }
}
