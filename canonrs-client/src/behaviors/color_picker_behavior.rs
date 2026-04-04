#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;

#[cfg(feature = "hydrate")]
fn add_state(el: &web_sys::Element, state: &str) {
    let mut s = el.get_attribute("data-rs-state").unwrap_or_default();
    if !s.split_whitespace().any(|x| x == state) {
        s = format!("{} {}", s, state).trim().to_string();
    }
    el.set_attribute("data-rs-state", &s).ok();
}

#[cfg(feature = "hydrate")]
fn remove_state(el: &web_sys::Element, state: &str) {
    if let Some(s) = el.get_attribute("data-rs-state") {
        let f = s.split_whitespace().filter(|x| *x != state).collect::<Vec<_>>().join(" ");
        if f.is_empty() { el.remove_attribute("data-rs-state").ok(); }
        else { el.set_attribute("data-rs-state", &f).ok(); }
    }
}

#[cfg(feature = "hydrate")]
fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let h = hex.trim_start_matches("#");
    if h.len() != 6 { return None; }
    let r = u8::from_str_radix(&h[0..2], 16).ok()?;
    let g = u8::from_str_radix(&h[2..4], 16).ok()?;
    let b = u8::from_str_radix(&h[4..6], 16).ok()?;
    Some((r, g, b))
}

#[cfg(feature = "hydrate")]
fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let r = r as f64 / 255.0; let g = g as f64 / 255.0; let b = b as f64 / 255.0;
    let max = r.max(g).max(b); let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    if (max - min).abs() < 1e-10 { return (0.0, 0.0, l * 100.0); }
    let d = max - min;
    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
    let h = if max == r { ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0 }
            else if max == g { ((b - r) / d + 2.0) / 6.0 }
            else { ((r - g) / d + 4.0) / 6.0 };
    (h * 360.0, s * 100.0, l * 100.0)
}

#[cfg(feature = "hydrate")]
fn rgb_to_cmyk(r: u8, g: u8, b: u8) -> (f64, f64, f64, f64) {
    let r = r as f64 / 255.0; let g = g as f64 / 255.0; let b = b as f64 / 255.0;
    let k = 1.0 - r.max(g).max(b);
    if (k - 1.0).abs() < 1e-10 { return (0.0, 0.0, 0.0, 100.0); }
    let c = (1.0 - r - k) / (1.0 - k);
    let m = (1.0 - g - k) / (1.0 - k);
    let y = (1.0 - b - k) / (1.0 - k);
    (c * 100.0, m * 100.0, y * 100.0, k * 100.0)
}

#[cfg(feature = "hydrate")]
fn format_color(hex: &str, format: &str) -> String {
    let (r, g, b) = match hex_to_rgb(hex) { Some(v) => v, None => return hex.to_string() };
    match format {
        "hex"  => hex.to_string(),
        "rgb"  => format!("rgb({}, {}, {})", r, g, b),
        "hsl"  => { let (h,s,l) = rgb_to_hsl(r,g,b); format!("hsl({:.0}, {:.0}%, {:.0}%)", h, s, l) }
        "cmyk" => { let (c,m,y,k) = rgb_to_cmyk(r,g,b); format!("cmyk({:.0}%, {:.0}%, {:.0}%, {:.0}%)", c, m, y, k) }
        _ => hex.to_string(),
    }
}

#[cfg(feature = "hydrate")]
fn apply_bg(el: &web_sys::Element, color: &str) {
    if let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() {
        html_el.style().set_property("background-color", color).ok();
    }
}

#[cfg(feature = "hydrate")]
fn sync_displays(scope: &web_sys::Element, color: &str) {
    if let Ok(displays) = scope.query_selector_all("[data-rs-color-picker-display]") {
        for i in 0..displays.length() {
            if let Some(node) = displays.item(i) {
                if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                    let fmt = el.get_attribute("data-rs-format").unwrap_or_else(|| "hex".to_string());
                    if let Ok(Some(val)) = el.query_selector("[data-rs-color-display-value]") {
                        val.set_text_content(Some(&format_color(color, &fmt)));
                    }
                }
            }
        }
    }
}

#[cfg(feature = "hydrate")]
fn sync_all(root: &web_sys::Element, color: &str) {
    root.set_attribute("data-rs-value", color).ok();
    // sync input nativo — atualiza propriedade JS e atributo HTML
    if let Ok(Some(input)) = root.query_selector("[data-rs-color-picker-input]") {
        if let Ok(el) = input.clone().dyn_into::<web_sys::HtmlInputElement>() {
            el.set_value(color);
            el.set_attribute("value", color).ok();
        }
    }
    // sync trigger swatch — sempre atualiza bg e data-rs-color
    if let Ok(Some(trigger)) = root.query_selector("[data-rs-color-picker-trigger]") {
        if let Ok(Some(swatch)) = trigger.query_selector("[data-rs-color-swatch]") {
            swatch.set_attribute("data-rs-color", color).ok();
            apply_bg(&swatch, color);
        }
    }
    // sync displays dentro do root
    sync_displays(root, color);
    // sync displays fora do root (showcase)
    if let Ok(Some(parent)) = root.closest("[data-rs-showcase-preview-hero], body") {
        sync_displays(&parent, color);
    }
}


