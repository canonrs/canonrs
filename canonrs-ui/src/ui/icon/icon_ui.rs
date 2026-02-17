use leptos::prelude::*;
use crate::primitives::IconPrimitive;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IconSize { Sm, Md, Lg }
impl IconSize {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Sm => "sm", Self::Md => "md", Self::Lg => "lg" }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IconVariant { Default, Muted, Primary, Destructive, Success, Warning }
impl IconVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Muted => "muted",
            Self::Primary => "primary",
            Self::Destructive => "destructive",
            Self::Success => "success",
            Self::Warning => "warning",
        }
    }
}

#[component]
pub fn Icon(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = IconSize::Md)] size: IconSize,
    #[prop(default = IconVariant::Default)] variant: IconVariant,
    #[prop(default = false)] spin: bool,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <span
            data-icon=""
            data-size={size.as_str()}
            data-variant={variant.as_str()}
            data-spin={spin.then_some("")}
            class={class}
            id={id.unwrap_or_default()}
        >
            <span data-icon-inner="" aria-hidden="true">
                {children.map(|c| c())}
            </span>
        </span>
    }
}
