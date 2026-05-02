//! use_canvas — bridge entre canvas runtime (DOM) e Leptos signals
//! DOM is source of truth — signals sao reflexo do DOM

// CanvasState disponivel em SSR e hydrate
#[derive(Clone, Debug, Default)]
pub struct CanvasState {
    pub tool:     String,
    pub selected: Vec<u64>,
    pub state:    String,
}

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use leptos::prelude::*;

#[cfg(feature = "hydrate")]
pub fn use_canvas(signal: RwSignal<CanvasState>) -> NodeRef<leptos::html::Div> {
    let node_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        if let Some(el) = node_ref.get() {
            let root = el.as_ref() as &web_sys::Element;

            // rs:canvas:selected
            {
                let root2   = root.clone();
                let signal2 = signal.clone();
                let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    let json = root2
                        .get_attribute("data-rs-canvas-selected")
                        .unwrap_or_default();
                    let ids = serde_json::from_str::<Vec<u64>>(&json).unwrap_or_default();
                    signal2.update(|s| s.selected = ids);
                }) as Box<dyn FnMut(web_sys::Event)>);
                root.add_event_listener_with_callback(
                    "rs:canvas:selected", cb.as_ref().unchecked_ref()
                ).ok();
                cb.forget();
            }

            // rs:canvas:deselected
            {
                let signal2 = signal.clone();
                let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    signal2.update(|s| s.selected = vec![]);
                }) as Box<dyn FnMut(web_sys::Event)>);
                root.add_event_listener_with_callback(
                    "rs:canvas:deselected", cb.as_ref().unchecked_ref()
                ).ok();
                cb.forget();
            }

            // rs:canvas:tool
            {
                let root2   = root.clone();
                let signal2 = signal.clone();
                let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    let tool = root2
                        .get_attribute("data-rs-canvas-tool")
                        .unwrap_or_else(|| "select".to_string());
                    signal2.update(|s| s.tool = tool);
                }) as Box<dyn FnMut(web_sys::Event)>);
                root.add_event_listener_with_callback(
                    "rs:canvas:tool", cb.as_ref().unchecked_ref()
                ).ok();
                cb.forget();
            }

            // rs:canvas:changed
            {
                let root2   = root.clone();
                let signal2 = signal.clone();
                let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    let state = root2
                        .get_attribute("data-rs-canvas-state")
                        .unwrap_or_default();
                    signal2.update(|s| s.state = state);
                }) as Box<dyn FnMut(web_sys::Event)>);
                root.add_event_listener_with_callback(
                    "rs:canvas:changed", cb.as_ref().unchecked_ref()
                ).ok();
                cb.forget();
            }
        }
    });

    node_ref
}

#[cfg(not(feature = "hydrate"))]
pub fn use_canvas(
    _signal: leptos::prelude::RwSignal<CanvasState>,
) -> leptos::prelude::NodeRef<leptos::html::Div> {
    leptos::prelude::NodeRef::new()
}

#[cfg(feature = "hydrate")]
pub fn use_canvas_with_ref(signal: RwSignal<CanvasState>, node_ref: NodeRef<leptos::html::Div>) {
    Effect::new(move |_| {
        if let Some(el) = node_ref.get() {
            let root = el.as_ref() as &web_sys::Element;

            {
                let root2   = root.clone();
                let signal2 = signal.clone();
                let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    let json = root2.get_attribute("data-rs-canvas-selected").unwrap_or_default();
                    let ids  = serde_json::from_str::<Vec<u64>>(&json).unwrap_or_default();
                    signal2.update(|s| s.selected = ids);
                }) as Box<dyn FnMut(web_sys::Event)>);
                root.add_event_listener_with_callback("rs:canvas:selected", cb.as_ref().unchecked_ref()).ok();
                cb.forget();
            }
            {
                let signal2 = signal.clone();
                let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    signal2.update(|s| s.selected = vec![]);
                }) as Box<dyn FnMut(web_sys::Event)>);
                root.add_event_listener_with_callback("rs:canvas:deselected", cb.as_ref().unchecked_ref()).ok();
                cb.forget();
            }
            {
                let root2   = root.clone();
                let signal2 = signal.clone();
                let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    let tool = root2.get_attribute("data-rs-canvas-tool").unwrap_or_else(|| "select".to_string());
                    signal2.update(|s| s.tool = tool);
                }) as Box<dyn FnMut(web_sys::Event)>);
                root.add_event_listener_with_callback("rs:canvas:tool", cb.as_ref().unchecked_ref()).ok();
                cb.forget();
            }
            {
                let root2   = root.clone();
                let signal2 = signal.clone();
                let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    let state = root2.get_attribute("data-rs-canvas-state").unwrap_or_default();
                    signal2.update(|s| s.state = state);
                }) as Box<dyn FnMut(web_sys::Event)>);
                root.add_event_listener_with_callback("rs:canvas:changed", cb.as_ref().unchecked_ref()).ok();
                cb.forget();
            }
        }
    });
}
