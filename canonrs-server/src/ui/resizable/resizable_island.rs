use leptos::prelude::*;
use super::resizable_ui::{Resizable, ResizablePanel, ResizableHandle};
use canonrs_core::primitives::ResizableOrientation;

/// Island minimo — só inicializa o JS, sem wrapper de layout
#[island]
pub fn ResizableInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        let f = Closure::wrap(Box::new(move || {
            crate::interactions::resizable::init_all();
        }) as Box<dyn Fn()>);
        leptos::web_sys::window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .ok();
        f.forget();
    }
    view! { <></> }
}

/// Componentes SSR normais — sem island wrapper
#[component]
pub fn ResizableIsland(
    children: Children,
    #[prop(optional, into)] orientation: Option<String>,
    #[prop(optional)] min_size: Option<u32>,
    #[prop(optional)] max_size: Option<u32>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let orientation_prop = match orientation.as_deref() {
        Some("vertical") => ResizableOrientation::Vertical,
        _ => ResizableOrientation::Horizontal,
    };
    let cls = class.unwrap_or_default();
    view! {
        <ResizableInit />
        <Resizable orientation=orientation_prop min_size=min_size.unwrap_or(20) max_size=max_size.unwrap_or(80) class=cls>
            {children()}
        </Resizable>
    }
}

#[component]
pub fn ResizablePanelIsland(
    children: Children,
    #[prop(optional)] default_size: Option<u32>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let size = default_size.unwrap_or(50);
    let cls = class.unwrap_or_default();
    view! {
        <ResizablePanel default_size=size class=cls>
            {children()}
        </ResizablePanel>
    }
}

#[component]
pub fn ResizableHandleIsland(
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let cls = class.unwrap_or_default();
    view! { <ResizableHandle class=cls /> }
}
