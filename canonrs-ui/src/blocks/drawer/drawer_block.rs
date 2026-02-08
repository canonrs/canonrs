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
    #[prop(optional)] header: Option<Children>,
    #[prop(optional)] footer: Option<Children>,
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
            attr:data-drawer-action="close-backdrop" on:click=handle_backdrop_click
        >
            {backdrop.then(|| view! {
                <div class="canon-drawer__backdrop" />
            })}
            
            <div 
                class=format!("canon-drawer canon-drawer--{} {}", position.as_str(), class)
                attr:data-drawer-action="prevent-close" on:click=|e| e.stop_propagation()
            >
                {header.map(|h| view! {
                    <div class="canon-drawer__header">
                        {h()}
                        <button 
                            class="canon-drawer__close"
                            attr:data-drawer-action="close" on:click=move |_| {
                                if let Some(cb) = on_close {
                                    cb.run(());
                                }
                            }
                        >
                            "×"
                        </button>
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
