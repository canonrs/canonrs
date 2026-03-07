//! # Card Block

use leptos::prelude::*;

#[derive(Clone, PartialEq)]
pub enum CardVariant { Default, Interactive }

impl CardVariant {
    fn as_str(&self) -> &'static str {
        match self {
            CardVariant::Default => "default",
            CardVariant::Interactive => "interactive",
        }
    }
}

#[component]
pub fn Card(
    #[prop(default = CardVariant::Default)] variant: CardVariant,
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(optional, into)] card_id: Option<String>,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    let is_interactive = variant == CardVariant::Interactive;
    let variant_str = variant.as_str();

    view! {
        <div
            class=class
            data-block="card"
            data-block-version="1"
            data-variant=variant_str
            data-card-id=card_id
            role=if is_interactive { Some("button") } else { None }
            tabindex=if is_interactive { Some("0") } else { None }
        >
            {header.map(|h| view! {
                <div data-block-region="header">{h()}</div>
            })}
            <div data-block-region="content">{children()}</div>
            {footer.map(|f| view! {
                <div data-block-region="footer">{f()}</div>
            })}
        </div>
    }
}
