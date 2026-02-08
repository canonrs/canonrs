use leptos::prelude::*;
use super::resizable_primitive::ResizablePrimitive;

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
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = ResizableDirection::Horizontal)] direction: ResizableDirection,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let base_class = format!("resizable {}", class);

    view! {
        <ResizablePrimitive
            id={id}
            class={base_class}
            direction={direction.as_str().to_string()}
        >
            {children.map(|c| c())}
        </ResizablePrimitive>
    }
}
