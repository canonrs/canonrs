use leptos::prelude::*;

#[island]
pub fn FormIsland(
    #[prop(optional, into)] action: Option<String>,
    #[prop(optional, into)] submit_label: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let class        = class.unwrap_or_default();
    let action       = action.unwrap_or_default();
    let submit_label = submit_label.unwrap_or_else(|| "Submit".to_string());
    let (loading, set_loading) = signal(false);
    let _ = set_loading;

    let validation = move || if loading.get() { "loading" } else { "idle" };

    #[cfg(feature = "hydrate")]
    let on_submit = move |e: leptos::ev::SubmitEvent| {
        e.prevent_default();
        set_loading.set(true);
    };
    #[cfg(not(feature = "hydrate"))]
    let on_submit = move |e: leptos::ev::SubmitEvent| { e.prevent_default(); };

    view! {
        <form
            data-rs-form=""
            data-rs-component="Form"
            data-rs-validation=move || validation()
            action=action
            novalidate
            class=class
            on:submit=on_submit
        >
            {children()}
            <div data-rs-form-actions="">
                <button
                    type="submit"
                    data-rs-button=""
                    data-rs-variant="primary"
                    disabled=move || loading.get()
                >
                    {move || if loading.get() { "Loading...".to_string() } else { submit_label.clone() }}
                </button>
            </div>
        </form>
    }
}
