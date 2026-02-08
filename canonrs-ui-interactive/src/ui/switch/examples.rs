use leptos::prelude::*;
use super::switch_interactive::*;

pub fn interactive_example() -> impl IntoView {
    let (checked, set_checked) = signal(false);
    
    view! {
        <div>
            <SwitchInteractive
                id="switch-notifications".to_string()
                checked=checked.get()
                name="notifications".to_string()
                value="on".to_string()
                on_change=Callback::new(move |_| {
                    set_checked.update(|v| *v = !*v);
                })
            >
                <span>"Notifications"</span>
            </SwitchInteractive>
            <p>"Checked: " {move || checked.get().to_string()}</p>
        </div>
    }
}
