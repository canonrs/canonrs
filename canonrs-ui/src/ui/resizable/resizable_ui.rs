use leptos::prelude::*;
use crate::primitives::{ResizablePrimitive, ResizablePanelPrimitive, ResizableHandlePrimitive};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResizableDirection {
    Horizontal,
    Vertical,
}

impl ResizableDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[component]
pub fn Resizable(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = ResizableDirection::Horizontal)] direction: ResizableDirection,
    #[prop(default = 20)] min_size: u32,
    #[prop(default = 80)] max_size: u32,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <ResizablePrimitive
            class={class}
            id={id.unwrap_or_default()}
        >
            <div
                data-resizable-wrapper=""
                data-direction={direction.as_str()}
                data-min-size={min_size.to_string()}
                data-max-size={max_size.to_string()}
            >
                {children.map(|c| c())}
            </div>
        </ResizablePrimitive>
    }
}

#[component]
pub fn ResizablePanel(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = 50)] default_size: u32,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <ResizablePanelPrimitive
            class={class}
            id={id.unwrap_or_default()}
        >
            <div
                data-resizable-panel-content=""
                data-size={default_size.to_string()}
                style={format!("flex-basis: {}%", default_size)}
            >
                {children.map(|c| c())}
            </div>
        </ResizablePanelPrimitive>
    }
}

#[component]
pub fn ResizableHandle() -> impl IntoView {
    view! {
        <ResizableHandlePrimitive />
    }
}
