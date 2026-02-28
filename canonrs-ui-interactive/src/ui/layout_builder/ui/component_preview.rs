use leptos::prelude::*;
use canonrs_ui::ui::button::button_ui::{Button, ButtonVariant, ButtonSize};
use canonrs_ui::ui::badge::badge_ui::{Badge, BadgeVariant};
use canonrs_ui::ui::icon::icon_ui::IconPreview;
use canonrs_ui::ui::separator::separator_ui::Separator;
use canonrs_ui::ui::spinner::spinner_ui::Spinner;
use canonrs_ui::ui::label::label_ui::Label;
use canonrs_ui::ui::progress::progress_ui::Progress;
use canonrs_ui::ui::kbd::kbd_ui::Kbd;
use canonrs_ui::ui::link::link_ui::Link;
use canonrs_ui::primitives::SpinnerSize;

#[component]
pub fn ComponentPreview(id: String) -> impl IntoView {
    if id == "button" {
        view! { <Button variant=ButtonVariant::Solid size=ButtonSize::Md>"Button"</Button> }.into_any()
    } else if id == "icon" {
        view! { <IconPreview /> }.into_any()
    } else if id == "badge" {
        view! { <Badge variant=BadgeVariant::Default>"Badge"</Badge> }.into_any()
    } else if id == "separator" {
        view! { <Separator /> }.into_any()
    } else if id == "spinner" {
        view! { <Spinner size=SpinnerSize::Medium /> }.into_any()
    } else if id == "label" {
        view! { <Label>"Label"</Label> }.into_any()
    } else if id == "progress" {
        view! { <Progress value=50.0 /> }.into_any()
    } else if id == "kbd" {
        view! { <Kbd>"⌘K"</Kbd> }.into_any()
    } else if id == "link" {
        view! { <Link href="#".to_string()>"Link"</Link> }.into_any()
    } else {
        view! { <span style="font-size:0.75rem;color:var(--theme-surface-fg-muted);padding:2px 6px;border:1px dashed currentColor;border-radius:4px;">{id}</span> }.into_any()
    }
}

pub fn render_component_preview(id: &str) -> AnyView {
    let id = id.to_string();
    view! { <ComponentPreview id=id /> }.into_any()
}
