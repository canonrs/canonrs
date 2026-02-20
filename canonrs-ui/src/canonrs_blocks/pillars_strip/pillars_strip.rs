use leptos::prelude::*;
use crate::layouts::section::Section;
use super::pillar::CanonPillar;

#[component]
pub fn PillarsStrip(pillars: &'static [CanonPillar]) -> impl IntoView {
    view! {
        <Section class="canon-pillars">
            <h2 class="canon-pillars-title">"Canonical Pillars"</h2>
            <ul class="canon-pillars-grid">
                {pillars.iter().map(|p| view! {
                    <li class="canon-pillar-item"
                        attr:data-pillar-id={p.id}
                        attr:data-pillar-doc={p.rule_doc}>
                        <img class="canon-pillar-icon" src={p.icon} alt={p.label} />
                        <span class="canon-pillar-label">{p.label}</span>
                    </li>
                }).collect_view()}
            </ul>
        </Section>
    }
}
