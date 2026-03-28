//! @canon-id: form
//! @canon-type: block
//! @canon-category: form
//! @canon-variant: feature
//! @canon-container: true
//! @canon-regions: fields, actions
//! @canon-label: Form
//! @canon-description: Form container with fields and actions
//! @canon-tags: form, fields, input, submit, data
//! @canon-prop: gap | Number | 1rem | visual | gap
//! @canon-prop: padding | Number | | visual | padding
//! @canon-slot-accepts: fields=Form,actions=Action
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
        <div
            data-rs-block=""
            data-rs-component="Form"
            data-rs-layout=layout.as_str()
            class=class
        >
            {fields.map(|f| view! { <div data-rs-region="fields">{f()}</div> })}
            {actions.map(|a| view! { <div data-rs-region="actions">{a()}</div> })}
        </div>
    }
}
