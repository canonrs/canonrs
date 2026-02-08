use leptos::prelude::*;
use super::select_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <div>
            <Select>
                <SelectTrigger controls_id="select-content-ex".to_string() value_text="Select an option".to_string()>
                    <SelectValue placeholder="Select an option".to_string() />
                </SelectTrigger>
                <SelectContent open=false content_id="select-content-ex".to_string()>
                    <SelectItem value="option-1".to_string() selected=true>"Option 1"</SelectItem>
                    <SelectItem value="option-2".to_string()>"Option 2"</SelectItem>
                    <SelectSeparator />
                    <SelectItem value="option-3".to_string()>"Option 3"</SelectItem>
                </SelectContent>
            </Select>
        </div>
    }
}
