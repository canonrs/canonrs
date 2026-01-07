use leptos::prelude::*;
use super::types::FormSchema;
use super::form_state::FormStateManager;
use super::form_field::FormField;
use std::sync::Arc;

/// FormBuilder - Main form component
#[component]
pub fn FormBuilder(
    schema: FormSchema,
    #[prop(into)] on_submit: Callback<std::collections::HashMap<String, String>>,
    #[prop(optional)] on_cancel: Option<Callback<()>>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let state = Arc::new(FormStateManager::new(schema.clone()));
    
    let state_submit = Arc::clone(&state);
    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        if state_submit.validate_all() {
            state_submit.set_submitting(true);
            let values = state_submit.get_all_values();
            on_submit.run(values);
        }
    };
    
    let state_reset = Arc::clone(&state);
    let handle_reset = move |_| {
        state_reset.reset();
    };
    
    view! {
        <form 
            class=format!("form-builder {}", class)
            on:submit=handle_submit
        >
            <div class="mb-6">
                <h2 class="text-2xl font-bold mb-2">{schema.title.clone()}</h2>
            </div>
            
            <div class="space-y-4">
                {schema.fields.iter().map(|field| {
                    let state_field = Arc::clone(&state);
                    let field_id = field.id.clone();
                    let field_clone = field.clone();
                    
                    let value = state_field.get_value(&field_id);
                    let errors = state_field.get_errors();
                    let touched = state_field.is_touched(&field_id);
                    let is_visible = state_field.is_field_visible(&field_clone);
                    
                    let state_change = Arc::clone(&state);
                    let field_id_change = field_id.clone();
                    let on_change = Callback::new(move |new_value: String| {
                        state_change.set_value(field_id_change.clone(), new_value);
                    });
                    
                    let state_blur = Arc::clone(&state);
                    let field_id_blur = field_id.clone();
                    let on_blur = Callback::new(move |_: ()| {
                        state_blur.set_touched(field_id_blur.clone());
                    });
                    
                    view! {
                        <Show when=move || is_visible.get()>
                            <FormField
                                field=field_clone.clone()
                                value=value
                                on_change=on_change
                                on_blur=on_blur
                                errors=errors
                                touched=touched
                            />
                        </Show>
                    }
                }).collect_view()}
            </div>
            
            {
            let state_disabled = Arc::clone(&state);
            let state_content = Arc::clone(&state);
            
            view! {
            <div class="flex gap-3 mt-6">
            

                <button
                    type="submit"
                    class="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed"
                    disabled=move || {
                        // state_disabled já clonado acima
                        state_disabled.is_submitting.get()
                    }
                >
                    {move || {
                        // state_content já clonado acima
                        if state_content.is_submitting.get() {
                            view! {
                                <span class="flex items-center gap-2">
                                    <svg class="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                    </svg>
                                    "Submitting..."
                                </span>
                            }.into_any()
                        } else {
                            view! { <span class="">{schema.submit_label.clone()}</span> }.into_any()
                        }
                    }}
                </button>
                {if let Some(cancel_callback) = on_cancel {
                    view! {
                        <button
                            type="button"
                            class="px-4 py-2 border rounded-md hover:bg-muted"
                            on:click=move |_| cancel_callback.run(())
                        >
                            {schema.cancel_label.clone().unwrap_or_else(|| "Cancel".to_string())}
                        </button>
                    }.into_any()
                } else {
                    view! { <></> }.into_any()
                }}
                
                <button
                    type="button"
                    class="px-4 py-2 border rounded-md hover:bg-muted"
                    on:click=handle_reset
                >
                    "Reset"
                </button>
            </div>
            
            }}

            {move || {
                let state_footer = Arc::clone(&state);
                let errors = state_footer.errors.get();
                if !errors.is_empty() && state_footer.touched.get().values().any(|&v| v) {
                    let error_count = errors.len();
                    view! {
                        <div class="mt-4 p-3 bg-red-50 border border-red-200 rounded-md">
                            <p class="text-sm text-red-800 font-medium">
                                {if error_count == 1 {
                                    "Please fix 1 error before submitting".to_string()
                                } else {
                                    format!("Please fix {} errors before submitting", error_count)
                                }}
                            </p>
                        </div>
                    }.into_any()
                } else {
                    view! { <></> }.into_any()
                }
            }}
        </form>
    }
}

/// QuickForm - Simplified wrapper
#[component]
pub fn QuickForm(
    title: String,
    fields: Vec<super::types::FieldDef>,
    #[prop(into)] on_submit: Callback<std::collections::HashMap<String, String>>,
    #[prop(optional)] on_cancel: Option<Callback<()>>,
) -> impl IntoView {
    let schema = FormSchema {
        id: "quick-form".to_string(),
        title,
        fields,
        submit_label: "Submit".to_string(),
        cancel_label: Some("Cancel".to_string()),
    };
    
    view! {
        <FormBuilder
            schema=schema
            on_submit=on_submit
            
        />
    }
}
