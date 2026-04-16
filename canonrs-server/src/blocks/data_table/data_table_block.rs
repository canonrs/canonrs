use leptos::prelude::*;
use canonrs_core::infra::uid::generate;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};
use crate::ui::scroll_area::scroll_area_boundary::ScrollArea;

#[component]
pub fn DataTableBlock(
    #[prop(optional)] toolbar: Option<ChildrenFn>,
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] body: Option<ChildrenFn>,
    #[prop(optional)] empty: Option<ChildrenFn>,
    #[prop(optional)] pagination: Option<ChildrenFn>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid = generate("bl");
    let toolbar = StoredValue::new(toolbar);
    let header = StoredValue::new(header);
    let body = StoredValue::new(body);
    let empty = StoredValue::new(empty);
    let pagination = StoredValue::new(pagination);
    view! {
        <div data-rs-data-table="" data-rs-uid=uid class=class>
            <Stack direction=StackDirection::Vertical gap=StackGap::Md>
                {move || toolbar.get_value().map(|t| view! { <div data-rs-region="toolbar">{t()}</div> })}
                <div data-rs-region="table-wrap">
                    <ScrollArea>
                        {move || {
                            if body.get_value().is_some() {
                                body.get_value().map(|b| view! { <div data-rs-region="body">{b()}</div> })
                            } else {
                                empty.get_value().map(|e| view! { <div data-rs-region="empty">{e()}</div> })
                            }
                        }}
                    </ScrollArea>
                </div>
                {move || header.get_value().map(|h| view! { <div data-rs-region="header">{h()}</div> })}
                {move || pagination.get_value().map(|p| view! { <div data-rs-region="pagination">{p()}</div> })}
            </Stack>
        </div>
    }
}
