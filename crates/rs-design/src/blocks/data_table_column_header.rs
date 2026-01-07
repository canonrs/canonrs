use leptos::prelude::*;
use crate::ui::button::{ButtonVariant, ButtonType};
use crate::ui::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SortDirection {
    Asc,
    Desc,
    None,
}

#[component]
pub fn DataTableColumnHeader(
    #[prop(into)] title: String,
    #[prop(default = SortDirection::None)] sort_direction: SortDirection,
    #[prop(default = true)] can_sort: bool,
    #[prop(default = true)] can_hide: bool,
    #[prop(optional)] on_sort: Option<Callback<()>>,
    #[prop(optional)] on_hide: Option<Callback<()>>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let title = StoredValue::new(title);
    
    if !can_sort {
        return view! {
            <div class=format!("flex items-center gap-2 {}", class)>
                {title.get_value()}
            </div>
        }.into_any();
    }

    view! {
        <div class=format!("flex items-center gap-2 {}", class)>
            <DropdownMenu>
                <DropdownMenuTrigger>
                    <Button variant=ButtonVariant::Ghost class="-ml-3 h-8">
                        <span>{title.get_value()}</span>
                        {move || match sort_direction {
                            SortDirection::Desc => view! {
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="ml-2 h-4 w-4">
                                    <path d="m7 20 5-5 5 5"/><path d="m7 4 5 5 5-5"/>
                                </svg>
                            }.into_any(),
                            SortDirection::Asc => view! {
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="ml-2 h-4 w-4">
                                    <path d="m7 15 5 5 5-5"/><path d="m7 9 5-5 5 5"/>
                                </svg>
                            }.into_any(),
                            SortDirection::None => view! {
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="ml-2 h-4 w-4">
                                    <path d="m7 15 5 5 5-5"/><path d="m7 9 5-5 5 5"/>
                                </svg>
                            }.into_any(),
                        }}
                    </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent>
                    <DropdownMenuItem on_select=Callback::new(move |_| {
                        if let Some(cb) = on_sort { cb.run(()); }
                    })>
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2 h-4 w-4">
                            <path d="m7 15 5 5 5-5"/><path d="m7 9 5-5 5 5"/>
                        </svg>
                        Asc
                    </DropdownMenuItem>
                    <DropdownMenuItem on_select=Callback::new(move |_| {
                        if let Some(cb) = on_sort { cb.run(()); }
                    })>
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2 h-4 w-4">
                            <path d="m7 20 5-5 5 5"/><path d="m7 4 5 5 5-5"/>
                        </svg>
                        Desc
                    </DropdownMenuItem>
                    {can_hide.then(|| view! {
                        <>
                            <DropdownMenuSeparator />
                            <DropdownMenuItem on_select=Callback::new(move |_| {
                                if let Some(cb) = on_hide { cb.run(()); }
                            })>
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2 h-4 w-4">
                                    <path d="M9.88 9.88a3 3 0 1 0 4.24 4.24"/><path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"/><path d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"/><line x1="2" x2="22" y1="2" y2="22"/>
                                </svg>
                                Hide
                            </DropdownMenuItem>
                        </>
                    })}
                </DropdownMenuContent>
            </DropdownMenu>
        </div>
    }.into_any()
}
