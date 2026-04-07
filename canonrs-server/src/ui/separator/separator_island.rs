use leptos::prelude::*;
use super::separator_ui::Separator;
use canonrs_core::Orientation;

#[component]
pub fn SeparatorIsland(
    #[prop(optional, into)] orientation: Option<String>,
    #[prop(optional)] decorative: Option<bool>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    let orient = match orientation.as_deref() {
        Some("vertical") => Orientation::Vertical,
        _ => Orientation::Horizontal,
    };
    view! {
        <Separator
            orientation=orient
            decorative=decorative.unwrap_or(true)
            aria_label=aria_label.unwrap_or_default()
            class=class.unwrap_or_default()
            id=id.unwrap_or_default()
        />
    }
}
