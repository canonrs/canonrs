use leptos::prelude::*;

#[component]
pub fn Section(
    children: Children,
    #[prop(optional)] header: Option<Children>,
    #[prop(optional)] footer: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <section
            id={if id.is_empty() { None } else { Some(id) }}
            class={format!("layout-section {}", class)}
            data-layout="section"
        >
            {header.map(|h| view! {
                <div class="layout-section__header" data-layout-region="header">
                    {h()}
                </div>
            })}
            <div class="layout-section__content" data-layout-region="content">
                {children()}
            </div>
            {footer.map(|f| view! {
                <div class="layout-section__footer" data-layout-region="footer">
                    {f()}
                </div>
            })}
        </section>
    }
}
