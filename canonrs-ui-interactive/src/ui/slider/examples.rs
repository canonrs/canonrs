use leptos::prelude::*;
use super::slider_interactive::*;

pub fn interactive_example() -> impl IntoView {
    let (value, set_value) = signal(50.0);
    
    view! {
        <div>
            <SliderInteractive
                value=value.get()
                on_change=Callback::new(move |v: f64| {
                    set_value.set(v);
                })
            />
            <p>"Value: " {move || value.get()}</p>
        </div>
    }
}
