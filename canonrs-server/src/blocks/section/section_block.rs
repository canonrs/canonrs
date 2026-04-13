use leptos::prelude::*;
use canonrs_core::infra::uid::generate;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn SectionBlock(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] body: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid = generate("bl");
    let header = StoredValue::new(header);
    let body = StoredValue::new(body);
    let footer = StoredValue::new(footer);
    view! {
        <section data-rs-section="" data-rs-uid=uid class=class>
            <Stack direction=StackDirection::Vertical gap=StackGap::Md>
                {move || header.get_value().map(|h| view! { <div data-rs-region="header">{h()}</div> })}
                {move || body.get_value().map(|b| view! { <div data-rs-region="body">{b()}</div> })}
                {move || footer.get_value().map(|f| view! { <div data-rs-region="footer">{f()}</div> })}
            </Stack>
        </section>
    }
}
