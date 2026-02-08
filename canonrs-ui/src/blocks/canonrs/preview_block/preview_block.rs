use leptos::prelude::*;
use crate::layouts::section::Section;
use crate::blocks::card::Card;

#[component]
pub fn PreviewBlock(
    children: Children,
) -> impl IntoView {
    view! {
        <Section class="canon-preview">
            <Card class="canon-preview-surface">
                {children()}
            </Card>
        </Section>
    }
}
