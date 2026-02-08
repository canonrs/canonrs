use leptos::prelude::*;
use crate::layouts::section::Section;
use crate::blocks::table::Table;

#[derive(Clone, Debug)]
pub struct ApiProp {
    pub name: String,
    pub prop_type: String,
    pub description: String,
    pub required: bool,
}

#[component]
pub fn ApiBlock(props: Vec<ApiProp>) -> impl IntoView {
    view! {
        <Section>
            <h3 class="canon-api-title">"API"</h3>
            
            <Table striped=true>
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Type"</th>
                        <th>"Required"</th>
                        <th>"Description"</th>
                    </tr>
                </thead>
                <tbody>
                    {props.into_iter().map(|prop| view! {
                        <tr>
                            <td><code>{prop.name}</code></td>
                            <td><code>{prop.prop_type}</code></td>
                            <td>{if prop.required { "✓" } else { "" }}</td>
                            <td>{prop.description}</td>
                        </tr>
                    }).collect_view()}
                </tbody>
            </Table>
        </Section>
    }
}
