use leptos::prelude::*;
use super::ColorPicker;

#[component]
pub fn BasicExample() -> impl IntoView {
    let (color, set_color) = signal("#3b82f6".to_string());
    
    view! {
        <div data-color-picker-example="">
            <ColorPicker 
                value=color.into()
                on_change=move |new_color: String| set_color.set(new_color)
            />
            <ColorLabels color=color.into() />
        </div>
    }
}

#[component]
fn ColorLabels(color: Signal<String>) -> impl IntoView {
    view! {
        <div data-color-labels="">
            <div data-color-label="">
                <span data-color-label-key="">"HEX: "</span>
                <span data-color-label-value="">{move || color.get()}</span>
            </div>
            <div data-color-label="">
                <span data-color-label-key="">"RGB: "</span>
                <span data-color-label-value="">{move || hex_to_rgb(&color.get())}</span>
            </div>
            <div data-color-label="">
                <span data-color-label-key="">"HSL: "</span>
                <span data-color-label-value="">{move || hex_to_hsl(&color.get())}</span>
            </div>
            <div data-color-label="">
                <span data-color-label-key="">"CMYK: "</span>
                <span data-color-label-value="">{move || hex_to_cmyk(&color.get())}</span>
            </div>
        </div>
    }
}

fn hex_to_rgb(hex: &str) -> String {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    format!("rgb({}, {}, {})", r, g, b)
}

fn hex_to_hsl(hex: &str) -> String {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
    
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    
    if max == min {
        return format!("hsl(0, 0%, {}%)", (l * 100.0) as u8);
    }
    
    let d = max - min;
    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
    
    let h = if max == r {
        (g - b) / d + if g < b { 6.0 } else { 0.0 }
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    } * 60.0;
    
    format!("hsl({}, {}%, {}%)", h as u8, (s * 100.0) as u8, (l * 100.0) as u8)
}

fn hex_to_cmyk(hex: &str) -> String {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
    
    let k = 1.0 - r.max(g).max(b);
    if k == 1.0 {
        return "cmyk(0%, 0%, 0%, 100%)".to_string();
    }
    
    let c = (1.0 - r - k) / (1.0 - k);
    let m = (1.0 - g - k) / (1.0 - k);
    let y = (1.0 - b - k) / (1.0 - k);
    
    format!("cmyk({}%, {}%, {}%, {}%)", 
        (c * 100.0) as u8, 
        (m * 100.0) as u8, 
        (y * 100.0) as u8, 
        (k * 100.0) as u8
    )
}
