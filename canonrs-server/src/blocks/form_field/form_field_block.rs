use leptos::prelude::*;
use canonrs_core::infra::uid::generate;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn FormFieldBlock(
    #[prop(optional)] label: Option<ChildrenFn>,
    #[prop(optional)] input: Option<ChildrenFn>,
    #[prop(optional)] hint: Option<ChildrenFn>,
    #[prop(optional)] error: Option<ChildrenFn>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid   = generate("bl");
    let label = StoredValue::new(label);
    let input = StoredValue::new(input);
    let hint  = StoredValue::new(hint);
    let error = StoredValue::new(error);
    view! {
        <div data-rs-form-field="" data-rs-uid=uid role="group" class=class>
            <Stack direction=StackDirection::Vertical gap=StackGap::Xs>
                {move || label.get_value().map(|l| view! { <div data-rs-region="label">{l()}</div> })}
                {move || input.get_value().map(|i| view! { <div data-rs-region="input">{i()}</div> })}
                {move || hint.get_value().map(|h| view! { <div data-rs-region="hint">{h()}</div> })}
                {move || error.get_value().map(|e| view! { <div data-rs-region="error" role="alert">{e()}</div> })}
            </Stack>
        </div>
    }
}
