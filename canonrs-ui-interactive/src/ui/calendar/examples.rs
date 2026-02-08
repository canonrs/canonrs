use leptos::prelude::*;
use super::CalendarInteractive;
use canonrs_ui::ui::calendar::CalendarDate;

pub fn basic_example() -> impl IntoView {
    let handle_select = Callback::new(|date: CalendarDate| {
        leptos::logging::log!("Selected date: {}", date.to_string());
    });
    
    view! {
        <CalendarInteractive
            initial_year=2026
            initial_month=2
            on_date_select=handle_select
            today=CalendarDate::new(2026, 2, 6)
        />
    }
}
