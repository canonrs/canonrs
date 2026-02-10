use leptos::prelude::*;

#[component]
pub fn CommandInteractive(
    #[prop(into)] id: String,
    items: Vec<String>,
) -> impl IntoView {
    let search_value = RwSignal::new(String::new());
    
    view! {
        <div data-command id=id>
            <input 
                data-command-input
                placeholder="Type a command..."
                on:input=move |ev| {
                    search_value.set(event_target_value(&ev));
                }
            />
            <div data-command-list>
                {items.into_iter().map(|item| {
                    view! {
                        <div data-command-item>{item}</div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
