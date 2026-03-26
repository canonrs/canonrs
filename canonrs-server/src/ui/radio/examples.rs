use leptos::prelude::*;
use super::Radio;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-3">
            <div class="font-medium text-sm">"Shipping Method"</div>
            <Radio name="shipping" value="standard" checked=true>
                "Standard (5-7 days)"
            </Radio>
            <Radio name="shipping" value="express">
                "Express (2-3 days)"
            </Radio>
            <Radio name="shipping" value="overnight">
                "Overnight"
            </Radio>
        </div>
    }
}
