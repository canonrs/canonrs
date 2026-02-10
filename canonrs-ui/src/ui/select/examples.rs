use leptos::prelude::*;
use super::select_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div>
            <Select>
                <SelectTrigger controls_id="select-content-ex" value_text="Select an option">
                    <SelectValue placeholder="Select an option" />
                </SelectTrigger>
                <SelectContent open=false content_id="select-content-ex">
                    <SelectItem value="option-1" selected=true>"Option 1"</SelectItem>
                    <SelectItem value="option-2">"Option 2"</SelectItem>
                    <SelectSeparator />
                    <SelectItem value="option-3">"Option 3"</SelectItem>
                </SelectContent>
            </Select>
        </div>
    }
}
