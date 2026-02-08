use leptos::callback::Callback;
use leptos::prelude::*;
use canonrs_ui::ui::switch::Switch;

#[component]
pub fn SwitchInteractive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] name: String,
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(optional)] on_change: Option<Callback<bool>>,
) -> impl IntoView {
    view! {
        <Switch
            checked=checked
            disabled=disabled
            name=name
            value=value
            class=class
            id=id
            on:click=move |_| {
                if !disabled {
                    if let Some(ref handler) = on_change {
                        handler.run(!checked);
                    }
                }
            }
        >
            {children.map(|c| c())}
        </Switch>
    }
}
