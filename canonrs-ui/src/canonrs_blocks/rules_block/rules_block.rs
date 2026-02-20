use leptos::prelude::*;
use crate::layouts::section::Section;
use crate::blocks::list::{List, ListItem};

#[component]
pub fn RulesBlock(
    rules: Vec<String>,
    anti_patterns: Vec<String>,
) -> impl IntoView {
    view! {
        <Section class="canon-rules">
            <div class="canon-rules-section">
                <h3 class="canon-rules-title">"Canon Rules"</h3>
                <List>
                    {rules.into_iter().map(|rule| view! {
                        <ListItem class="canon-rule-item">{rule}</ListItem>
                    }).collect_view()}
                </List>
            </div>

            <div class="canon-rules-section canon-rules-negative">
                <h3 class="canon-rules-title">"Anti-patterns"</h3>
                <List>
                    {anti_patterns.into_iter().map(|pattern| view! {
                        <ListItem class="canon-antipattern-item">{pattern}</ListItem>
                    }).collect_view()}
                </List>
            </div>
        </Section>
    }
}
