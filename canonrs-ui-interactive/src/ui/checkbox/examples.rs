use leptos::prelude::*;
use super::checkbox_interactive::*;

pub fn interactive_example() -> impl IntoView {
    let (checked, set_checked) = signal(false);
    
    view! {
        <div>
            <CheckboxInteractive
                id="checkbox-interactive-1".to_string()
                checked=checked.get()
                name="agree".to_string()
                value="yes".to_string()
                on_change=Callback::new(move |_| {
                    set_checked.update(|v| *v = !*v);
                })
            >
                <span>"Accept terms (interactive)"</span>
            </CheckboxInteractive>
            <p>"Checked: " {move || checked.get().to_string()}</p>
        </div>
    }
}
