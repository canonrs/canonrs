//! AlertDialog Island — Canon Rule #342
//! Bootstrap only. Toda a lógica vive em canonrs-client/src/interactions/alert_dialog.rs

use leptos::prelude::*;

#[island]
pub fn AlertDialogIsland(
    #[prop(optional, into)] trigger_label: Option<String>,
    #[prop(optional, into)] title:         Option<String>,
    #[prop(optional, into)] description:   Option<String>,
    #[prop(optional, into)] confirm_label: Option<String>,
    #[prop(optional, into)] cancel_label:  Option<String>,
    #[prop(optional, into)] class:         Option<String>,
) -> impl IntoView {
    let class         = class.unwrap_or_default();
    let trigger_label = trigger_label.unwrap_or_else(|| "Delete".to_string());
    let confirm_label = confirm_label.unwrap_or_else(|| "Confirm".to_string());
    let cancel_label  = cancel_label.unwrap_or_else(|| "Cancel".to_string());

    #[cfg(feature = "hydrate")]
    {
        use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
            canonrs_client::interactions::alert_dialog::init_all();
        });
    }

    view! {
        <div
            data-rs-alert-dialog=""
            data-rs-component="AlertDialog"
            data-rs-state="closed"
            class=class
        >
            <button
                type="button"
                data-rs-alert-dialog-trigger=""
                data-rs-button=""
                data-rs-variant="destructive"
                aria-haspopup="alertdialog"
            >
                {trigger_label}
            </button>
            <div data-rs-alert-dialog-portal="">
                <div data-rs-alert-dialog-overlay=""></div>
                <div
                    data-rs-alert-dialog-content=""
                    role="alertdialog"
                    aria-modal="true"
                    aria-live="assertive"
                    tabindex="-1"
                >
                    {title.map(|t| view! { <h2 data-rs-alert-dialog-title="">{t}</h2> })}
                    {description.map(|d| view! { <p data-rs-alert-dialog-description="">{d}</p> })}
                    <div data-rs-alert-dialog-actions="">
                        <button type="button" data-rs-button="" data-rs-variant="outline"
                            data-rs-alert-dialog-cancel="">
                            {cancel_label}
                        </button>
                        <button type="button" data-rs-button="" data-rs-variant="destructive"
                            data-rs-alert-dialog-confirm="">
                            {confirm_label}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
