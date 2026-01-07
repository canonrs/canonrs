//! Data Table Row Actions
//!
//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Legacy table component
//! @canon-owner: data-team
//! @canon-target-date: 2025-03-15
//! @canon-migration-status: planned

/// Row Actions Dropdown - Canon Rule #8
use leptos::prelude::*;
use crate::ui::*;

#[derive(Clone)]
pub struct RowAction {
    pub label: String,
    pub on_click: Callback<()>,
    pub variant: RowActionVariant,
}

#[derive(Clone, Copy, PartialEq)]
pub enum RowActionVariant {
    Default,
    Destructive,
}

#[component]
fn RowActionItem(
    label: String,
    variant: RowActionVariant,
    on_select: Callback<()>,
) -> impl IntoView {
    let label = StoredValue::new(label);
    
    view! {
        <DropdownMenuItem
            class=if variant == RowActionVariant::Destructive {
                "text-destructive"
            } else { "" }
            on_select=on_select
        >
            {move || label.get_value()}
        </DropdownMenuItem>
    }
}

#[component]
pub fn DataTableRowActions(actions: Vec<RowAction>) -> impl IntoView {
    let actions = StoredValue::new(actions);
    let is_browser = RwSignal::new(false);
    
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        if web_sys::window().is_some() {
            is_browser.set(true);
        }
    });

    view! {
        <Show
            when=move || is_browser.get()
            fallback=|| view! { <div class="w-8 h-8"></div> }
        >
            <DropdownMenu>
                <DropdownMenuTrigger class="">
                    {|| view! {
                        <IconButton>
                            <IconMoreHorizontal />
                        </IconButton>
                    }}
                </DropdownMenuTrigger>
                <DropdownMenuContent class="w-[160px]">
                    <DropdownMenuLabel>"Actions"</DropdownMenuLabel>
                    <DropdownMenuSeparator />
                    <For
                        each=move || actions.get_value()
                        key=|a| a.label.clone()
                        children=move |action| {
                            view! {
                                <RowActionItem
                                    label=action.label
                                    variant=action.variant
                                    on_select=action.on_click
                                />
                            }
                        }
                    />
                </DropdownMenuContent>
            </DropdownMenu>
        </Show>
    }
}
