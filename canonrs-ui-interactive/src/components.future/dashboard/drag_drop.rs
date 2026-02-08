use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct DragItemId(pub String);
impl DragItemId {
    pub fn new(s: String) -> Self { Self(s) }
}

#[derive(Clone, Debug)]
pub struct DropTargetId(pub String);
impl DropTargetId {
    pub fn new(s: &str) -> Self { Self(s.to_string()) }
}

#[component]
pub fn DragHandle(
    #[prop(optional)] item_id: Option<DragItemId>,
    children: Children,
) -> impl IntoView {
    view! { <div class="drag-handle">{children()}</div> }
}

#[component]
pub fn DropZone(
    #[prop(optional)] target_id: Option<DropTargetId>,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    view! { <div class={format!("drop-zone {}", class)}>{children()}</div> }
}
