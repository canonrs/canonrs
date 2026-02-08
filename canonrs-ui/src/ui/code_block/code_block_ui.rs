//! @canon-level: strict
//! @canon-owner: ui-team
//! CodeBlock UI Component

use leptos::prelude::*;
use crate::primitives::{
    CodeBlockPrimitive,
    CodeBlockHeaderPrimitive,
    CodeBlockLanguagePrimitive,
    CodeBlockContentPrimitive,
};
// use crate::ui::copy_button::CopyButton;

#[component]
pub fn CodeBlock(
    code: String,
    #[prop(into, optional)] language: Option<String>,
    #[prop(default = true)] show_copy: bool,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let lang = language.unwrap_or_else(|| "text".to_string());
    let code_clone = code.clone();

    view! {
        <CodeBlockPrimitive
            language={lang.clone()}
            class={class.unwrap_or_default()}
            id={id.unwrap_or_default()}
        >
            <CodeBlockHeaderPrimitive>
                <div data-code-header-inner="">
                    <CodeBlockLanguagePrimitive language=lang />
                    {show_copy.then(|| view! {
                        // <CopyButton>
                            "Copy"
                        // </CopyButton>
                    })}
                </div>
            </CodeBlockHeaderPrimitive>

            <CodeBlockContentPrimitive>
                {code_clone}
            </CodeBlockContentPrimitive>
        </CodeBlockPrimitive>
    }
}
