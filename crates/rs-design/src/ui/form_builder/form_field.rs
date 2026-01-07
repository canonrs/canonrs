use leptos::prelude::*;
use super::types::{FieldDef, FieldType, ValidationError};

/// FormField - Renders a single form field with label, input, and error
#[component]
pub fn FormField(
    field: FieldDef,
    value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(into)] on_blur: Callback<()>,
    errors: Signal<Vec<ValidationError>>,
    touched: Signal<bool>,
) -> impl IntoView {
    let field_id = field.id.clone();
    let field_errors = Signal::derive(move || {
        errors.get()
            .into_iter()
            .filter(|e| e.field_id == field_id)
            .collect::<Vec<_>>()
    });

    let has_error = Signal::derive(move || touched.get() && !field_errors.get().is_empty());
    let input_class = move || {
        let base = "w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2";
        if has_error.get() {
            format!("{} border-red-500 focus:ring-red-500", base)
        } else {
            format!("{} border-gray-300 focus:ring-primary", base)
        }
    };
    
    view! {
        <div class="form-field mb-4">
            <label 
                for=field.id.clone()
                class="block text-sm font-medium mb-2"
            >
                {field.label.clone()}
                {if field.required {
                    view! { <span class="text-red-500 ml-1">"*"</span> }.into_any()
                } else {
                    view! { <></> }.into_any().into_any()
                }}
            </label>
            
            {match field.field_type {
                FieldType::Text | FieldType::Email | FieldType::Password | 
                FieldType::Number | FieldType::Tel | FieldType::Url => {
                    let input_type = match field.field_type {
                        FieldType::Text => "text",
                        FieldType::Email => "email",
                        FieldType::Password => "password",
                        FieldType::Number => "number",
                        FieldType::Tel => "tel",
                        FieldType::Url => "url",
                        _ => "text",
                    };
                    
                    view! {
                        <input
                            type=input_type
                            id=field.id.clone()
                            class=input_class
                            placeholder=field.placeholder.clone().unwrap_or_default()
                            value=move || value.get()
                            on:input=move |ev| on_change.run(event_target_value(&ev))
                            on:blur=move |_| on_blur.run(())
                        />
                    }.into_any()
                }
                
                FieldType::TextArea => {
                    view! {
                        <textarea
                            id=field.id.clone()
                            class=input_class
                            placeholder=field.placeholder.clone().unwrap_or_default()
                            rows=4
                            on:input=move |ev| on_change.run(event_target_value(&ev))
                            on:blur=move |_| on_blur.run(())
                        >
                            {move || value.get()}
                        </textarea>
                    }.into_any()
                }
                
                FieldType::Select => {
                    view! {
                        <select
                            id=field.id.clone()
                            class=input_class
                            on:change=move |ev| on_change.run(event_target_value(&ev))
                            on:blur=move |_| on_blur.run(())
                        >
                            <option value="">"-- Select --"</option>
                            {field.options.iter().map(|opt| {
                                let opt_value = opt.value.clone();
                                view! {
                                    <option 
                                        value=opt.value.clone()
                                        selected=move || value.get() == opt_value
                                    >
                                        {opt.label.clone()}
                                    </option>
                                }
                            }).collect_view()}
                        </select>
                    }.into_any()
                }
                
                FieldType::Checkbox => {
                    view! {
                        <div class="flex items-center">
                            <input
                                type="checkbox"
                                id=field.id.clone()
                                class="w-4 h-4 text-primary border-gray-300 rounded focus:ring-primary"
                                checked=move || value.get() == "true"
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    on_change.run(if checked { "true".to_string() } else { "false".to_string() });
                                }
                                on:blur=move |_| on_blur.run(())
                            />
                            <label for=field.id.clone() class="ml-2 text-sm">
                                {field.label.clone()}
                            </label>
                        </div>
                    }.into_any()
                }
                
                FieldType::Radio => {
                    view! {
                        <div class="space-y-2">
                            {field.options.iter().map(|opt| {
                                let opt_value = opt.value.clone();
                                let opt_value_change = opt.value.clone();
                                let field_id = field.id.clone();
                                
                                view! {
                                    <div class="flex items-center">
                                        <input
                                            type="radio"
                                            id=format!("{}_{}", field_id, opt.value)
                                            name=field_id.clone()
                                            class="w-4 h-4 text-primary border-gray-300 focus:ring-primary"
                                            checked=move || value.get() == opt_value
                                            on:change=move |_| on_change.run(opt_value_change.clone())
                                            on:blur=move |_| on_blur.run(())
                                        />
                                        <label 
                                            for=format!("{}_{}", field_id, opt.value)
                                            class="ml-2 text-sm"
                                        >
                                            {opt.label.clone()}
                                        </label>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    }.into_any()
                }
                
                FieldType::Date => {
                    view! {
                        <input
                            type="date"
                            id=field.id.clone()
                            class=input_class
                            value=move || value.get()
                            on:input=move |ev| on_change.run(event_target_value(&ev))
                            on:blur=move |_| on_blur.run(())
                        />
                    }.into_any()
                }
                
                FieldType::DateTime => {
                    view! {
                        <input
                            type="datetime-local"
                            id=field.id.clone()
                            class=input_class
                            value=move || value.get()
                            on:input=move |ev| on_change.run(event_target_value(&ev))
                            on:blur=move |_| on_blur.run(())
                        />
                    }.into_any()
                }
                
                FieldType::Time => {
                    view! {
                        <input
                            type="time"
                            id=field.id.clone()
                            class=input_class
                            value=move || value.get()
                            on:input=move |ev| on_change.run(event_target_value(&ev))
                            on:blur=move |_| on_blur.run(())
                        />
                    }.into_any()
                }
                
                FieldType::File => {
                    view! {
                        <input
                            type="file"
                            id=field.id.clone()
                            class="w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded file:border-0 file:text-sm file:font-semibold file:bg-primary file:text-primary-foreground hover:file:bg-primary/90"
                            on:change=move |ev| {
                                // File handling would need custom logic
                                on_change.run(event_target_value(&ev));
                            }
                            on:blur=move |_| on_blur.run(())
                        />
                    }.into_any()
                }
                
                FieldType::Color => {
                    view! {
                        <input
                            type="color"
                            id=field.id.clone()
                            class="w-full h-10 border rounded cursor-pointer"
                            value=move || value.get()
                            on:input=move |ev| on_change.run(event_target_value(&ev))
                            on:blur=move |_| on_blur.run(())
                        />
                    }.into_any()
                }
                
                FieldType::Range => {
                    view! {
                        <div>
                            <input
                                type="range"
                                id=field.id.clone()
                                class="w-full"
                                value=move || value.get()
                                on:input=move |ev| on_change.run(event_target_value(&ev))
                                on:blur=move |_| on_blur.run(())
                            />
                            <div class="text-sm text-gray-600 mt-1">
                                "Value: " {move || value.get()}
                            </div>
                        </div>
                    }.into_any()
                }
                
                FieldType::MultiSelect => {
                    view! {
                        <select
                            id=field.id.clone()
                            class=input_class
                            multiple
                            size=5
                            on:change=move |ev| on_change.run(event_target_value(&ev))
                            on:blur=move |_| on_blur.run(())
                        >
                            {field.options.iter().map(|opt| {
                                view! {
                                    <option value=opt.value.clone()>
                                        {opt.label.clone()}
                                    </option>
                                }
                            }).collect_view()}
                        </select>
                    }.into_any()
                }
            }}
            
            {if let Some(help) = field.help_text {
                view! {
                    <p class="text-xs text-gray-500 mt-1">{help}</p>
                }.into_any()
            } else {
                view! { <></> }.into_any()
            }}
            
            {move || {
                if has_error.get() {
                    let errors = field_errors.get();
                    view! {
                        <div class="mt-1 space-y-1">
                            {errors.into_iter().map(|err| {
                                view! {
                                    <p class="text-xs text-red-500">{err.message}</p>
                                }
                            }).collect_view()}
                        </div>
                    }.into_any()
                } else {
                    view! { <></> }.into_any()
                }
            }}
        </div>
    }
}
