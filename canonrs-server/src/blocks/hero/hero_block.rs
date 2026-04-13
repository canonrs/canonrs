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
    #[prop(optional)] content: Option<ChildrenFn>,
    #[prop(optional)] actions: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(default = HeroVariant::Centered)] variant: HeroVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid = generate("bl");
    #[cfg(debug_assertions)]
    {
        let provided: &[&str] = &[
            if content.is_some() { "content" } else { "" },
            if header.is_some() { "header" } else { "" },
            if media.is_some() { "media" } else { "" },
            if actions.is_some() { "actions" } else { "" },
            if footer.is_some() { "footer" } else { "" },
        ];
        canonrs_core::validate_block_regions!("hero", provided);
    }
    let header = StoredValue::new(header);
    let media = StoredValue::new(media);
    let content = StoredValue::new(content);
    let actions = StoredValue::new(actions);
    let footer = StoredValue::new(footer);
    view! {
        <div data-rs-hero="" data-rs-uid=uid data-rs-variant=variant.as_str() class=class>
            {move || header.get_value().map(|h| view! { <div data-rs-region="header">{h()}</div> })}
            {move || media.get_value().map(|m| view! { <div data-rs-region="media">{m()}</div> })}
            {move || content.get_value().map(|c| view! { <div data-rs-region="content">{c()}</div> })}
            {move || actions.get_value().map(|a| view! { <div data-rs-region="actions">{a()}</div> })}
            {move || footer.get_value().map(|f| view! { <div data-rs-region="footer">{f()}</div> })}
        </div>
    }
}
