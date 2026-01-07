use leptos::prelude::*;
use crate::ui::label::Label;

#[component]
pub fn Switch(
    #[prop(into)] checked: RwSignal<bool>,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] label: Option<String>,
) -> impl IntoView {
    view! {
        <div class="flex items-center gap-2">
            <button
                type="button"
                role="switch"
                aria-checked=move || if checked.get() { "true" } else { "false" }
                class=move || {
                    let base = "relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2";
                    let state = if checked.get() {
                        "bg-primary"
                    } else {
                        "bg-input border border-border" // ✅ Canon: bg-input, border-border
                    };
                    format!("{} {}", base, state)
                }
                on:click=move |_| checked.update(|c| *c = !*c)
            >
                <span
                    class=move || {
                        let base = "pointer-events-none block h-5 w-5 rounded-full bg-background shadow-lg ring-0 transition-transform";
                        let position = if checked.get() {
                            "translate-x-5"
                        } else {
                            "translate-x-0.5" // ✅ Canon: 0.5 = 2px (Tailwind scale)
                        };
                        format!("{} {}", base, position)
                    }
                />
            </button>
            {label.map(|text| view! {
                <Label>{text}</Label>
            })}
        </div>
    }
}
