//! # AlertBlock / Callout
//! Important messages, warnings, contextual information

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum AlertVariant {
    Info,
    Success,
    Warning,
    Error,
}

impl AlertVariant {
    fn as_str(&self) -> &'static str {
        match self {
            AlertVariant::Info => "info",
            AlertVariant::Success => "success",
            AlertVariant::Warning => "warning",
            AlertVariant::Error => "error",
        }
    }
}

#[component]
pub fn AlertBlock(
    #[prop(default = AlertVariant::Info)] variant: AlertVariant,
    #[prop(optional)] title: Option<ChildrenFn>,
    #[prop(optional, into)] open: Option<MaybeSignal<bool>>,
    #[prop(optional)] on_dismiss: Option<Callback<()>>,
    #[prop(default = false)] dismissible: bool,
    #[prop(optional)] close_button: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    let is_open = move || open.as_ref().map(|o| o.get()).unwrap_or(true);

    view! {
        <div
            class=move || format!("canon-alert canon-alert--{} {}{}",
                variant.as_str(),
                class,
                if is_open() { "" } else { " canon-alert--hidden" }
            )
            attr:data-block="alert"
            attr:data-variant=variant.as_str()
        >
            {title.map(|t| view! {
                <div class="canon-alert__title">{t()}</div>
            })}

            <div class="canon-alert__content">
                {children()}
            </div>

            {dismissible.then(|| {
                close_button.map(|btn| view! {
                    <div
                        class="canon-alert__close"
                        attr:data-action="dismiss" on:click=move |_| {
                            if let Some(cb) = on_dismiss {
                                cb.run(());
                            }
                        }
                    >
                        {btn()}
                    </div>
                })
            })}
        </div>
    }
}
