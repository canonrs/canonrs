use leptos::prelude::*;
use crate::blocks::Card;

pub fn nested_example() -> impl IntoView {
    view! {
        <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 1rem;">
            <Card>
                <div data-card-header>
                    <h4 style="font-weight: 600;">"Nested Card 1"</h4>
                </div>
                <div data-card-content>
                    <p style="font-size: 0.875rem;">"Cards can be nested inside other cards for complex layouts."</p>
                </div>
            </Card>
            <Card>
                <div data-card-header>
                    <h4 style="font-weight: 600;">"Nested Card 2"</h4>
                </div>
                <div data-card-content>
                    <p style="font-size: 0.875rem;">"This demonstrates Card composition in Family F."</p>
                </div>
            </Card>
        </div>
    }
}
