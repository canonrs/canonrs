//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DropdownSide {
    Top,
    Bottom,
    Left,
    Right,
}

#[component]
pub fn DropdownPrimitive(children: Children) -> impl IntoView {
    let open = RwSignal::new(false);
    provide_context(open);

    view! {
        <div data-name="Dropdown" class="relative inline-block">
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownTriggerPrimitive(children: Children) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();

    view! {
        <button
            data-name="DropdownTrigger"
            type="button"
            on:click=move |_| open.update(|o| *o = !*o)
        >
            {children()}
        </button>
    }
}

#[component]
pub fn DropdownContentPrimitive(
    children: Children,
    #[prop(default = DropdownSide::Bottom)] side: DropdownSide,
    #[prop(default = 4)] side_offset: i32,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();

    let style_value = move || {
        let position = match side {
            DropdownSide::Bottom => format!("top: {}px;", side_offset),
            DropdownSide::Top => format!("bottom: {}px;", side_offset),
            DropdownSide::Left => format!("right: {}px;", side_offset),
            DropdownSide::Right => format!("left: {}px;", side_offset),
        };

        if open.get() {
            position
        } else {
            format!("{}; display: none;", position)
        }
    };

    view! {
        <>
            {move || open.get().then(|| view! {
                <div
                    class="fixed inset-0 z-40"
                    on:click=move |_| open.set(false)
                />
            })}

            <div
                data-name="DropdownContent"
                class="absolute z-50"
                style=style_value
            >
                {children()}
            </div>
        </>
    }
}

#[component]
pub fn DropdownItemPrimitive(
    children: Children,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] on_select: Option<Callback<()>>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();

    view! {
        <div
            data-name="DropdownItem"
            data-disabled=disabled
            role="menuitem"
            on:click=move |_| {
                if !disabled {
                    if let Some(callback) = on_select {
                        callback.run(());
                    }
                    open.set(false);
                }
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownCheckboxItemPrimitive(
    children: Children,
    #[prop(default = false)] checked: bool,
    #[prop(optional)] on_checked_change: Option<Callback<bool>>,
) -> impl IntoView {
    let is_checked = RwSignal::new(checked);

    view! {
        <div
            data-name="DropdownCheckboxItem"
            data-checked=move || is_checked.get()
            role="menuitemcheckbox"
            on:click=move |_| {
                let new_value = !is_checked.get();
                is_checked.set(new_value);
                if let Some(callback) = on_checked_change {
                    callback.run(new_value);
                }
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownLabelPrimitive(children: Children) -> impl IntoView {
    view! {
        <div data-name="DropdownLabel">
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownSeparatorPrimitive() -> impl IntoView {
    view! {
        <div data-name="DropdownSeparator" role="separator" />
    }
}
