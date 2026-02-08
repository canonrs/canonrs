use leptos::prelude::*;
use super::aspect_ratio_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <AspectRatio>
            <div style="width: 100%; height: 100%; background: #e5e7eb; display: flex; align-items: center; justify-content: center;">
                "16:9 Content"
            </div>
        </AspectRatio>
    }
}