#[cfg(feature = "hydrate")]
fn sync_selected_swatch(root: &web_sys::Element, color: &str) {
    if let Ok(swatches) = root.query_selector_all("[data-rs-color-swatch][data-rs-color]") {
        for i in 0..swatches.length() {
            if let Some(node) = swatches.item(i) {
                if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                    let swatch_color = el.get_attribute("data-rs-color").unwrap_or_default();
                    if swatch_color.to_lowercase() == color.to_lowercase() {
                        add_state(&el, "selected");
                        el.set_attribute("aria-selected", "true").ok();
                    } else {
                        remove_state(&el, "selected");
                        el.set_attribute("aria-selected", "false").ok();
                    }
                }
            }
        }
    }
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-color-picker", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {
        // INIT IMEDIATO — sem timeout, sem observer
        let init_color = root.get_attribute("data-rs-value")
            .or_else(|| root.query_selector("[data-rs-color-picker-input]").ok().flatten()
                .and_then(|el| el.get_attribute("value")))
            .unwrap_or_else(|| "#000000".to_string());
        sync_all(root, &init_color);
        sync_selected_swatch(root, &init_color);

        if root.has_attribute("data-rs-disabled") {
            add_state(root, "disabled");
            if let Ok(Some(input)) = root.query_selector("[data-rs-color-picker-input]") {
                input.set_attribute("disabled", "").ok();
            }
            return Ok(());
        }
        // INIT — aplica cor inicial com timeout
        { let r = root.clone();
          let closure = Closure::once(Box::new(move || {
              let color = r.get_attribute("data-rs-value")
                  .filter(|s| !s.is_empty())
                  .or_else(|| r.query_selector("[data-rs-color-picker-input]").ok().flatten()
                      .and_then(|el| el.get_attribute("value")))
                  .unwrap_or_else(|| "#000000".to_string());
              sync_all(&r, &color);
          }) as Box<dyn FnOnce()>);
          web_sys::window().unwrap()
              .set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 0).ok();
          closure.forget(); }
        // input change
        if let Ok(Some(input_el)) = root.query_selector("[data-rs-color-picker-input]") {
            let r = root.clone();
            let cb = Closure::wrap(Box::new(move |e: web_sys::Event| {
                let color = e.target()
                    .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                    .map(|el| el.value())
                    .unwrap_or_default();
                sync_all(&r, &color);
                if let Ok(event) = web_sys::CustomEvent::new("rs-change") {
                    r.dispatch_event(&event).ok();
                }
            }) as Box<dyn FnMut(_)>);
            input_el.add_event_listener_with_callback("input", cb.as_ref().unchecked_ref()).ok();
            input_el.add_event_listener_with_callback("change", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }
        Ok(())
    }));

    register_behavior("data-rs-color-swatch", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {
        // aplicar bg do data-rs-color
        if let Some(color) = root.get_attribute("data-rs-color") {
            apply_bg(root, &color);
        }
        if root.has_attribute("data-rs-disabled") {
            add_state(root, "disabled");
            return Ok(());
        }
        { let swatch = root.clone();
          let cb = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
              let color = swatch.get_attribute("data-rs-color").unwrap_or_default();
              // deseleciona todos no grupo
              if let Ok(Some(group)) = swatch.closest("[data-rs-showcase-preview-row], [data-rs-color-swatch-group]") {
                  if let Ok(swatches) = group.query_selector_all("[data-rs-color-swatch]") {
                      for i in 0..swatches.length() {
                          if let Some(node) = swatches.item(i) {
                              if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                                  remove_state(&el, "selected");
                                  el.set_attribute("aria-selected", "false").ok();
                              }
                          }
                      }
                  }
              }
              add_state(&swatch, "selected");
              swatch.set_attribute("aria-selected", "true").ok();
              // tenta picker ancestral direto
              let picker_opt = swatch.closest("[data-rs-color-picker]").ok().flatten()
                  // senão busca no escopo do showcase
                  .or_else(|| {
                      swatch.closest("[data-rs-showcase-preview-hero], body").ok().flatten()
                          .and_then(|scope| scope.query_selector("[data-rs-color-picker]").ok().flatten())
                  });
              if let Some(picker) = picker_opt {
                  sync_all(&picker, &color);
                  sync_selected_swatch(&picker, &color);
              }
              if let Ok(event) = web_sys::CustomEvent::new("rs-change") {
                  swatch.dispatch_event(&event).ok();
              }
          }) as Box<dyn FnMut(_)>);
          root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
          cb.forget(); }
        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
