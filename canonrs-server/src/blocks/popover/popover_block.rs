//! # Popover Block — Categoria C (Overlay)
//! Regra: overlay container NÃO é drop zone. Só content interno é region.
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum PopoverPlacement { Top, Bottom, Left, Right }

impl PopoverPlacement {
    fn as_str(&self) -> &'static str {
        match self { PopoverPlacement::Top => "top", PopoverPlacement::Bottom => "bottom", PopoverPlacement::Left => "left", PopoverPlacement::Right => "right" }
    }
}

#[component]
pub fn PopoverBlock(
    #[prop(into)] open: MaybeSignal<bool>,
    #[prop(optional)] on_close: Option<Callback<()>>,
    #[prop(default = PopoverPlacement::Bottom)] placement: PopoverPlacement,
    #[prop(optional)] title: Option<ChildrenFn>,
    #[prop(optional)] close_button: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=move || format!("canon-popover canon-popover--{} {} {}", placement.as_str(), if open.get() { "canon-popover--open" } else { "" }, class)
            data-block="popover"
            data-block-version="1"
        >
            <div data-block-region="header">
                {title.map(|t| view! {
                    <div class="canon-popover__header">
                        <div class="canon-popover__title">{t()}</div>
                        {close_button.map(|btn| view! { <div class="canon-popover__close">{btn()}</div> })}
                    </div>
                })}
            </div>
            <div data-block-region="content" class="canon-popover__content">
                {children()}
            </div>
        </div>
    }
}
