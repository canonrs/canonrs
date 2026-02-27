//! # Table Block
use leptos::prelude::*;

#[component]
pub fn Table(
    #[prop(optional)] header: Option<Children>,
    #[prop(default = false)] striped: bool,
    #[prop(default = false)] bordered: bool,
    #[prop(default = false)] hoverable: bool,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="canon-table-wrapper" data-block="table" data-block-version="1">
            <div data-block-region="header">
                {header.map(|h| view! { <thead class="canon-table__head">{h()}</thead> })}
            </div>
            <table
                class=move || format!("canon-table {} {} {} {}", if striped { "canon-table--striped" } else { "" }, if bordered { "canon-table--bordered" } else { "" }, if hoverable { "canon-table--hoverable" } else { "" }, class)
            >
                <tbody data-block-region="body" class="canon-table__body">
                    {children()}
                </tbody>
            </table>
        </div>
    }
}
