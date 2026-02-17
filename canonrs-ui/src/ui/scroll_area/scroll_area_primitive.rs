//! @canon-level: strict
//! ScrollArea Primitive - Viewport + Scrollbar + Thumb enterprise architecture

use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ScrollOrientation {
    #[default]
    Vertical,
    Horizontal,
    Both,
}

impl ScrollOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Vertical   => "vertical",
            Self::Horizontal => "horizontal",
            Self::Both       => "both",
        }
    }
}

#[component]
pub fn ScrollAreaPrimitive(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = ScrollOrientation::Vertical)] orientation: ScrollOrientation,
    #[prop(default = true)] auto_hide: bool,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let show_v = matches!(orientation, ScrollOrientation::Vertical | ScrollOrientation::Both);
    let show_h = matches!(orientation, ScrollOrientation::Horizontal | ScrollOrientation::Both);

    view! {
        <div
            id={id}
            class={class}
            data-scroll-area=""
            data-orientation={orientation.as_str()}
            data-auto-hide={auto_hide.to_string()}
        >
            <div data-scroll-viewport="">
                {children.map(|c| c())}
            </div>

            {show_v.then(|| view! {
                <div data-scrollbar="" data-orientation="vertical">
                    <div data-scroll-thumb="" data-orientation="vertical" />
                </div>
            })}

            {show_h.then(|| view! {
                <div data-scrollbar="" data-orientation="horizontal">
                    <div data-scroll-thumb="" data-orientation="horizontal" />
                </div>
            })}
        </div>
    }
}
