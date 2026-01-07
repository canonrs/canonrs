use leptos::prelude::*;
use crate::providers::{use_theme, use_density, DensityMode};
use crate::themes::ThemeRegistry;
use crate::ui::button::{Button, ButtonVariant, ButtonSize};
use super::icons::{SunIcon, MoonIcon, LaptopIcon};

#[component]
pub fn ThemeSettingsDropdown(
    #[prop(into, optional)] class: String,
    /// Callback to save theme (app provides server function)
    #[prop(optional)]
    on_theme_change: Option<Callback<(String, String)>>,
    /// Callback to save density (app provides server function)
    #[prop(optional)]
    on_density_change: Option<Callback<String>>,
) -> impl IntoView {
    let theme = use_theme();
    let density = use_density();
    let is_open = RwSignal::new(false);
    let presets: Vec<_> = ThemeRegistry::available_presets();
    
    let toggle_dropdown = move |_| {
        is_open.update(|open| *open = !*open);
    };
    
    let set_mode = move |new_mode| {
        theme.mode.set(new_mode);
        
        if let Some(callback) = on_theme_change {
            let mode_str = match new_mode {
                crate::providers::ThemeMode::Light => "light",
                crate::providers::ThemeMode::Dark => "dark",
                crate::providers::ThemeMode::System => "system",
            };
            callback.run((mode_str.to_string(), theme.preset.get()));
        }
    };
    
    let set_preset = move |preset_id: String| {
        theme.preset.set(preset_id.clone());
        
        if let Some(callback) = on_theme_change {
            let mode_str = match theme.mode.get() {
                crate::providers::ThemeMode::Light => "light",
                crate::providers::ThemeMode::Dark => "dark",
                crate::providers::ThemeMode::System => "system",
            };
            callback.run((mode_str.to_string(), preset_id));
        }
    };
    
    let set_density = move |new_density: DensityMode| {
        density.mode.set(new_density);
        
        if let Some(callback) = on_density_change {
            callback.run(new_density.as_str().to_string());
        }
    };
    
    view! {
        <div class=format!("relative {}", class)>
            <Button
                variant=ButtonVariant::Ghost
                size=ButtonSize::Icon
                on:click=toggle_dropdown
            >
                {move || {
                    match theme.mode.get() {
                        crate::providers::ThemeMode::Light => view! { <SunIcon /> }.into_any(),
                        crate::providers::ThemeMode::Dark => view! { <MoonIcon /> }.into_any(),
                        crate::providers::ThemeMode::System => view! { <LaptopIcon /> }.into_any(),
                    }
                }}
            </Button>
            
            <Show when=move || is_open.get()>
                <div class="absolute right-0 mt-2 w-64 rounded-md border border-border bg-popover p-4 shadow-lg z-dropdown">
                    <div class="space-y-4">
                        // Mode selector
                        <div class="space-y-2">
                            <label class="text-sm font-medium">"Mode"</label>
                            <div class="grid grid-cols-3 gap-2">
                                <button
                                    class="px-3 py-2 text-sm rounded border hover:bg-accent"
                                    on:click=move |_| set_mode(crate::providers::ThemeMode::Light)
                                >
                                    <SunIcon />
                                    <span class="ml-2">"Light"</span>
                                </button>
                                <button
                                    class="px-3 py-2 text-sm rounded border hover:bg-accent"
                                    on:click=move |_| set_mode(crate::providers::ThemeMode::Dark)
                                >
                                    <MoonIcon />
                                    <span class="ml-2">"Dark"</span>
                                </button>
                                <button
                                    class="px-3 py-2 text-sm rounded border hover:bg-accent"
                                    on:click=move |_| set_mode(crate::providers::ThemeMode::System)
                                >
                                    <LaptopIcon />
                                    <span class="ml-2">"System"</span>
                                </button>
                            </div>
                        </div>
                        
                        // Preset selector
                        <div class="space-y-2">
                            <label class="text-sm font-medium">"Theme"</label>
                            <select
                                class="w-full rounded-md border border-input bg-background px-3 py-2"
                                on:change=move |ev| {
                                    set_preset(event_target_value(&ev));
                                }
                            >
                                {presets.iter().map(|p| {
                                    let id = p.id.to_string();
                                    let label = p.label.to_string();
                                    view! {
                                        <option value=id>{label}</option>
                                    }
                                }).collect::<Vec<_>>()}
                            </select>
                        </div>
                        
                        // Density selector
                        <div class="space-y-2">
                            <label class="text-sm font-medium">"Density"</label>
                            <div class="grid grid-cols-3 gap-2">
                                <button
                                    class="px-2 py-2 text-xs rounded border hover:bg-accent"
                                    on:click=move |_| set_density(DensityMode::Compact)
                                >
                                    "Compact"
                                </button>
                                <button
                                    class="px-2 py-2 text-xs rounded border hover:bg-accent"
                                    on:click=move |_| set_density(DensityMode::Comfortable)
                                >
                                    "Comfortable"
                                </button>
                                <button
                                    class="px-2 py-2 text-xs rounded border hover:bg-accent"
                                    on:click=move |_| set_density(DensityMode::Spacious)
                                >
                                    "Spacious"
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}
