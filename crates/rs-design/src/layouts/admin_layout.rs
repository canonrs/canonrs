// TEMPORARILY DISABLED - FnOnce issues to be resolved
// Will be fixed in next iteration

use leptos::prelude::*;

#[component]
pub fn AdminLayout(
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <div class="p-6">
            <h1>"Admin Layout (under construction)"</h1>
            {children()}
        </div>
    }
}
