//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct CommandContext {
    pub search: RwSignal<String>,
}

#[component]
pub fn CommandPrimitive(
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    let search = RwSignal::new(String::new());
    provide_context(CommandContext { search });
    view! { <div data-slot="command" class=class>{children()}</div> }
}

#[component]
pub fn CommandInputPrimitive(
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let ctx = expect_context::<CommandContext>();
    view! {
        <div data-slot="command-input-wrapper" class="flex items-center gap-2 border-b px-3">
            <input 
                type="text" 
                placeholder=placeholder 
                on:input=move |ev| ctx.search.set(event_target_value(&ev)) 
                data-slot="command-input" 
                class=class 
            />
        </div>
    }
}

#[component]
pub fn CommandListPrimitive(
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    view! { <div data-slot="command-list" class=class>{children()}</div> }
}

#[component]
pub fn CommandEmptyPrimitive(
    #[prop(default = String::new(), into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = expect_context::<CommandContext>();
    let classes = StoredValue::new(class);
    
    view! {
        <Show when=move || !ctx.search.get().is_empty()>
            <div data-slot="command-empty" class=classes.get_value()>
                {children()}
            </div>
        </Show>
    }
}

#[component]
pub fn CommandGroupPrimitive(
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    view! { <div data-slot="command-group" class=class>{children()}</div> }
}

#[component]
pub fn CommandItemPrimitive(
    #[prop(into)] value: String,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = Callback::new(|_| {}))] on_select: Callback<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = expect_context::<CommandContext>();
    let value_stored = StoredValue::new(value.clone());
    let classes = StoredValue::new(class);
    
    let is_visible = Memo::new(move |_| {
        let search_term = ctx.search.get().to_lowercase();
        if search_term.is_empty() {
            return true;
        }
        value_stored.get_value().to_lowercase().contains(&search_term)
    });
    
    view! {
        <Show when=move || is_visible.get()>
            <div 
                on:click=move |_| on_select.run(value_stored.get_value())
                data-slot="command-item"
                data-value=value_stored.get_value()
                class=classes.get_value()
            >
                {children()}
            </div>
        </Show>
    }
}
