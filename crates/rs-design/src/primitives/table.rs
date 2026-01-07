//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[component]
pub fn TablePrimitive(children: Children, #[prop(default = String::new(), into)] class: String) -> impl IntoView {
    view! {
        <div class=format!("relative w-full overflow-auto {}", class)>
            <table class="w-full caption-bottom text-sm">
                {children()}
            </table>
        </div>
    }
}

#[component]
pub fn TableHeaderPrimitive(children: Children, #[prop(default = String::new(), into)] class: String) -> impl IntoView {
    view! {
        <thead class=format!("[&_tr]:border-b {}", class)>
            {children()}
        </thead>
    }
}

#[component]
pub fn TableBodyPrimitive(children: Children, #[prop(default = String::new(), into)] class: String) -> impl IntoView {
    view! {
        <tbody class=format!("[&_tr:last-child]:border-0 {}", class)>
            {children()}
        </tbody>
    }
}

#[component]
pub fn TableFooterPrimitive(children: Children, #[prop(default = String::new(), into)] class: String) -> impl IntoView {
    view! {
        <tfoot class=format!("border-t bg-muted/50 font-medium [&>tr]:last:border-b-0 {}", class)>
            {children()}
        </tfoot>
    }
}

#[component]
pub fn TableRowPrimitive(children: Children, #[prop(default = String::new(), into)] class: String) -> impl IntoView {
    view! {
        <tr class=format!("border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted {}", class)>
            {children()}
        </tr>
    }
}

#[component]
pub fn TableHeadPrimitive(children: Children, #[prop(default = String::new(), into)] class: String) -> impl IntoView {
    view! {
        <th class=format!("h-12 px-4 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 {}", class)>
            {children()}
        </th>
    }
}

#[component]
pub fn TableCellPrimitive(children: Children, #[prop(default = String::new(), into)] class: String) -> impl IntoView {
    view! {
        <td class=format!("p-4 align-middle [&:has([role=checkbox])]:pr-0 {}", class)>
            {children()}
        </td>
    }
}

#[component]
pub fn TableCaptionPrimitive(children: Children, #[prop(default = String::new(), into)] class: String) -> impl IntoView {
    view! {
        <caption class=format!("mt-4 text-sm text-muted-foreground {}", class)>
            {children()}
        </caption>
    }
}
