//! Toast Interactive - Estado + Auto-dismiss

use leptos::prelude::*;
use canonrs_core::{
    ToastPrimitive,
    ToastTitlePrimitive,
    ToastDescriptionPrimitive,
    ToastClosePrimitive,
    ToastVariant,
};
use canonrs_core::meta::VisibilityState;

#[component]
pub fn InteractiveToast(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(default = ToastVariant::Default)] variant: ToastVariant,
    #[prop(optional)] #[allow(unused_variables)] auto_dismiss_ms: Option<u32>,
) -> impl IntoView {
    let (open, set_open) = signal(true);

    let title = StoredValue::new(title);
    let description = StoredValue::new(description);

    #[cfg(target_arch = "wasm32")]
    {
        if let Some(duration) = auto_dismiss_ms {
            use gloo_timers::future::TimeoutFuture;
            use wasm_bindgen_futures::spawn_local;

            spawn_local(async move {
                TimeoutFuture::new(duration).await;
                set_open.set(false);
            });
        }
    }

    view! {
        {move || open.get().then(|| view! {
            <ToastPrimitive variant=variant state=VisibilityState::Open>
                <ToastTitlePrimitive>{title.get_value()}</ToastTitlePrimitive>
                <ToastDescriptionPrimitive>{description.get_value()}</ToastDescriptionPrimitive>
                <ToastClosePrimitive on:click=move |_| set_open.set(false)>
                    "×"
                </ToastClosePrimitive>
            </ToastPrimitive>
        })}
    }
}
