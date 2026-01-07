use leptos::prelude::*;
use crate::providers::{use_theme, ThemeMode, ThemeContext};
use super::icons::{SunIcon, MoonIcon, LaptopIcon};
use super::types::{ThemeToggleVariant, ToggleSize};

#[component]
pub fn ThemeToggle(
    #[prop(optional)] variant: ThemeToggleVariant,
    #[prop(optional)] size: ToggleSize,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let theme = use_theme();
    
    // Ciclo: Light → Dark → System → Light
    let next_mode = move || match theme.mode.get() {
        ThemeMode::Light => ThemeMode::Dark,
        ThemeMode::Dark => ThemeMode::System,
        ThemeMode::System => ThemeMode::Light,
    };
    
    let toggle_theme = move |_| {
        theme.mode.set(next_mode());
    };
    
    match variant {
        ThemeToggleVariant::Icon => {
            view! {
                <ThemeToggleIcon 
                    theme=theme 
                    on_click=toggle_theme
                    size=size
                    class=class
                />
            }.into_any()
        }
        ThemeToggleVariant::Button => {
            view! {
                <ThemeToggleButton 
                    theme=theme 
                    on_click=toggle_theme
                    size=size
                    class=class
                />
            }.into_any()
        }
        ThemeToggleVariant::Dropdown => {
            view! {
                <ThemeToggleDropdown 
                    theme=theme
                    size=size
                    class=class
                />
            }.into_any()
        }
    }
}

// Icon variant (default - para header/navbar)
#[component]
fn ThemeToggleIcon(
    theme: crate::providers::ThemeContext,
    on_click: impl Fn(leptos::ev::MouseEvent) + 'static,
    size: ToggleSize,
    class: String,
) -> impl IntoView {
    let size_class = match size {
        ToggleSize::Sm => "p-1.5",
        ToggleSize::Md => "p-2",
        ToggleSize::Lg => "p-2.5",
    };
    
    let icon = move || match theme.mode.get() {
        ThemeMode::Light => view! { <SunIcon /> }.into_any(),
        ThemeMode::Dark => view! { <MoonIcon /> }.into_any(),
        ThemeMode::System => view! { <LaptopIcon /> }.into_any(),
    };
    
    let label = move || match theme.mode.get() {
        ThemeMode::Light => "Switch to dark mode",
        ThemeMode::Dark => "Switch to system mode",
        ThemeMode::System => "Switch to light mode",
    };
    
    view! {
        <button
            type="button"
            class=format!(
                "inline-flex items-center justify-center rounded-md {} \
                 text-fg-default hover:bg-muted transition-colors \
                 focus:outline-none focus:ring-2 focus:ring-primary-border {}",
                size_class, class
            )
            on:click=on_click
            aria-label=label
        >
            {icon}
        </button>
    }
}

// Button variant (para settings/preferences)
#[component]
fn ThemeToggleButton(
    theme: crate::providers::ThemeContext,
    on_click: impl Fn(leptos::ev::MouseEvent) + 'static,
    size: ToggleSize,
    class: String,
) -> impl IntoView {
    let size_class = match size {
        ToggleSize::Sm => "px-2 py-1 text-xs",
        ToggleSize::Md => "px-3 py-2 text-sm",
        ToggleSize::Lg => "px-4 py-2.5 text-base",
    };
    
    let label = move || match theme.mode.get() {
        ThemeMode::Light => "Light",
        ThemeMode::Dark => "Dark",
        ThemeMode::System => "System",
    };
    
    view! {
        <button
            type="button"
            class=format!(
                "inline-flex items-center justify-center gap-2 rounded-md {} \
                 font-medium border border-border bg-surface text-fg-default \
                 hover:bg-muted transition-colors \
                 focus:outline-none focus:ring-2 focus:ring-primary-border {}",
                size_class, class
            )
            on:click=on_click
        >
            {move || match theme.mode.get() {
                ThemeMode::Light => view! { <SunIcon /> }.into_any(),
                ThemeMode::Dark => view! { <MoonIcon /> }.into_any(),
                ThemeMode::System => view! { <LaptopIcon /> }.into_any(),
            }}
            <span>{label}</span>
        </button>
    }
}

// Dropdown variant (para configurações detalhadas)
#[component]
fn ThemeToggleDropdown(
    theme: crate::providers::ThemeContext,
    size: ToggleSize,
    class: String,
) -> impl IntoView {
    let size_class = match size {
        ToggleSize::Sm => "px-2 py-1 text-xs",
        ToggleSize::Md => "px-3 py-2 text-sm",
        ToggleSize::Lg => "px-4 py-2.5 text-base",
    };
    
    view! {
        <div class=format!("relative inline-block {}", class)>
            <select
                class=format!(
                    "inline-flex items-center gap-2 rounded-md {} \
                     font-medium border border-border bg-surface text-fg-default \
                     hover:bg-muted transition-colors cursor-pointer \
                     focus:outline-none focus:ring-2 focus:ring-primary-border",
                    size_class
                )
                on:change=move |ev| {
                    let value = event_target_value(&ev);
                    theme.mode.set(match value.as_str() {
                        "dark" => ThemeMode::Dark,
                        "system" => ThemeMode::System,
                        _ => ThemeMode::Light,
                    });
                }
                prop:value=move || match theme.mode.get() {
                    ThemeMode::Light => "light",
                    ThemeMode::Dark => "dark",
                    ThemeMode::System => "system",
                }
            >
                <option value="light">"Light"</option>
                <option value="dark">"Dark"</option>
                <option value="system">"System"</option>
            </select>
        </div>
    }
}
