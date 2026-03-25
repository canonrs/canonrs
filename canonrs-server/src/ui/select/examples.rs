use leptos::prelude::*;
use super::select_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Select>
            <SelectTrigger>
                <SelectValue placeholder="Select an option..." />
            </SelectTrigger>
            <SelectContent>
                <SelectItem value="option-1" selected=true>"Option 1"</SelectItem>
                <SelectItem value="option-2">"Option 2"</SelectItem>
                <SelectSeparator />
                <SelectItem value="option-3">"Option 3"</SelectItem>
            </SelectContent>
        </Select>
    }
}
