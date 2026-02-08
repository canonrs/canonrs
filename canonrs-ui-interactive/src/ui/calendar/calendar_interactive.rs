use leptos::prelude::*;
use leptos::ev;
use leptos::callback::Callback;
use canonrs_ui::ui::calendar::{Calendar, CalendarDate};

#[component]
pub fn CalendarInteractive(
    #[prop(default = 2026)] initial_year: i32,
    #[prop(default = 2)] initial_month: u32,
    on_date_select: Callback<CalendarDate>,
    #[prop(optional)] selected_date: Option<CalendarDate>,
    #[prop(optional)] today: Option<CalendarDate>,
    #[prop(optional)] disabled_dates: Option<Vec<CalendarDate>>,
    #[prop(optional)] min_date: Option<CalendarDate>,
    #[prop(optional)] max_date: Option<CalendarDate>,
) -> impl IntoView {
    let year = create_rw_signal(initial_year);
    let month = create_rw_signal(initial_month);
    let selected = StoredValue::new(selected_date);
    let today_val = StoredValue::new(today);
    let disabled = StoredValue::new(disabled_dates);
    let min = StoredValue::new(min_date);
    let max = StoredValue::new(max_date);
    
    let handle_prev = move |_: ev::MouseEvent| {
        if month.get() == 1 {
            year.update(|y| *y -= 1);
            month.set(12);
        } else {
            month.update(|m| *m -= 1);
        }
    };
    
    let handle_next = move |_: ev::MouseEvent| {
        if month.get() == 12 {
            year.update(|y| *y += 1);
            month.set(1);
        } else {
            month.update(|m| *m += 1);
        }
    };

    view! {
        <div data-calendar-interactive>
            <Calendar
                year=year.get()
                month=month.get()
                selected_date=selected.get_value()
                today=today_val.get_value()
                disabled_dates=disabled.get_value()
                min_date=min.get_value()
                max_date=max.get_value()
            />
        </div>
    }
}
