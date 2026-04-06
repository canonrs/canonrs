use leptos::prelude::*;
use canonrs_core::primitives::{InputPrimitive, InputVariant, InputSize};
use canonrs_core::meta::DisabledState;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum InputGroupSlot {
    Addon(String),
    Input {
        placeholder: String,
        input_type:  String,
        name:        String,
    },
}

#[island]
pub fn InputGroupIsland(
    slots: Vec<InputGroupSlot>,
    #[prop(optional)] merge_radius: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let merge_radius = merge_radius.unwrap_or(false);
    let class        = class.unwrap_or_default();

    let focused = RwSignal::new(false);

    let state = move || {
        let mut s = vec![];
        if merge_radius  { s.push("merge-radius"); }
        if focused.get() { s.push("focus-within"); }
        s.join(" ")
    };

    let slots_view = slots.into_iter().map(|slot| {
        match slot {
            InputGroupSlot::Addon(text) => view! {
                <span data-rs-input-group-addon="">{text}</span>
            }.into_any(),
            InputGroupSlot::Input { placeholder, input_type, name } => {
                view! {
                    <InputPrimitive
                        placeholder=placeholder
                        input_type=input_type
                        name=name
                        variant=InputVariant::Default
                        size=InputSize::Md
                        disabled=DisabledState::Enabled
                        on:focus=move |_: leptos::ev::FocusEvent| { focused.set(true); }
                        on:blur=move |_: leptos::ev::FocusEvent| { focused.set(false); }
                    />
                }.into_any()
            }
        }
    }).collect::<Vec<_>>();

    view! {
        <div
            data-rs-input-group=""
            data-rs-component="InputGroup"
            data-rs-state=state
            class=class
        >
            {slots_view}
        </div>
    }
}
