use leptos::prelude::*;

#[derive(Clone, PartialEq)]
pub enum CardVariant {
    Default,
    Interactive,
}

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
    #[prop(default = CardVariant::Default)]
    variant: CardVariant,
    #[prop(optional)]
    header: Option<Children>,
    #[prop(optional)]
    footer: Option<Children>,
    #[prop(optional)]
    card_id: Option<String>,
    #[prop(default = String::new(), into)]
    class: String,
    children: Children,
) -> impl IntoView {
    let is_interactive = variant == CardVariant::Interactive;
    let variant_str = variant.as_str();

    view! {
        <div
            class=class
            data-card=""
            data-variant=variant_str
            data-card-id=card_id
            role=if is_interactive { Some("button") } else { None }
            tabindex=if is_interactive { Some("0") } else { None }
        >
            {header.map(|h| view! {
                <div data-card-header="">
                    {h()}
                </div>
            })}

            <div data-card-content="">
                {children()}
            </div>

            {footer.map(|f| view! {
                <div data-card-footer="">
                    {f()}
                </div>
            })}
        </div>
    }
}
