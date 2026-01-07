use leptos::prelude::*;
use rs_design::{Button, ButtonVariant, Input};

#[component]
pub fn ThemeDemo() -> impl IntoView {
    let input_value = RwSignal::new(String::new());
    
    view! {
        <div class="p-8 space-y-8">
            <div>
                <h2 class="text-2xl font-bold mb-2">"Theme Testing"</h2>
                <p class="text-muted-foreground">"All components update in real-time when you change themes"</p>
            </div>
            
            {/* Buttons */}
            <div class="space-y-4">
                <h3 class="text-lg font-semibold">"Buttons"</h3>
                <div class="flex gap-3 flex-wrap">
                    <Button variant=ButtonVariant::Solid>"Primary"</Button>
                    <Button variant=ButtonVariant::Outline>"Outline"</Button>
                    <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
                    <Button variant=ButtonVariant::Danger>"Danger"</Button>
                    <Button variant=ButtonVariant::Warning>"Warning"</Button>
                    <Button variant=ButtonVariant::Success>"Success"</Button>
                    <Button variant=ButtonVariant::Muted>"Muted"</Button>
                </div>
            </div>
            
            {/* Inputs */}
            <div class="space-y-4">
                <h3 class="text-lg font-semibold">"Inputs"</h3>
                <div class="space-y-3 max-w-md">
                    <Input placeholder="Type something..." value=input_value />
                    <Input placeholder="Disabled input" value=input_value disabled=true />
                </div>
            </div>
            
            {/* Color Swatches */}
            <div class="space-y-4">
                <h3 class="text-lg font-semibold">"Color Palette"</h3>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                    <ColorSwatch name="Primary" class="bg-primary" />
                    <ColorSwatch name="Secondary" class="bg-secondary" />
                    <ColorSwatch name="Destructive" class="bg-destructive" />
                    <ColorSwatch name="Success" class="bg-success" />
                    <ColorSwatch name="Warning" class="bg-warning" />
                    <ColorSwatch name="Muted" class="bg-muted" />
                    <ColorSwatch name="Accent" class="bg-accent" />
                    <ColorSwatch name="Card" class="bg-card border" />
                </div>
            </div>
            
            {/* Instructions */}
            <div class="mt-8 p-4 bg-muted rounded-lg">
                <p class="text-sm text-muted-foreground">
                    <strong>"💡 How to test: "</strong>
                    "Use the Theme Settings (⚙️) in the top-right corner to switch between themes, modes, and density. Watch all colors update instantly!"
                </p>
            </div>
        </div>
    }
}

#[component]
fn ColorSwatch(name: &'static str, class: &'static str) -> impl IntoView {
    view! {
        <div class="space-y-2">
            <div class="text-xs text-muted-foreground font-medium">{name}</div>
            <div class=format!("w-full h-16 rounded-md {}", class)></div>
        </div>
    }
}
