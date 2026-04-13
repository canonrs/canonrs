use leptos::prelude::*;
use canonrs_core::infra::uid::generate;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum HeroVariant {
    #[default]
    Centered,
    Split,
    MediaTop,
}
impl HeroVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Centered => "centered",
            Self::Split    => "split",
            Self::MediaTop => "media-top",
        }
    }
}

#[component]
pub fn Hero(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] media: Option<ChildrenFn>,
    content: ChildrenFn,
    #[prop(optional)] actions: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(default = HeroVariant::Centered)] variant: HeroVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid = generate("bl");
    let header = StoredValue::new(header);
    let media = StoredValue::new(media);
    let content = StoredValue::new(content);
    let actions = StoredValue::new(actions);
    let footer = StoredValue::new(footer);
    view! {
        <div data-rs-hero="" data-rs-uid=uid data-rs-variant=variant.as_str() class=class>
            {move || header.get_value().map(|h| view! { <div data-rs-region="header">{h()}</div> })}
            {move || media.get_value().map(|m| view! { <div data-rs-region="media">{m()}</div> })}
            <div data-rs-region="content">{move || content.get_value()()}</div>
            {move || actions.get_value().map(|a| view! { <div data-rs-region="actions">{a()}</div> })}
            {move || footer.get_value().map(|f| view! { <div data-rs-region="footer">{f()}</div> })}
        </div>
    }
}
