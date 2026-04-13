use leptos::prelude::*;
use canonrs_core::infra::uid::generate;
use crate::ui::card::{Card, CardHeader, CardContent, CardFooter};

#[derive(Clone, Copy, PartialEq, Default)]
pub enum CardVariant {
    #[default] Default,
    Interactive,
    Outlined,
    Elevated,
}
impl CardVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default     => "default",
            Self::Interactive => "interactive",
            Self::Outlined    => "outlined",
            Self::Elevated    => "elevated",
        }
    }
}

#[component]
pub fn CardBlock(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] content: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(default = CardVariant::Default)] variant: CardVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid = generate("bl");
    let header = StoredValue::new(header);
    let content = StoredValue::new(content);
    let footer = StoredValue::new(footer);
    view! {
        <div attr:data-rs-card="" attr:data-rs-uid=uid>
        <Card variant=variant.as_str().to_string() class=class>
            {move || header.get_value().map(|h| view! { <CardHeader>{h()}</CardHeader> })}
            {move || content.get_value().map(|c| view! { <CardContent>{c()}</CardContent> })}
            {move || footer.get_value().map(|f| view! { <CardFooter>{f()}</CardFooter> })}
        </Card>
        </div>
    }
}
