use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum TextareaSize {
    Small,      // h-20 (80px)
    Medium,     // h-32 (128px)
    Large,      // h-40 (160px)
}

impl Default for TextareaSize {
    fn default() -> Self {
        Self::Medium
    }
}

#[component]
pub fn Textarea(
    #[prop(into)] value: RwSignal<String>,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional)] size: TextareaSize,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let size_class = match size {
        TextareaSize::Small => "h-20",   // ✅ Canon: h-20 (80px)
        TextareaSize::Medium => "h-32",  // ✅ Canon: h-32 (128px)
        TextareaSize::Large => "h-40",   // ✅ Canon: h-40 (160px)
    };
    
    view! {
        <textarea
            class=format!(
                "flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 {} {}",
                size_class,
                class
            )
            placeholder=placeholder
            prop:value=move || value.get()
            on:input=move |ev| {
                value.set(event_target_value(&ev));
            }
        />
    }
}
