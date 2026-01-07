//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum LinkVariant {
    Default,
    Muted,
    Underline,
}

#[component]
pub fn LinkPrimitive(
    #[prop(optional)] href: Option<String>,
    #[prop(optional)] variant: Option<LinkVariant>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] external: Option<bool>,
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or(LinkVariant::Default);
    let disabled = disabled.unwrap_or(false);
    let external = external.unwrap_or(false);

    view! {
        <a

            href=href
            data-variant=match variant {
                LinkVariant::Default => "default",
                LinkVariant::Muted => "muted",
                LinkVariant::Underline => "underline",
            }
            data-disabled=disabled
            target=if external { Some("_blank") } else { None }
            rel=if external { Some("noopener noreferrer") } else { None }
        >
            {children()}
        </a>
    }
}
