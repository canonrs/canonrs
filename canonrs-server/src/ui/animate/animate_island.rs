//! @canon-level: strict
//! Animate Island — prefers-reduced-motion, scroll-driven activation, stagger

use leptos::prelude::*;
use super::animate_ui::{Animate, AnimationName, AnimationEasing};

static ANIMATE_ID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);

#[island]
pub fn AnimateIsland(
    children: Children,
    #[prop(optional, into)] animation: Option<String>,
    #[prop(optional, into)] easing: Option<String>,
    #[prop(optional, into)] duration: Option<String>,
    #[prop(optional, into)] delay: Option<String>,
    #[prop(optional)] stagger: Option<f64>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let animation_val = match animation.as_deref() {
        Some("fade-out")  => AnimationName::FadeOut,
        Some("slide-in")  => AnimationName::SlideIn,
        Some("slide-out") => AnimationName::SlideOut,
        Some("scale-in")  => AnimationName::ScaleIn,
        Some("scale-out") => AnimationName::ScaleOut,
        _                 => AnimationName::FadeIn,
    };
    let easing_val = match easing.as_deref() {
        Some("ease-in")   => AnimationEasing::EaseIn,
        Some("ease-out")  => AnimationEasing::EaseOut,
        Some("linear")    => AnimationEasing::Linear,
        _                 => AnimationEasing::EaseInOut,
    };
    let duration_val = duration.unwrap_or_else(|| "300ms".to_string());
    let delay_val    = delay.unwrap_or_default();
    let stagger_ms   = stagger.unwrap_or(0.0);
    let class_val    = class.unwrap_or_default();
    let node_ref     = NodeRef::<leptos::html::Div>::new();
    let _id          = ANIMATE_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;
        use leptos::web_sys::js_sys;

        let Some(root_html) = node_ref.get() else { return };
        let wrapper: web_sys::Element = (*root_html).clone().unchecked_into();
        let win = match web_sys::window() { Some(w) => w, None => return };

        let raf_cb = Closure::wrap(Box::new(move || {
            if wrapper.has_attribute("data-rs-animate-js") { return; }
            wrapper.set_attribute("data-rs-animate-js", "1").ok();

            let target = match wrapper.query_selector("[data-rs-animate]").ok().flatten() {
                Some(t) => t,
                None => return,
            };

            // prefers-reduced-motion
            let reduced = js_sys::Function::new_no_args(
                "return window.matchMedia('(prefers-reduced-motion: reduce)').matches;"
            ).call0(&JsValue::NULL).map(|v| v.is_truthy()).unwrap_or(false);

            if reduced {
                target.set_attribute("data-rs-animate-reduced", "1").ok();
                target.set_attribute("data-rs-state", "active").ok();
                return;
            }

            // stagger
            if stagger_ms > 0.0 {
                let ch = target.children();
                for i in 0..ch.length() {
                    if let Some(c) = ch.item(i) {
                        c.set_attribute("data-rs-stagger-index", &i.to_string()).ok();
                    }
                }
            }

            // scroll-driven activation — deterministic, no GC issues
            let script = "
                

                if (!window.__rsAnimateRegistry) {
                    window.__rsAnimateRegistry = [];

                    window.addEventListener('scroll', function() {
                        window.__rsAnimateRegistry = window.__rsAnimateRegistry.filter(function(item) {
                            if (item.done) return false;
                            var rect = item.el.getBoundingClientRect();
                            if (rect.top < window.innerHeight * 0.85) {
                                item.el.removeAttribute('data-rs-state');
                                void item.el.offsetWidth;
                                item.el.setAttribute('data-rs-state', 'active');
                                return false;
                            }
                            return true;
                        });
                    }, { passive: true });
                }

                el.setAttribute('data-rs-state', 'inactive');

                // estado inicial obrigatório
                el.setAttribute('data-rs-state', 'inactive');

                window.__rsAnimateRegistry.push({ el: el, done: false });

                // primeira verificação imediata
                var rect = el.getBoundingClientRect();
                if (rect.top < window.innerHeight * 0.85) {
                    el.removeAttribute('data-rs-state');
                    void el.offsetWidth;
                    el.setAttribute('data-rs-state', 'active');
                }
            ";
            let f = js_sys::Function::new_with_args("el", script);
            f.call1(&JsValue::NULL, &target).ok();

        }) as Box<dyn Fn()>);

        win.request_animation_frame(raf_cb.as_ref().unchecked_ref()).ok();
        raf_cb.forget();
    });

    view! {
        <div node_ref=node_ref data-rs-animate-root="">
            <Animate
                animation=animation_val
                easing=easing_val
                duration=duration_val
                delay=delay_val
                class=class_val
            >
                {children()}
            </Animate>
        </div>
    }
}
