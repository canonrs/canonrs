use leptos::prelude::*;
use crate::layouts::section::Section;
use crate::blocks::list::{List, ListItem};

#[component]
pub fn UsageBlock(
    when_to_use: Vec<String>,
    when_not_to_use: Vec<String>,
) -> impl IntoView {
    view! {
        <Section class="canon-usage">
            <div class="canon-usage-section">
                <h3 class="canon-usage-section-title">"When to use"</h3>
                <List>
                    {when_to_use.into_iter().map(|item| view! {
                        <ListItem class="canon-usage-item">
                            <p class="canon-usage-description">{item}</p>
                        </ListItem>
                    }).collect_view()}
                </List>
            </div>

            <div class="canon-usage-section">
                <h3 class="canon-usage-section-title">"When NOT to use"</h3>
                <List>
                    {when_not_to_use.into_iter().map(|item| view! {
                        <ListItem class="canon-usage-item canon-usage-item--negative">
                            <p class="canon-usage-description">{item}</p>
                        </ListItem>
                    }).collect_view()}
                </List>
            </div>
        </Section>
    }
}
