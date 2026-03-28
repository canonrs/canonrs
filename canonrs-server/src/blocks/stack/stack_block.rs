//! @canon-id: stack
//! @canon-type: block
//! @canon-category: layout
//! @canon-variant: structure
//! @canon-container: true
//! @canon-regions: items
//! @canon-label: Stack
//! @canon-description: Flex stack container vertical or horizontal
//! @canon-tags: stack, flex, column, row, vertical, horizontal
//! @canon-prop: flex-direction | Select(column:Vertical,row:Horizontal) | column | visual | flex-direction
//! @canon-prop: gap | Number | 0.5rem | visual | gap
//! @canon-prop: align-items | Select(stretch:Stretch,flex-start:Start,center:Center,flex-end:End) | stretch | visual | align-items
//! @canon-slot-accepts: items=Any
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum StackDirection { #[default] Vertical, Horizontal }
impl StackDirection {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Vertical => "vertical", Self::Horizontal => "horizontal" }
    }
}

#[component]
pub fn Stack(
    #[prop(default = StackDirection::Vertical)] direction: StackDirection,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] style: String,
    #[prop(optional)] items: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <div
            data-rs-block=""
            data-rs-component="Stack"
            data-rs-direction=direction.as_str()
            class=class
            style=style
        >
            {items.map(|i| view! { <div data-rs-region="items">{i()}</div> })}
        </div>
    }
}
