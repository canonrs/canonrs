use leptos::prelude::*;

#[component]
pub fn LoadingSkeleton(
    #[prop(default = 10)] rows: usize,
    #[prop(default = 36.0)] row_height: f64,
) -> impl IntoView {
    view! {
        <div class="animate-pulse">
            {(0..rows).map(|_| {
                view! {
                    <div 
                        class="flex border-b border-border gap-4 px-[0.5rem]"
                        style=format!("height: {}px;", row_height)
                    >
                        <div class="flex-1 bg-muted rounded h-4 my-auto"></div>
                        <div class="w-24 bg-muted rounded h-4 my-auto"></div>
                        <div class="flex-[2] bg-muted rounded h-4 my-auto"></div>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
