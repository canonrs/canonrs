//! Combobox Component
//!
//! @canon-level: loose
//! @canon-exceptions: [#24]
//! @canon-justification: Fixed dropdown height
//! @canon-owner: ui-team
//! @canon-target-date: 2025-03-01
//! @canon-migration-status: planned

use leptos::prelude::*;
use crate::ui::{Popover, PopoverTrigger, PopoverContent};

use super::types::{ComboboxOption, ComboboxSelectionMode};
use super::variants::{ComboboxSize, ComboboxValidation};

const TRIGGER_BASE: &str = "w-full inline-flex items-center justify-between rounded-[var(--radius-md)] border-[length:var(--border-width-thin)] bg-background font-family-[var(--font-family-sans)] font-[number:var(--font-weight-medium)] text-foreground line-height-[var(--line-height-normal)] shadow-[var(--shadow-sm)] transition-all duration-[var(--motion-duration-fast)] ease-[var(--motion-ease-standard)] focus:outline-none focus:ring-2 hover:opacity-[var(--state-hover-opacity)] active:opacity-[var(--state-active-opacity)] disabled:opacity-[var(--state-disabled-opacity)] disabled:cursor-not-allowed";
const POPOVER_CONTENT: &str = "w-[var(--radix-popover-trigger-width)] max-h-[300px] overflow-y-auto rounded-[var(--radius-md)] border-[length:var(--border-width-thin)] border-border bg-card shadow-[var(--shadow-md)] z-[var(--z-dropdown)] p-[var(--space-sm)]";
const SEARCH_INPUT: &str = "w-full h-[var(--field-height)] px-[var(--field-padding)] py-[var(--space-sm)] rounded-[var(--radius-sm)] border-[length:var(--border-width-thin)] border-[var(--color-border)] bg-background font-family-[var(--font-family-sans)] font-[number:var(--font-weight-regular)] text-[length:var(--font-size-sm)] text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-[color:var(--state-focus-ring)] mb-[var(--space-sm)]";
const LIST_ITEM_BASE: &str = "relative flex cursor-pointer select-none items-center h-[var(--list-item-height)] px-[var(--list-item-padding)] rounded-[var(--radius-sm)] font-family-[var(--font-family-sans)] font-[number:var(--font-weight-regular)] text-[length:var(--font-size-sm)] line-height-[var(--line-height-tight)] text-foreground transition-colors duration-[var(--motion-duration-fast)] hover:bg-muted hover:opacity-[var(--state-hover-opacity)] active:opacity-[var(--state-active-opacity)] focus:outline-none focus:bg-muted data-[selected=true]:bg-primary data-[selected=true]:text-primary-foreground data-[disabled=true]:opacity-[var(--state-disabled-opacity)] data-[disabled=true]:cursor-not-allowed data-[disabled=true]:pointer-events-none";
const CHECK_ICON: &str = "ml-auto size-4 text-primary-foreground";
const EMPTY_STATE: &str = "py-[var(--space-lg)] text-center font-family-[var(--font-family-sans)] text-[length:var(--font-size-sm)] text-muted-foreground line-height-[var(--line-height-relaxed)]";

#[component]
fn ComboboxItem(
    option: ComboboxOption,
    selected_value: RwSignal<String>,
    on_select: Callback<String>,
    size: ComboboxSize,
) -> impl IntoView {
    let value = option.value.clone();
    let label = option.label.clone();
    let disabled = option.disabled;
    let value_for_click = value.clone();
    let value_for_aria = value.clone();
    let value_for_check = value.clone();
    
    view! {
        <div
            on:click=move |_| {
                if !disabled {
                    on_select.run(value_for_click.clone());
                }
            }
            class=format!("{} {}", LIST_ITEM_BASE, size.list_item_classes())
            data-selected=move || selected_value.get() == value
            data-disabled=disabled
            role="option"
            aria-selected=move || selected_value.get() == value_for_aria
            aria-disabled=disabled
        >
            <span>{label}</span>
            <Show when=move || selected_value.get() == value_for_check>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class=CHECK_ICON>
                    <path d="M20 6 9 17l-5-5"/>
                </svg>
            </Show>
        </div>
    }
}

#[component]
pub fn Combobox(
    options: Vec<ComboboxOption>,
    value: RwSignal<String>,
    #[prop(optional)] size: Option<ComboboxSize>,
    #[prop(default = ComboboxValidation::None)] validation: ComboboxValidation,
    #[prop(default = ComboboxSelectionMode::Single)] mode: ComboboxSelectionMode,
    #[prop(default = false)] disabled: bool,
    #[prop(default = "Select option...".into())] placeholder: String,
    #[prop(default = "Search...".into())] search_placeholder: String,
    #[prop(default = "No results found.".into())] empty_message: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let size = size.unwrap_or_default();
    let is_open = RwSignal::new(false);
    let search_query = RwSignal::new(String::new());
    
    let search_ph = StoredValue::new(search_placeholder);
    let empty_msg = StoredValue::new(empty_message);
    
    let opts = StoredValue::new(options);
    let filtered_options = Memo::new(move |_| {
        let query = search_query.get().to_lowercase();
        if query.is_empty() {
            opts.get_value()
        } else {
            opts.get_value().iter().filter(|opt| opt.label.to_lowercase().contains(&query)).cloned().collect()
        }
    });
    
    let selected_label = Memo::new(move |_| {
        opts.get_value().iter().find(|opt| opt.value == value.get()).map(|opt| opt.label.clone()).unwrap_or_else(|| placeholder.clone())
    });
    
    let on_select = Callback::new(move |val: String| {
        value.set(val);
        is_open.set(false);
        search_query.set(String::new());
    });

    view! {
        <Popover open=is_open.get()>
            <PopoverTrigger class=format!("{} {} {} {} {}", TRIGGER_BASE, size.trigger_classes(), validation.border_classes(), validation.ring_classes(), class)>
                <span>{move || selected_label.get()}</span>
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="ml-2 size-4 shrink-0 opacity-50">
                    <path d="m6 9 6 6 6-6"/>
                </svg>
            </PopoverTrigger>
            
            <PopoverContent class=POPOVER_CONTENT align="start">
                <input type="text" placeholder=search_ph.get_value() value=move || search_query.get() on:input=move |ev| search_query.set(event_target_value(&ev)) class=SEARCH_INPUT aria-label="Search" />
                
                <div role="listbox" aria-label="Options">
                    {move || {
                        let opts = filtered_options.get();
                        if opts.is_empty() {
                            view! { <div class=EMPTY_STATE>{empty_msg.get_value()}</div> }.into_any()
                        } else {
                            opts.into_iter().map(|opt| view! { <ComboboxItem option=opt selected_value=value on_select=on_select size=size /> }).collect::<Vec<_>>().into_any()
                        }
                    }}
                </div>
            </PopoverContent>
        </Popover>
    }
}
