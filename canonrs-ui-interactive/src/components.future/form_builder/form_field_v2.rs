use leptos::prelude::*;
use crate::ui::{Input, Textarea, Select, Checkbox};
use super::types::{FieldDef, FieldType, ValidationError};

// Component principal - match retorna View unificado
#[component]
pub fn FormFieldV2(
    field: FieldDef,
    value: RwSignal<String>,
    errors: RwSignal<Vec<ValidationError>>,
    touched: RwSignal<bool>,
) -> impl IntoView {
    let field_errors = Memo::new(move |_| errors.get());
    let has_error = Memo::new(move |_| touched.get() && !field_errors.get().is_empty());
    let error_class = Memo::new(move |_| {
        if has_error.get() {
            "border-red-500"
        } else {
            ""
        }
    });
    
    let placeholder = field.placeholder.clone().unwrap_or_default();
    let field_id = field.id.clone();
    let field_label = RwSignal::new(field.label.clone());
    let field_required = field.required;
    let field_help = RwSignal::new(field.help_text.clone());
    
    view! {
        <div class="form-field">
            <label for={field_id.clone()} class="block text-sm font-medium mb-1">
                {field_label.get()}
                <Show when=move || field_required>
                    <span class="text-red-500">"*"</span>
                </Show>
            </label>

            {match field.field_type {
                FieldType::Text | FieldType::Email | FieldType::Password | FieldType::Number => {
                    view! {
                        <Input
                            attr:data-field-type="text"
                            id={field_id.clone()}
                            placeholder={placeholder}
                            value={value.get()}
                            class={error_class.get().to_string()}
                            on:input=move |ev| {
                                value.set(event_target_value(&ev));
                            }
                        />
                    }.into_any()
                }
                FieldType::TextArea => {
                    view! {
                        <Textarea
                            attr:data-field-type="textarea"
                            id={field_id.clone()}
                            placeholder={placeholder}
                            value={value.get()}
                            class={error_class.get().to_string()}
                        />
                    }.into_any()
                }
                FieldType::Select => {
                    let opts = field.options.clone();
                    view! {
                        <Select id={field_id.clone()} class={error_class.get().to_string()}>
                            <option value="">"-- Select --"</option>
                            {opts.into_iter().map(|opt| {
                                let is_selected = value.get() == opt.value;
                                view! {
                                    <option
                                        value={opt.value}
                                        selected=is_selected
                                    >
                                        {opt.label}
                                    </option>
                                }
                            }).collect_view()}
                        </Select>
                    }.into_any()
                }
                FieldType::Checkbox => {
                    view! {
                        <div class="flex items-center">
                            <Checkbox
                                id={field_id.clone()}
                                checked={value.get() == "true"}
                            />
                            <label for={field_id.clone()} class="ml-2 text-sm">
                                {field_label.get()}
                            </label>
                        </div>
                    }.into_any()
                }
                FieldType::Date => {
                    view! {
                        <Input
                            attr:data-field-type="text"
                            id={field_id.clone()}
                            value={value.get()}
                            class={error_class.get().to_string()}
                        />
                    }.into_any()
                }
                _ => {
                    view! { <div>"Unsupported field type"</div> }.into_any()
                }
            }}

            <Show when=move || field_help.get().is_some()>
                <p class="text-xs text-gray-500 mt-1">{field_help.get().unwrap_or_default()}</p>
            </Show>

            <Show when=move || has_error.get()>
                <div class="text-xs text-red-500 mt-1">
                    {move || {
                        field_errors.get().into_iter()
                            .map(|err| view! { <p>{err.message}</p> })
                            .collect_view()
                    }}
                </div>
            </Show>
        </div>
    }
}
