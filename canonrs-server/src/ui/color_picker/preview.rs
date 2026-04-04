use leptos::prelude::*;
use super::color_picker_island::{ColorPickerIsland, ColorSwatchGroupIsland, SwatchColor, ColorFormat};

#[component]
pub fn ColorPickerShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ColorPickerIsland value="#3b82f6" />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Color input with swatch preview — state governed by data-rs-state."
            </p>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Format Display"</span>
                <div data-rs-showcase-preview-row="">
                    <ColorPickerIsland value="#3b82f6" format=ColorFormat::Hex />
                    <ColorPickerIsland value="#3b82f6" format=ColorFormat::Rgb />
                    <ColorPickerIsland value="#3b82f6" format=ColorFormat::Hsl />
                    <ColorPickerIsland value="#3b82f6" format=ColorFormat::Cmyk />
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Swatches"</span>
                <div data-rs-showcase-preview-row="">
                    <ColorSwatchGroupIsland
                        selected="#3b82f6"
                        swatches=vec![
                            SwatchColor { color: "#3b82f6".into(), label: "Blue".into() },
                            SwatchColor { color: "#ef4444".into(), label: "Red".into() },
                            SwatchColor { color: "#22c55e".into(), label: "Green".into() },
                            SwatchColor { color: "#f59e0b".into(), label: "Amber".into() },
                            SwatchColor { color: "#8b5cf6".into(), label: "Purple".into() },
                        ]
                    />
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <div data-rs-showcase-preview-row="">
                    <ColorPickerIsland value="#6b7280" disabled=true />
                </div>
            </div>

        </div>
    }
}
