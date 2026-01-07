use crate::primitives::pagination::*;
use crate::ui::button::ButtonVariant;
use leptos::prelude::*;

#[component]
pub fn Pagination(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <PaginationPrimitive>
            <nav class=format!("mx-auto flex w-full justify-center {}", class)>
                {children()}
            </nav>
        </PaginationPrimitive>
    }
}

#[component]
pub fn PaginationContent(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <PaginationContentPrimitive>
            <ul class=format!("flex flex-row items-center gap-1 {}", class)>
                {children()}
            </ul>
        </PaginationContentPrimitive>
    }
}

#[component]
pub fn PaginationItem(
    children: Children,
) -> impl IntoView {
    view! {
        <PaginationItemPrimitive>
            <li>
                {children()}
            </li>
        </PaginationItemPrimitive>
    }
}

#[component]
pub fn PaginationLink(
    children: Children,
    #[prop(default = false)] is_active: bool,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = Callback::new(|_| {}))] on_click: Callback<()>,
) -> impl IntoView {
    let _variant = if is_active { ButtonVariant::Outline } else { ButtonVariant::Ghost };
    let base = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 h-9 w-9";
    
    view! {
        <PaginationLinkPrimitive is_active=is_active>
            <a 
                class=format!("{} {}", base, class)
                on:click=move |_| on_click.run(())
            >
                {children()}
            </a>
        </PaginationLinkPrimitive>
    }
}

#[component]
pub fn PaginationPrevious(
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = Callback::new(|_| {}))] on_click: Callback<()>,
) -> impl IntoView {
    view! {
        <PaginationLink class=format!("gap-1 px-2.5 sm:pl-2.5 {}", class) on_click=on_click>
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                <path d="m15 18-6-6 6-6"/>
            </svg>
            <span class="hidden sm:block">Previous</span>
        </PaginationLink>
    }
}

#[component]
pub fn PaginationNext(
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = Callback::new(|_| {}))] on_click: Callback<()>,
) -> impl IntoView {
    view! {
        <PaginationLink class=format!("gap-1 px-2.5 sm:pr-2.5 {}", class) on_click=on_click>
            <span class="hidden sm:block">Next</span>
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                <path d="m9 18 6-6-6-6"/>
            </svg>
        </PaginationLink>
    }
}

#[component]
pub fn PaginationEllipsis(
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <PaginationEllipsisPrimitive>
            <span class=format!("flex size-9 items-center justify-center {}", class)>
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                    <circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/>
                </svg>
                <span class="sr-only">More pages</span>
            </span>
        </PaginationEllipsisPrimitive>
    }
}
