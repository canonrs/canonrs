use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum ColorFormat {
    #[default]
    Hex,
    Rgb,
    Hsl,
    Cmyk,
}

impl ColorFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Hex  => "hex",
            Self::Rgb  => "rgb",
            Self::Hsl  => "hsl",
            Self::Cmyk => "cmyk",
        }
    }
}

fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let h = hex.trim_start_matches('#');
    if h.len() != 6 { return None; }
    let r = u8::from_str_radix(&h[0..2], 16).ok()?;
    let g = u8::from_str_radix(&h[2..4], 16).ok()?;
    let b = u8::from_str_radix(&h[4..6], 16).ok()?;
    Some((r, g, b))
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let r = r as f64 / 255.0; let g = g as f64 / 255.0; let b = b as f64 / 255.0;
    let max = r.max(g).max(b); let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    if (max - min).abs() < 1e-10 { return (0.0, 0.0, l * 100.0); }
    let d = max - min;
    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
    let h = if max == r { ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0 }
            else if max == g { ((b - r) / d + 2.0) / 6.0 }
            else { ((r - g) / d + 4.0) / 6.0 };
    (h * 360.0, s * 100.0, l * 100.0)
}

fn rgb_to_cmyk(r: u8, g: u8, b: u8) -> (f64, f64, f64, f64) {
    let r = r as f64 / 255.0; let g = g as f64 / 255.0; let b = b as f64 / 255.0;
    let k = 1.0 - r.max(g).max(b);
    if (k - 1.0).abs() < 1e-10 { return (0.0, 0.0, 0.0, 100.0); }
    let c = (1.0 - r - k) / (1.0 - k);
    let m = (1.0 - g - k) / (1.0 - k);
    let y = (1.0 - b - k) / (1.0 - k);
    (c * 100.0, m * 100.0, y * 100.0, k * 100.0)
}

fn format_color(hex: &str, format: ColorFormat) -> String {
    match hex_to_rgb(hex) {
        None => hex.to_string(),
        Some((r, g, b)) => match format {
            ColorFormat::Hex  => hex.to_string(),
            ColorFormat::Rgb  => format!("rgb({}, {}, {})", r, g, b),
            ColorFormat::Hsl  => { let (h,s,l) = rgb_to_hsl(r,g,b); format!("hsl({:.0}, {:.0}%, {:.0}%)", h, s, l) }
            ColorFormat::Cmyk => { let (c,m,y,k) = rgb_to_cmyk(r,g,b); format!("cmyk({:.0}%, {:.0}%, {:.0}%, {:.0}%)", c, m, y, k) }
        }
    }
}

#[island]
pub fn ColorPickerIsland(
    #[prop(into)] value: String,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] format: Option<ColorFormat>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);
    let format   = format.unwrap_or_default();
    let name     = name.unwrap_or_default();
    let class    = class.unwrap_or_default();

    let (color, set_color) = signal(value.clone());

    let initial_state = if disabled { "disabled".to_string() } else { "closed".to_string() };

    let display_value = move || format_color(&color.get(), format);

    let swatch_style = move || format!("background-color:{}", color.get());

    #[cfg(feature = "hydrate")]
    let on_input = move |e: leptos::ev::Event| {
        use leptos::wasm_bindgen::JsCast;
        if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
            set_color.set(input.value());
        }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_input = move |_: leptos::ev::Event| { let _ = set_color; };

    view! {
        <div
            data-rs-color-picker=""
            data-rs-component="ColorPicker"
            data-rs-state=move || { let s = if disabled { "disabled".to_string() } else { "closed".to_string() }; if s.is_empty() { initial_state.clone() } else { s } }
            aria-disabled=disabled.to_string()
            class=class
        >
            <button
                type="button"
                data-rs-color-picker-trigger=""
                aria-haspopup="dialog"
                aria-label="Open color picker"
                aria-disabled=disabled.to_string()
                disabled=disabled
            >
                <div
                    data-rs-color-swatch=""
                    data-rs-color=move || color.get()
                    style=swatch_style
                />
            </button>
            <input
                type="color"
                data-rs-color-picker-input=""
                value=move || color.get()
                name=name
                aria-label="Color picker"
                aria-disabled=disabled.to_string()
                disabled=disabled
                on:input=on_input
            />
            <div data-rs-color-picker-display="" data-rs-format=format.as_str()>
                <span data-rs-color-display-format="">{format.as_str().to_uppercase()}</span>
                <span data-rs-color-display-value="">{display_value}</span>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct SwatchColor {
    pub color: String,
    pub label: String,
}

#[island]
pub fn ColorSwatchGroupIsland(
    swatches: Vec<SwatchColor>,
    #[prop(into)] selected: String,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);
    let class    = class.unwrap_or_default();
    let (selected_color, set_selected) = signal(selected.clone());

    let initial_state = selected.clone();

    let swatches_view = swatches.into_iter().map(|swatch| {
        let color = swatch.color.clone();
        let label = swatch.label.clone();
        let on_click = {
            let color = color.clone();
            move |_: leptos::ev::MouseEvent| {
                if !disabled { set_selected.set(color.clone()); }
            }
        };
        let swatch_style = format!("background-color:{}", color);
        let item_state = {
            let color = color.clone();
            move || if selected_color.get() == color { "selected".to_string() } else { String::new() }
        };
        view! {
            <button
                type="button"
                data-rs-color-swatch=""
                data-rs-color=color.clone()
                data-rs-state=item_state
                aria-label=label
                aria-selected=move || (selected_color.get() == color).to_string()
                aria-disabled=disabled.to_string()
                disabled=disabled
                style=swatch_style
                on:click=on_click
            />
        }
    }).collect::<Vec<_>>();

    view! {
        <div
            data-rs-color-swatch-group=""
            data-rs-component="ColorSwatchGroup"
            data-rs-state=move || { let s = selected_color.get(); if s.is_empty() { initial_state.clone() } else { s } }
            class=class
        >
            {swatches_view}
        </div>
    }
}
