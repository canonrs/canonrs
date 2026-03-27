//! @canon-id: form
//! @canon-type: block
//! @canon-category: form
//! @canon-variant: feature
//! @canon-container: true
//! @canon-regions: fields, actions
//! @canon-prop: gap | Number | 1rem | visual | gap
//! @canon-prop: padding | Number | | visual | padding
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum FormLayout { #[default] Vertical, Horizontal, Inline }
impl FormLayout {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Vertical => "vertical", Self::Horizontal => "horizontal", Self::Inline => "inline" }
    }
}

#[component]
pub fn FormBlock(
    #[prop(default = FormLayout::Vertical)] layout: FormLayout,
    #[prop(optional)] fields: Option<ChildrenFn>,
    #[prop(optional)] actions: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] _style: String,
) -> impl IntoView {
    view! {
        <div data-block="form" data-block-version="1" data-block-layout=layout.as_str() class=class>
            {fields.map(|f| view! { <div data-block-region="fields">{f()}</div> })}
            {actions.map(|a| view! { <div data-block-region="actions">{a()}</div> })}
        </div>
    }
}
