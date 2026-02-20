//! # Drawer Block
//! Slide-out panel overlay

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum DrawerPosition {
    Left,
    Right,
    Top,
    Bottom,
}

impl DrawerPosition {
    fn as_str(&self) -> &'static str {
        match self {
            DrawerPosition::Left => "left",
            DrawerPosition::Right => "right",
            DrawerPosition::Top => "top",
            DrawerPosition::Bottom => "bottom",
        }
    }
}

#[component]
pub fn DrawerBlock(
    #[prop(into)] open: MaybeSignal<bool>,
    #[prop(optional)] on_close: Option<Callback<()>>,
    #[prop(default = DrawerPosition::Right)] position: DrawerPosition,
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(optional)] close_button: Option<ChildrenFn>,
    #[prop(default = true)] backdrop: bool,
    #[prop(default = true)] close_on_backdrop: bool,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    let handle_backdrop_click = move |_| {
        if close_on_backdrop {
            if let Some(cb) = on_close {
                cb.run(());
            }
        }
    };

    view! {
        <div
            class=move || format!(
                "canon-drawer-overlay {}",
                if open.get() { "canon-drawer-overlay--open" } else { "" }
            )
            attr:data-block="drawer"
            attr:data-action="close-backdrop" on:click=handle_backdrop_click
        >
            {backdrop.then(|| view! {
                <div class="canon-drawer__backdrop" />
            })}

            <div
                class=format!("canon-drawer canon-drawer--{} {}", position.as_str(), class)
                attr:data-action="prevent-close" on:click=|e| e.stop_propagation()
            >
                {header.map(|h| view! {
                    <div class="canon-drawer__header">
                        {h()}
                        {close_button.map(|btn| view! {
                            <div class="canon-drawer__close">{btn()}</div>
                        })}
                    </div>
                })}

                <div class="canon-drawer__content">
                    {children()}
                </div>

                {footer.map(|f| view! {
                    <div class="canon-drawer__footer">{f()}</div>
                })}
            </div>
        </div>
    }
}
