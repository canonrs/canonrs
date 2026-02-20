//! # Dialog Block
//! Modal dialog overlay with backdrop

use leptos::prelude::*;

#[component]
pub fn DialogBlock(
    #[prop(into)] open: MaybeSignal<bool>,
    #[prop(optional)] on_close: Option<Callback<()>>,
    #[prop(optional)] title: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(optional)] close_button: Option<ChildrenFn>,
    #[prop(default = true)] backdrop: bool,
    #[prop(default = true)] close_on_backdrop: bool,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    let handle_backdrop_click = move |_| {
        if close_on_backdrop {
            if let Some(cb) = on_close {
                cb.run(());
            }
        }
    };

    view! {
        <div
            class=move || format!(
                "canon-dialog-overlay {}",
                if open.get() { "canon-dialog-overlay--open" } else { "" }
            )
            attr:data-block="dialog"
            attr:data-action="close-backdrop" on:click=handle_backdrop_click
        >
            {backdrop.then(|| view! {
                <div class="canon-dialog__backdrop" />
            })}

            <div
                class=format!("canon-dialog {}", class)
                attr:data-action="prevent-close" on:click=|e| e.stop_propagation()
            >
                {title.map(|t| view! {
                    <div class="canon-dialog__header">
                        <div class="canon-dialog__title">{t()}</div>
                        {close_button.map(|btn| view! {
                            <div class="canon-dialog__close">{btn()}</div>
                        })}
                    </div>
                })}

                <div class="canon-dialog__content">
                    {children()}
                </div>

                {footer.map(|f| view! {
                    <div class="canon-dialog__footer">{f()}</div>
                })}
            </div>
        </div>
    }
}
