#[cfg(feature = "hydrate")]
use super::register_behavior;
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::document;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::Element;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-carousel", Box::new(|element_id, _state| {
        let Some(carousel) = document().get_element_by_id(element_id) else {
            return Ok(());
        };
        setup_carousel(&carousel)?;
        Ok(())
    }));
}

#[cfg(feature = "hydrate")]
fn setup_carousel(carousel: &Element) -> BehaviorResult<()> {
    if carousel.get_attribute("data-carousel-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = carousel.set_attribute("data-carousel-attached", "1");

    let Some(wrapper) = carousel.query_selector("[data-carousel-wrapper]")
        .ok().flatten() else {
        return Ok(());
    };

    let wrapper_el: web_sys::HtmlElement = wrapper.dyn_into()
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "cast wrapper".into() })?;

    let initial_index = wrapper_el.get_attribute("data-initial-index")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);
    
    let autoplay = wrapper_el.has_attribute("data-autoplay");
    let loop_mode = wrapper_el.has_attribute("data-loop");
    let interval = wrapper_el.get_attribute("data-interval")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(5000);

    let items = carousel.query_selector_all("[data-carousel-item]")
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "query items".into() })?;
    
    let total_items = items.length() as usize;
    if total_items == 0 {
        return Ok(());
    }

    // Generate indicators
    if let Some(indicators_container) = carousel.query_selector("[data-carousel-indicators]").ok().flatten() {
        for i in 0..total_items {
            let dot = document().create_element("button")
                .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "create dot".into() })?;
            let _ = dot.set_attribute("data-carousel-dot", "");
            let _ = dot.set_attribute("data-index", &i.to_string());
            let _ = dot.set_attribute("aria-label", &format!("Go to slide {}", i + 1));
            
            let carousel_clone = carousel.clone();
            let on_click = Closure::wrap(Box::new(move |_: web_sys::Event| {
                go_to_slide(&carousel_clone, i);
            }) as Box<dyn FnMut(_)>);
            
            dot.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref()).ok();
            on_click.forget();
            
            let _ = indicators_container.append_child(&dot);
        }
    }

    go_to_slide(carousel, initial_index);

    // Setup prev/next
    if let Some(prev) = carousel.query_selector("[data-carousel-prev]").ok().flatten() {
        let carousel_clone = carousel.clone();
        let total = total_items;
        let on_click = Closure::wrap(Box::new(move |_: web_sys::Event| {
            prev_slide(&carousel_clone, total, loop_mode);
        }) as Box<dyn FnMut(_)>);
        prev.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref()).ok();
        on_click.forget();
    }

    if let Some(next) = carousel.query_selector("[data-carousel-next]").ok().flatten() {
        let carousel_clone = carousel.clone();
        let total = total_items;
        let on_click = Closure::wrap(Box::new(move |_: web_sys::Event| {
            next_slide(&carousel_clone, total, loop_mode);
        }) as Box<dyn FnMut(_)>);
        next.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref()).ok();
        on_click.forget();
    }

    // Keyboard
    let carousel_clone = carousel.clone();
    let total = total_items;
    let on_keydown = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        match e.key().as_str() {
            "ArrowLeft" => {
                e.prevent_default();
                prev_slide(&carousel_clone, total, loop_mode);
            },
            "ArrowRight" => {
                e.prevent_default();
                next_slide(&carousel_clone, total, loop_mode);
            },
            _ => {}
        }
    }) as Box<dyn FnMut(_)>);
    
    carousel.add_event_listener_with_callback("keydown", on_keydown.as_ref().unchecked_ref())
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "keydown".into() })?;
    on_keydown.forget();

    // Autoplay
    if autoplay {
        let carousel_clone = carousel.clone();
        let total = total_items;
        
        let autoplay_fn = Closure::wrap(Box::new(move || {
            next_slide(&carousel_clone, total, true);
        }) as Box<dyn FnMut()>);
        
        web_sys::window()
            .unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                autoplay_fn.as_ref().unchecked_ref(),
                interval,
            ).ok();
        
        autoplay_fn.forget();
    }

    Ok(())
}

#[cfg(feature = "hydrate")]
fn get_current_index(carousel: &Element) -> usize {
    carousel.get_attribute("data-current-index")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0)
}

#[cfg(feature = "hydrate")]
fn go_to_slide(carousel: &Element, index: usize) {
    let _ = carousel.set_attribute("data-current-index", &index.to_string());

    // Update items - cast Node to Element
    if let Ok(items) = carousel.query_selector_all("[data-carousel-item]") {
        for i in 0..items.length() {
            if let Some(node) = items.item(i) {
                if let Ok(item) = node.dyn_into::<Element>() {
                    if i == index as u32 {
                        item.set_attribute("data-active", "").ok();
                        item.set_attribute("aria-hidden", "false").ok();
                    } else {
                        item.remove_attribute("data-active").ok();
                        item.set_attribute("aria-hidden", "true").ok();
                    }
                }
            }
        }
    }

    // Update indicators - cast Node to Element
    if let Ok(dots) = carousel.query_selector_all("[data-carousel-dot]") {
        for i in 0..dots.length() {
            if let Some(node) = dots.item(i) {
                if let Ok(dot) = node.dyn_into::<Element>() {
                    if i == index as u32 {
                        dot.set_attribute("data-active", "").ok();
                        dot.set_attribute("aria-current", "true").ok();
                    } else {
                        dot.remove_attribute("data-active").ok();
                        dot.remove_attribute("aria-current").ok();
                    }
                }
            }
        }
    }

    dispatch_slide_event(carousel, index);
}

#[cfg(feature = "hydrate")]
fn prev_slide(carousel: &Element, total: usize, loop_mode: bool) {
    let current = get_current_index(carousel);
    let new_index = if current == 0 {
        if loop_mode { total - 1 } else { 0 }
    } else {
        current - 1
    };
    go_to_slide(carousel, new_index);
}

#[cfg(feature = "hydrate")]
fn next_slide(carousel: &Element, total: usize, loop_mode: bool) {
    let current = get_current_index(carousel);
    let new_index = if current >= total - 1 {
        if loop_mode { 0 } else { total - 1 }
    } else {
        current + 1
    };
    go_to_slide(carousel, new_index);
}

#[cfg(feature = "hydrate")]
fn dispatch_slide_event(carousel: &Element, index: usize) {
    let detail = js_sys::Object::new();
    js_sys::Reflect::set(&detail, &JsValue::from_str("index"), &JsValue::from_f64(index as f64)).ok();

    let f = js_sys::Function::new_with_args(
        "el,detail",
        "el.dispatchEvent(new CustomEvent('canon:carousel-change', {bubbles:true, detail:detail}));"
    );
    f.call2(&JsValue::NULL, carousel, &detail).ok();
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
