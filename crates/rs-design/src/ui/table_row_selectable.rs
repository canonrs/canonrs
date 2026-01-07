use leptos::prelude::*;

#[component]
pub fn TableRowSelectable(
    #[prop(into)] selected: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <tr
            class="border-b transition-colors bg-transparent"
            class=("!bg-[hsl(var(--color-muted)/0.95)]", move || selected.get())
            class=("hover:!bg-[hsl(var(--color-muted)/0.7)]", move || !selected.get())
            class=("hover:!bg-muted", move || selected.get())
        >
            {children()}
        </tr>
    }
}
