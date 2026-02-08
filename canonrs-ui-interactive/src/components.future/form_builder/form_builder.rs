use leptos::callback::Callback;
use super::form_state::FormStateManager;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use crate::ui::button::Button;
use super::form_field::FormField;

#[component]
pub fn FormBuilder(
    state: FormStateManager,
    #[prop(optional)] on_submit: Option<Callback<Vec<(String, String)>>>,
    #[prop(optional)] on_cancel: Option<Callback<()>>,
    #[prop(default = "Submit".to_string())] submit_label: String,
    #[prop(optional)] cancel_label: Option<String>,
    #[prop(default = false)] show_reset: bool,
) -> impl IntoView {
    let fields = state.fields().to_vec();
    let cancel_label_signal = RwSignal::new(cancel_label);
    let show_reset_signal = RwSignal::new(show_reset);
    
    // Wrap state em Signal para permitir múltiplos clones
    let state_signal = RwSignal::new(state);

    view! {
        <form class="space-y-4">
            <div class="space-y-4">
                {fields.into_iter().map(|field| {
                    let state = state_signal.get();
                    let value = state.get_value(&field.id);
                    let errors = state.get_errors();
                    let touched = state.is_touched(&field.id);
                    
                    view! {
                        <FormField
                            field={field}
                            value={value}
                            errors={errors}
                            touched={touched}
                        />
                    }
                }).collect_view()}
            </div>

            <div class="flex gap-2 mt-6">
                <Button
                    container_class="px-4 py-2 bg-blue-600 text-white".to_string()
                    on_click=Callback::new(move |_: MouseEvent| {
                        let state = state_signal.get();
                        if let Some(submit_cb) = on_submit {
                            if state.validate_all() {
                                let values: Vec<(String, String)> = state.get_all_values()
                                    .into_iter()
                                    .collect();
                                submit_cb.run(values);
                            }
                        }
                    })
                >
                    {submit_label}
                </Button>

                <Show when=move || cancel_label_signal.get().is_some()>
                    <Button
                        container_class="px-4 py-2 border".to_string()
                        on_click=Callback::new(move |_: MouseEvent| {
                            if let Some(cancel_cb) = on_cancel {
                                cancel_cb.run(());
                            }
                        })
                    >
                        {move || cancel_label_signal.get().unwrap_or_default()}
                    </Button>
                </Show>

                <Show when=move || show_reset_signal.get()>
                    <Button
                        container_class="px-4 py-2 border".to_string()
                        on_click=Callback::new(move |_: MouseEvent| {
                            state_signal.get().reset();
                        })
                    >
                        "Reset"
                    </Button>
                </Show>
            </div>
        </form>
    }
}
