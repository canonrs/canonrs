use leptos::prelude::*;
use super::textarea_interactive::*;

pub fn interactive_example() -> impl IntoView {
    let (value, set_value) = signal(String::new());
    
    view! {
        <div>
            <TextareaInteractive
                placeholder="Enter description...".to_string()
                on_input=Callback::new(move |v: String| {
                    set_value.set(v);
                })
            />
            <p>"Value: " {move || value.get()}</p>
        </div>
    }
}
