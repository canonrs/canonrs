use crate::primitives::color_picker::ColorPickerPrimitive;
use crate::ui::Label;
use leptos::prelude::*;

#[component]
pub fn ColorPicker(
    #[prop(into)] label: String,
    #[prop(into)] value: RwSignal<String>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let id = format!("color-picker-{}", label.to_lowercase().replace(" ", "-"));

    view! {
        <div class=format!("flex items-center justify-between gap-2 {}", class)>
            <div class="flex-1">
                <Label html_for=id.clone() class="text-sm font-medium">{label.clone()}</Label>
                <div class="text-xs text-muted-foreground font-mono">
                    {move || value.get()}
                </div>
            </div>
            <ColorPickerPrimitive
                value=value
                class="size-10 cursor-pointer rounded border border-border bg-transparent flex-shrink-0"
            />
        </div>
    }
}
