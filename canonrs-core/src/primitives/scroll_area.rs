//! @canon-level: strict
//! @canon-owner: primitives-team
//! ScrollArea Primitive - HTML puro

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
    children: Children,
    #[prop(default = ScrollOrientation::Vertical)] orientation: ScrollOrientation,
    #[prop(default = true)] auto_hide: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] viewport_id: Option<String>,
) -> impl IntoView {
    let show_v = matches!(orientation, ScrollOrientation::Vertical | ScrollOrientation::Both);
    let show_h = matches!(orientation, ScrollOrientation::Horizontal | ScrollOrientation::Both);

    view! {
        <div
            data-rs-scroll-area=""
            data-rs-component="ScrollArea"
            data-rs-behavior="scroll"
            data-rs-orientation=orientation.as_str()
            data-rs-auto-hide={auto_hide.then_some("")}
            role="region"
            class=class
        >
            <div
                data-rs-scroll-viewport=""
                id={viewport_id}
                role="presentation"
                tabindex="0"
            >
                {children()}
            </div>

            {show_v.then(|| view! {
                <div data-rs-scrollbar="" data-rs-orientation="vertical" role="scrollbar" aria-orientation="vertical">
                    <div data-rs-scroll-track="">
                        <div data-rs-scroll-thumb="" data-rs-orientation="vertical" />
                    </div>
                </div>
            })}

            {show_h.then(|| view! {
                <div data-rs-scrollbar="" data-rs-orientation="horizontal" role="scrollbar" aria-orientation="horizontal">
                    <div data-rs-scroll-track="">
                        <div data-rs-scroll-thumb="" data-rs-orientation="horizontal" />
                    </div>
                </div>
            })}
        </div>
    }
}
