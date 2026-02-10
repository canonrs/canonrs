use leptos::prelude::*;
use super::CalendarInteractive;
use time::Date;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <CalendarInteractive />
    }
}

#[component]
pub fn WithInitialDate() -> impl IntoView {
    let initial = Date::from_calendar_date(2026, time::Month::December, 25).unwrap();
    
    view! {
        <CalendarInteractive initial_date=initial />
    }
}
