use leptos::prelude::*;

#[component]
pub fn DropIndicator(
    visible: Signal<bool>,
    #[prop(default = "top".to_string())] position: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-drop-indicator=""
            attr:data-visible={visible.get()}
            attr:data-position={position}
        >
        </div>
    }
}
