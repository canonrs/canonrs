use leptos::prelude::*;
use crate::layouts::section::Section;

#[component]
pub fn ComponentHeader(
    name: String,
    description: String,
) -> impl IntoView {
    view! {
        <Section class="canon-component-hero">
            <div class="canon-component-hero-inner">
                <div class="canon-component-hero-text">
                    <h1 class="canon-component-title">{name}</h1>
                    <p class="canon-component-description">{description}</p>
                </div>
            </div>
        </Section>
    }
}
