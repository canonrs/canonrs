use leptos::prelude::*;
use super::calendar_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <Calendar
            year=2026
            month=2
            selected_date=CalendarDate::new(2026, 2, 15)
            today=CalendarDate::new(2026, 2, 2)
        />
    }
}
