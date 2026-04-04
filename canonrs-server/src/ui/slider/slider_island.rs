use leptos::prelude::*;

#[island]
pub fn SliderIsland(
    #[prop(optional)] min: Option<f64>,
    #[prop(optional)] max: Option<f64>,
    #[prop(optional)] step: Option<f64>,
    #[prop(optional)] value: Option<f64>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let safe_min = min.unwrap_or(0.0);
    let safe_max = max.unwrap_or(100.0).max(safe_min + 0.001);
    let safe_val = value.unwrap_or(50.0).clamp(safe_min, safe_max);
    let safe_step = step.unwrap_or(0.0);
    let disabled = disabled.unwrap_or(false);
    let class = class.unwrap_or_default();

    let (current, set_current) = signal(safe_val);
    let (is_dragging, set_dragging) = signal(false);
    let (is_focused, set_focused) = signal(false);

    let percent = move || ((current.get() - safe_min) / (safe_max - safe_min)) * 100.0;

    #[cfg(feature = "hydrate")]
    let on_pointer_down = move |e: leptos::ev::PointerEvent| {
        if disabled { return; }
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;
        set_dragging.set(true);
        if let Some(target) = e.current_target() {
            let el = target.unchecked_into::<web_sys::HtmlElement>();
            el.set_pointer_capture(e.pointer_id()).ok();
            let el_el = el.unchecked_into::<web_sys::Element>();
            let mut s = el_el.get_attribute("data-rs-state").unwrap_or_default();
            if !s.contains("active") {
                s = format!("{} active", s).trim().to_string();
                el_el.set_attribute("data-rs-state", &s).ok();
            }
        }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_pointer_down = move |_: leptos::ev::PointerEvent| { let _ = (set_dragging, is_dragging); };

    #[cfg(feature = "hydrate")]
    let on_pointer_move = move |e: leptos::ev::PointerEvent| {
        if !is_dragging.get() || disabled { return; }
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;
        if let Some(target) = e.current_target() {
            let el = target.unchecked_into::<web_sys::HtmlElement>();
            let rect = el.get_bounding_client_rect();
            let x = (e.client_x() as f64 - rect.left()).max(0.0).min(rect.width());
            let pct = x / rect.width();
            let raw = safe_min + pct * (safe_max - safe_min);
            let snapped = if safe_step > 0.0 {
                ((raw / safe_step).round() * safe_step).clamp(safe_min, safe_max)
            } else {
                raw.clamp(safe_min, safe_max)
            };
            set_current.set(snapped);
        }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_pointer_move = move |_: leptos::ev::PointerEvent| { let _ = set_current; };

    #[cfg(feature = "hydrate")]
    let on_pointer_up = move |_: leptos::ev::PointerEvent| {
        set_dragging.set(false);
    };
    #[cfg(not(feature = "hydrate"))]
    let on_pointer_up = move |_: leptos::ev::PointerEvent| { let _ = set_dragging; };

    #[cfg(feature = "hydrate")]
    let on_focus = move |_: leptos::ev::FocusEvent| { set_focused.set(true); };
    #[cfg(not(feature = "hydrate"))]
    let on_focus = move |_: leptos::ev::FocusEvent| { let _ = set_focused; };

    #[cfg(feature = "hydrate")]
    let on_blur = move |_: leptos::ev::FocusEvent| { set_focused.set(false); };
    #[cfg(not(feature = "hydrate"))]
    let on_blur = move |_: leptos::ev::FocusEvent| { let _ = set_focused; };

    #[cfg(feature = "hydrate")]
    let on_key = move |e: leptos::ev::KeyboardEvent| {
        if disabled { return; }
        let val = current.get_untracked();
        let snap = |raw: f64| if safe_step > 0.0 {
            ((raw / safe_step).round() * safe_step).clamp(safe_min, safe_max)
        } else { raw.clamp(safe_min, safe_max) };
        let new_val = match e.key().as_str() {
            "ArrowRight" | "ArrowUp"   => snap(val + if safe_step > 0.0 { safe_step } else { 1.0 }),
            "ArrowLeft"  | "ArrowDown" => snap(val - if safe_step > 0.0 { safe_step } else { 1.0 }),
            "Home" => safe_min,
            "End"  => safe_max,
            _ => return,
        };
        e.prevent_default();
        set_current.set(new_val);
    };
    #[cfg(not(feature = "hydrate"))]
    let on_key = move |_: leptos::ev::KeyboardEvent| { let _ = current; };

    let initial_state = {
        let mut s = vec![];
        if disabled { s.push("disabled"); }
        s.join(" ")
    };

    view! {
        <div
            data-rs-slider=""
            data-rs-component="Slider"
            data-rs-orientation="horizontal"
            data-rs-value=move || format!("{:.2}", current.get())
            data-rs-percent=move || format!("{:.4}", percent())
            data-rs-step=safe_step.to_string()
            data-rs-state=move || {
                let mut s = vec![];
                if is_dragging.get() { s.push("active"); }
                if is_focused.get() { s.push("focus"); }
                if disabled { s.push("disabled"); }
                let s = s.join(" ");
                if s.is_empty() { initial_state.clone() } else { s }
            }
            role="slider"
            aria-valuemin=safe_min.to_string()
            aria-valuemax=safe_max.to_string()
            aria-valuenow=move || format!("{:.2}", current.get())
            aria-orientation="horizontal"
            aria-disabled=disabled.to_string()
            tabindex=if disabled { "-1" } else { "0" }
            style=move || format!("--slider-fill: {:.4}%", percent())
            class=class
            on:pointerdown=on_pointer_down
            on:pointermove=on_pointer_move
            on:pointerup=on_pointer_up
            on:focus=on_focus
            on:blur=on_blur
            on:keydown=on_key
        >
            <div data-rs-slider-track="" data-rs-component="SliderTrack">
                <div data-rs-slider-range="" data-rs-component="SliderRange" />
                <div data-rs-slider-thumb="" data-rs-component="SliderThumb"
                    tabindex="0" aria-label="Slider thumb" />
            </div>
        </div>
    }
}

#[component]
pub fn Slider(
    #[prop(optional)] min: Option<f64>,
    #[prop(optional)] max: Option<f64>,
    #[prop(optional)] step: Option<f64>,
    #[prop(optional)] value: Option<f64>,
    #[prop(optional)] with_marks: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let safe_min = min.unwrap_or(0.0);
    let safe_max = max.unwrap_or(100.0).max(safe_min + 0.001);
    let safe_step = step.unwrap_or(1.0).max(0.001);
    let with_marks = with_marks.unwrap_or(false);

    let marks: Vec<String> = if with_marks {
        let count = ((safe_max - safe_min) / safe_step).round() as usize + 1;
        (0..count)
            .map(|i| safe_min + i as f64 * safe_step)
            .filter(|v| *v <= safe_max + 1e-9)
            .map(|v| format!("left:{:.4}%", ((v - safe_min) / (safe_max - safe_min)) * 100.0))
            .collect()
    } else { vec![] };

    view! {
        <div data-rs-slider-shell="">
            <SliderIsland
                min=safe_min
                max=safe_max
                step=step.unwrap_or(0.0)
                value=value.unwrap_or(50.0)
                disabled=disabled.unwrap_or(false)
                class=class.unwrap_or_default()
            />
            {if !marks.is_empty() {
                Some(view! {
                    <div data-rs-slider-marks="">
                        {marks.into_iter().map(|s| view! {
                            <span data-rs-slider-mark="" style=s />
                        }).collect::<Vec<_>>()}
                    </div>
                })
            } else { None }}
        </div>
    }
}
