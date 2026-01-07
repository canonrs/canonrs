use leptos::prelude::*;
use crate::providers::use_theme;
use crate::themes::ThemeRegistry;
use crate::ui::select::Select;

#[component]
pub fn ThemePicker(
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let theme = use_theme();
    let presets = ThemeRegistry::available_presets();
    
    view! {
        <Select
            value=theme.preset
            on_change=Callback::new(move |value: String| {
                theme.preset.set(value);
            })
            class=class
        >
            {presets.into_iter().map(|preset| {
                view! {
                    <option value=preset.id>
                        {preset.label}
                    </option>
                }
            }).collect::<Vec<_>>()}
        </Select>
    }
}
