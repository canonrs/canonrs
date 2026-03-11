use leptos::prelude::*;

#[derive(Clone, PartialEq)]
pub enum TextVariant { H1, H2, H3, P, Caption, Label }

#[component]
pub fn TextBlock(
    variant: TextVariant,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    let block_type = match &variant {
        TextVariant::H1      => "text-h1",
        TextVariant::H2      => "text-h2",
        TextVariant::H3      => "text-h3",
        TextVariant::P       => "text-p",
        TextVariant::Caption => "text-caption",
        TextVariant::Label   => "text-label",
    };
    view! {
        <div class=class data-block=block_type data-block-version="1">
            {children()}
        </div>
    }
}
