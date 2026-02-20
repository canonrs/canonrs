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
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <ResizablePrimitive class={class} id={id}>
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
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <ResizablePanelPrimitive class={class} id={id}>
            <div
                data-resizable-panel-content=""
                data-size={default_size.to_string()}
            >
                {children.map(|c| c())}
            </div>
        </ResizablePanelPrimitive>
    }
}

#[component]
pub fn ResizableHandle(
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <ResizableHandlePrimitive id={id} />
    }
}
