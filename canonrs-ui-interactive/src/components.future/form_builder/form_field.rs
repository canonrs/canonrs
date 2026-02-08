use leptos::prelude::*;
use leptos::ev;
use super::types::{FieldDef, FieldType, ValidationError};
use crate::ui::{Input, Textarea, Select, Checkbox};

#[component]
pub fn FormField(
    field: FieldDef,
    value: RwSignal<String>,
    errors: RwSignal<Vec<ValidationError>>,
    touched: RwSignal<bool>,
) -> impl IntoView {
    let field_id = field.id.clone();

    let field_errors = Memo::new(move |_| {
        errors.get()
            .into_iter()
            .filter(|e| e.field_id == field_id)
            .collect::<Vec<_>>()
    });

    let has_error = Memo::new(move |_| touched.get() && !field_errors.get().is_empty());

    // Normalizar placeholder antes
    let placeholder = field.placeholder.clone().unwrap_or(String::new());
    let error_class = if has_error.get() { "border-red-500" } else { "" };

    view! {
        <div class="form-field mb-4">
            <label
                for={field.id.clone()}
                class="block text-sm font-medium mb-2"
            >
                {field.label.clone()}
                {field.required.then(|| view! {
                    <span class="text-red-500 ml-1">"*"</span>
                })}
            </label>

            {match field.field_type {
                FieldType::Text | FieldType::Email | FieldType::Password |
                FieldType::Number | FieldType::Tel | FieldType::Url => {
                    view! {
                        <Input
                            id={field.id.clone()}
                            placeholder={placeholder.clone()}
                            value={value.get()}
                            class={error_class.to_string()}
                            on_input=Callback::new(move |ev: ev::Event| {
                                value.set(event_target_value(&ev));
                            })
                        />
                    }.into_any()
                }

                FieldType::TextArea => view! {
                    <Textarea
                        id={field.id.clone()}
                        placeholder={placeholder.clone()}
                        value={value.get()}
                        class={error_class.to_string()}
                    />
                }.into_any() as leptos::prelude::AnyView,

                FieldType::Select => view! {
                    <Select
                        id={field.id.clone()}
                        class={error_class.to_string()}
                    >
                        <option value="">"-- Select --"</option>
                        {field.options.iter().map(|opt| view! {
                            <option
                                value={opt.value.clone()}
                                prop:selected={value.get() == opt.value}
                            >
                                {opt.label.clone()}
                            </option>
                        }).collect_view()}
                    </Select>
                }.into_any() as leptos::prelude::AnyView,

                FieldType::Checkbox => view! {
                    <div class="flex items-center">
                        <Checkbox
                            id={field.id.clone()}
                            checked={value.get() == "true"}
                        />
                        <label
                            for={field.id.clone()}
                            class="ml-2 text-sm"
                        >
                            {field.label.clone()}
                        </label>
                    </div>
                }.into_any() as leptos::prelude::AnyView,

                FieldType::Date => view! {
                    <Input
                        id={field.id.clone()}
                        value={value.get()}
                        class={error_class.to_string()}
                    />
                }.into_any() as leptos::prelude::AnyView,

                _ => view! { <div>"Unsupported field type"</div> }.into_any() as leptos::prelude::AnyView,
            }}

            {field.help_text.as_ref().map(|help| view! {
                <p class="text-xs text-gray-500 mt-1">{help.clone()}</p>
            })}

            {has_error.get().then(|| view! {
                <div class="mt-1 space-y-1">
                    {field_errors.get().into_iter().map(|err| view! {
                        <p class="text-xs text-red-500">{err.message}</p>
                    }).collect_view()}
                </div>
            })}
        </div>
    }
}
