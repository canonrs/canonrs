use crate::primitives::LabelPrimitive;
use crate::tokens::SEMANTIC;
use leptos::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Label(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] html_for: String,
    #[prop(optional)] disabled: Option<bool>,
    children: Children,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);

    let peer_classes = if !html_for.is_empty() {
        format!(
            "peer-disabled/{}:cursor-not-allowed peer-disabled/{}:opacity-50",
            html_for, html_for
        )
    } else {
        "peer-disabled:cursor-not-allowed peer-disabled:opacity-50".to_string()
    };

    let merged_class = tw_merge!(
        "flex items-center gap-2 text-sm leading-none font-medium select-none",
        format!("text-[{}]", SEMANTIC.foreground),
        "group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50",
        &peer_classes,
        class
    );

    view! {
        <LabelPrimitive html_for=html_for.clone() disabled=disabled>
            <label class=merged_class r#for=html_for>
                {children()}
            </label>
        </LabelPrimitive>
    }
}
