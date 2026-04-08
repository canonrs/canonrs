//! Icon Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
use super::icon_ui::{Icon, IconSize, IconVariant};

#[component]
pub fn IconIsland(
    #[prop(optional)] children:                        Option<Children>,
    #[prop(default = IconSize::Md)] size:              IconSize,
    #[prop(default = IconVariant::Default)] variant:   IconVariant,
    #[prop(default = false)] spin:                     bool,
    #[prop(into, default = String::new())] class:      String,
    #[prop(into, optional)] id:                        Option<String>,
) -> impl IntoView {
    view! {
        <Icon size=size variant=variant spin=spin class=class id=id.unwrap_or_default()>
            {children.map(|c| c())}
        </Icon>
    }
}
