#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::document;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::Element;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = navigator, js_name = clipboard)]
    #[wasm_bindgen(thread_local_v2)]
    static CLIPBOARD: Clipboard;

    type Clipboard;

    #[wasm_bindgen(method, js_name = writeText)]
    fn write_text(this: &Clipboard, text: &str) -> js_sys::Promise;
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-copy-button", Box::new(|root: &web_sys::Element, _state: &ComponentState| {
        setup_copy_button(root)?;
        Ok(())
    }));
}

#[cfg(feature = "hydrate")]
fn fallback_copy(text: &str) {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    let Ok(el) = doc.create_element("textarea") else { return };
    el.set_text_content(Some(text));

    let style = el.dyn_ref::<web_sys::HtmlElement>().map(|e| e.style());
    if let Some(style) = style {
        let _ = style.set_property("position", "fixed");
        let _ = style.set_property("top", "-9999px");
        let _ = style.set_property("left", "-9999px");
        let _ = style.set_property("opacity", "0");
    }

    let Some(body) = doc.body() else { return };
    let _ = body.append_child(&el);

    if let Some(textarea) = el.dyn_ref::<web_sys::HtmlTextAreaElement>() {
        textarea.select();
        if let Some(html_doc) = doc.dyn_ref::<web_sys::HtmlDocument>() {
            let _ = html_doc.exec_command("copy");
        }
    }

    let _ = body.remove_child(&el);
}

#[cfg(feature = "hydrate")]
fn setup_copy_button(btn: &Element) -> BehaviorResult<()> {

    // hover
    {
        let el = btn.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let mut states = el.get_attribute("data-rs-state").unwrap_or_default();
            if !states.split_whitespace().any(|s| s == "hover") {
                states = format!("{} hover", states).trim().to_string();
            }
            el.set_attribute("data-rs-state", &states).ok();
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("mouseenter", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }
    {
        let el = btn.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            if let Some(states) = el.get_attribute("data-rs-state") {
                let filtered = states.split_whitespace().filter(|s| *s != "hover").collect::<Vec<_>>().join(" ");
                if filtered.is_empty() { el.remove_attribute("data-rs-state").ok(); }
                else { el.set_attribute("data-rs-state", &filtered).ok(); }
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("mouseleave", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    let btn_clone = btn.clone();

    let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
        let text = btn_clone.get_attribute("data-rs-copy-text")
            .filter(|t| !t.is_empty())
            .or_else(|| {
                btn_clone.get_attribute("data-rs-copy-target").and_then(|target| {
                    let selector = if target.starts_with('#') {
                        target.clone()
                    } else {
                        format!("#{}", target)
                    };
                    document().query_selector(&selector).ok()
                        .flatten()
                        .and_then(|el| el.text_content())
                })
            })
            .unwrap_or_default();

        if text.is_empty() {
            let _ = btn_clone.set_attribute("data-rs-state", "error");
            reset_after(&btn_clone, 2000);
            return;
        }

        let text_clone = text.clone();

        CLIPBOARD.with(|clip| {
            let _ = clip.write_text(&text_clone);
        });

        // Try clipboard API — if it fails silently, use fallback
        fallback_copy(&text_clone);

        let _ = btn_clone.set_attribute("data-rs-state", "copied");

        let delay = btn_clone.get_attribute("data-rs-reset-delay")
            .and_then(|d| d.parse::<i32>().ok())
            .unwrap_or(1300);
        reset_after(&btn_clone, delay);

    }) as Box<dyn FnMut(_)>);

    btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .map_err(|_| crate::BehaviorError::JsError { message: "click listener".into() })?;

    closure.forget();
    Ok(())
}

#[cfg(feature = "hydrate")]
fn reset_after(btn: &Element, delay: i32) {
    let btn_reset = btn.clone();
    let timeout = Closure::once(Box::new(move || {
        let _ = btn_reset.set_attribute("data-rs-state", "idle");
    }) as Box<dyn FnOnce()>);

    let _ = web_sys::window()
        .unwrap()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            timeout.as_ref().unchecked_ref(),
            delay,
        );
    timeout.forget();
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
