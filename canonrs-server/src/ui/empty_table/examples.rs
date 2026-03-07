use leptos::prelude::*;
use super::EmptyTable;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div class="border rounded">
            <table class="w-full">
                <thead class="border-b bg-muted/50">
                    <tr>
                        <th class="p-3 text-left font-semibold">"Name"</th>
                        <th class="p-3 text-left font-semibold">"Email"</th>
                        <th class="p-3 text-left font-semibold">"Status"</th>
                    </tr>
                </thead>
                <tbody>
                    <EmptyTable
                        colspan=3
                        title="No users found".to_string()
                        description="Get started by adding your first user.".to_string()
                    />
                </tbody>
            </table>
        </div>
    }
}
