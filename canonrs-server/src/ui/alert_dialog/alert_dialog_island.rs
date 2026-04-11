//! @canon-level: strict
//! AlertDialog Island — Canon Rule #340 (zero-logic boundary)
//! CR-342 v3.0.0: interaction delegated to canonrs-interactions-overlay

use leptos::prelude::*;
use super::alert_dialog_ui::{
    AlertDialog, AlertDialogTrigger, AlertDialogOverlay,
    AlertDialogContent, AlertDialogTitle, AlertDialogDescription,
    AlertDialogClose,
};
use crate::ui::button::ButtonVariant;

#[component]
pub fn AlertDialogIsland(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::from("Delete"))] trigger_label: String,
    #[prop(into, default = String::from("Confirm"))] confirm_label: String,
    #[prop(into, default = String::from("Cancel"))] cancel_label: String,
    #[prop(into, optional)] title: Option<String>,
    #[prop(into, optional)] description: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AlertDialog class=class>
            <AlertDialogTrigger variant=ButtonVariant::Destructive>
                {trigger_label}
            </AlertDialogTrigger>
            <AlertDialogOverlay />
            <AlertDialogContent>
                {title.map(|t| view! { <AlertDialogTitle>{t}</AlertDialogTitle> })}
                {description.map(|d| view! { <AlertDialogDescription>{d}</AlertDialogDescription> })}
                {children.map(|c| c())}
                <div data-rs-alert-dialog-actions="">
                    <AlertDialogClose variant=ButtonVariant::Outline>
                        {cancel_label}
                    </AlertDialogClose>
                    <AlertDialogClose variant=ButtonVariant::Destructive>
                        {confirm_label}
                    </AlertDialogClose>
                </div>
            </AlertDialogContent>
        </AlertDialog>
    }
}
