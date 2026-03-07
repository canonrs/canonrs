use leptos::prelude::*;
use time::{Date, Month};

#[component]
pub fn CalendarInteractive(
    #[prop(optional)] initial_date: Option<Date>,
) -> impl IntoView {
    let today = get_browser_date();
    let initial = initial_date.unwrap_or(today);
    
    let year = RwSignal::new(initial.year());
    let month = RwSignal::new(initial.month() as u8);
    let selected = RwSignal::new(Some(initial));
    
    let prev_month = move |_| {
        let current_month = month.get();
        if current_month == 1 {
            month.set(12);
            year.set(year.get() - 1);
        } else {
            month.set(current_month - 1);
        }
    };
    
    let next_month = move |_| {
        let current_month = month.get();
        if current_month == 12 {
            month.set(1);
            year.set(year.get() + 1);
        } else {
            month.set(current_month + 1);
        }
    };
    
    view! {
        <div data-calendar>
            <div data-calendar-header>
                <button data-calendar-nav-button on:click=prev_month>"←"</button>
                <div data-calendar-month-year>
                    {move || format!("{} {}", format_month(month.get()), year.get())}
                </div>
                <button data-calendar-nav-button on:click=next_month>"→"</button>
            </div>
            
            <div data-calendar-grid>
                <div data-calendar-weekday>"S"</div>
                <div data-calendar-weekday>"M"</div>
                <div data-calendar-weekday>"T"</div>
                <div data-calendar-weekday>"W"</div>
                <div data-calendar-weekday>"T"</div>
                <div data-calendar-weekday>"F"</div>
                <div data-calendar-weekday>"S"</div>
                
                {move || {
                    let days = generate_calendar_days(year.get(), month.get());
                    let today_date = today;
                    
                    days.into_iter().map(|day| {
                        view! {
                            <button
                                data-calendar-day
                                class:is-selected=move || {
                                    if let Some(d) = day {
                                        if let Some(s) = selected.get() {
                                            return d == s;
                                        }
                                    }
                                    false
                                }
                                class:is-today=move || {
                                    day.map(|d| d == today_date).unwrap_or(false)
                                }
                                disabled=day.is_none()
                                on:click=move |_| {
                                    if let Some(d) = day {
                                        selected.set(Some(d));
                                    }
                                }
                            >
                                {day.map(|d| d.day().to_string()).unwrap_or_default()}
                            </button>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
            
            {move || selected.get().map(|d| {
                view! {
                    <div data-calendar-selected>
                        "Selected: " {format!("{:04}-{:02}-{:02}", d.year(), d.month() as u8, d.day())}
                    </div>
                }
            })}
        </div>
    }
}

fn get_browser_date() -> Date {
    let js_date = js_sys::Date::new_0();
    let year = js_date.get_full_year() as i32;
    let month = (js_date.get_month() + 1) as u8;
    let day = js_date.get_date() as u8;
    
    Date::from_calendar_date(year, Month::try_from(month).unwrap(), day).unwrap()
}

fn format_month(m: u8) -> &'static str {
    match m {
        1 => "January", 2 => "February", 3 => "March", 4 => "April",
        5 => "May", 6 => "June", 7 => "July", 8 => "August",
        9 => "September", 10 => "October", 11 => "November", 12 => "December",
        _ => "Unknown"
    }
}

fn generate_calendar_days(year: i32, month: u8) -> Vec<Option<Date>> {
    let first = Date::from_calendar_date(year, Month::try_from(month).unwrap(), 1).unwrap();
    let first_weekday = first.weekday().number_days_from_sunday();
    
    let mut days = vec![None; first_weekday as usize];
    
    let days_in_month = time::util::days_in_year_month(year, Month::try_from(month).unwrap());
    for day in 1..=days_in_month {
        if let Ok(date) = Date::from_calendar_date(year, Month::try_from(month).unwrap(), day) {
            days.push(Some(date));
        }
    }
    
    while days.len() % 7 != 0 {
        days.push(None);
    }
    
    days
}
