use leptos::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct FormContext {
    pub values: RwSignal<HashMap<String, String>>,
    pub errors: RwSignal<HashMap<String, String>>,
}

impl FormContext {
    pub fn new() -> Self {
        Self {
            values: RwSignal::new(HashMap::new()),
            errors: RwSignal::new(HashMap::new()),
        }
    }

    pub fn set_field(&self, name: String, value: String) {
        self.values.update(|v| {
            v.insert(name, value);
        });
    }

    pub fn get_field(&self, name: &str) -> String {
        self.values
            .with(|v| v.get(name).cloned().unwrap_or_default())
    }

    pub fn set_error(&self, name: String, error: String) {
        self.errors.update(|e| {
            e.insert(name, error);
        });
    }

    pub fn clear_errors(&self) {
        self.errors.update(|e| e.clear());
    }

    pub fn get_values(&self) -> HashMap<String, String> {
        self.values.get()
    }
}

#[component]
pub fn Form(
    children: Children,
    #[prop(optional)] on_submit: Option<Callback<HashMap<String, String>>>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let ctx = FormContext::new();
    let ctx_clone = ctx.clone();

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        ctx_clone.clear_errors();
        if let Some(callback) = on_submit {
            callback.run(ctx_clone.get_values());
        }
    };

    view! {
        <leptos::context::Provider value=ctx>
            <form on:submit=handle_submit class=class>
                {children()}
            </form>
        </leptos::context::Provider>
    }
}

#[component]
pub fn FormField(
    #[prop(into)] name: String,
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let ctx = use_context::<FormContext>().expect("FormField must be inside Form");

    let error = Memo::new(move |_| ctx.errors.with(|e| e.get(&name).cloned()));

    view! {
        <div class=class>
            {children()}
            {move || error.get().map(|err| view! {
                <p class="text-sm text-destructive mt-1">{err}</p>
            })}
        </div>
    }
}
