use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CalendarDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl CalendarDate {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    pub fn to_string(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    pub fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if Self::is_leap_year(year) { 29 } else { 28 },
            _ => 0,
        }
    }

    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    pub fn first_day_of_week(year: i32, month: u32) -> u32 {
        let m = month as i32;
        let y = year;
        let (m, y) = if m < 3 {
            (m + 12, y - 1)
        } else {
            (m, y)
        };
        let k = y % 100;
        let j = y / 100;
        let h = (1 + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
        ((h + 6) % 7) as u32
    }
}

// Função PURA para gerar células (sempre 42 itens)
fn calendar_cells(year: i32, month: u32) -> Vec<(i32, u32, u32, bool)> {
    let days_in_month = CalendarDate::days_in_month(year, month);
    let first_day = CalendarDate::first_day_of_week(year, month);

    let prev_month = if month == 1 { 12 } else { month - 1 };
    let prev_year = if month == 1 { year - 1 } else { year };
    let prev_days = CalendarDate::days_in_month(prev_year, prev_month);

    let mut cells = Vec::with_capacity(42);

    // Dias do mês anterior
    for i in 0..first_day {
        let day = prev_days - first_day + i + 1;
        cells.push((prev_year, prev_month, day, true));
    }

    // Dias do mês atual
    for day in 1..=days_in_month {
        cells.push((year, month, day, false));
    }

    // Dias do próximo mês
    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };
    let remaining = 42 - cells.len();
    for day in 1..=remaining as u32 {
        cells.push((next_year, next_month, day, true));
    }

    cells
}

#[component]
pub fn Calendar(
    year: i32,
    month: u32,
    #[prop(optional)] selected_date: Option<CalendarDate>,
    #[prop(optional)] today: Option<CalendarDate>,
    #[prop(optional)] disabled_dates: Option<Vec<CalendarDate>>,
    #[prop(optional)] min_date: Option<CalendarDate>,
    #[prop(optional)] max_date: Option<CalendarDate>,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let month_names = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ];
    let month_name = month_names.get((month - 1) as usize).unwrap_or(&"");

    view! {
        <div data-calendar id={id.unwrap_or_default()} class={class.unwrap_or_default()}>
            <div data-calendar-header>
                <button
                    data-button
                    data-ui-variant="ghost"
                    data-ui-size="sm"
                    data-calendar-nav="prev"
                    aria-label="Previous month"
                >"←"</button>
                <span data-calendar-title>{format!("{} {}", month_name, year)}</span>
                <button
                    data-button
                    data-ui-variant="ghost"
                    data-ui-size="sm"
                    data-calendar-nav="next"
                    aria-label="Next month"
                >"→"</button>
            </div>
            <div data-calendar-grid>
                <div data-calendar-grid-head>
                    <span>"Su"</span>
                    <span>"Mo"</span>
                    <span>"Tu"</span>
                    <span>"We"</span>
                    <span>"Th"</span>
                    <span>"Fr"</span>
                    <span>"Sa"</span>
                </div>
                <div data-calendar-grid-body>
                    {calendar_cells(year, month)
                        .into_iter()
                        .enumerate()
                        .map(|(idx, (y, m, d, outside))| {
                            let date = CalendarDate::new(y, m, d);
                            let date_str = date.to_string();

                            let is_selected = selected_date.map(|sd| sd == date).unwrap_or(false);
                            let is_today = today.map(|td| td == date).unwrap_or(false);
                            let is_disabled = disabled_dates.as_ref()
                                .map(|dd| dd.contains(&date))
                                .unwrap_or(false)
                                || min_date.map(|min| date.year < min.year || (date.year == min.year && (date.month < min.month || (date.month == min.month && date.day < min.day)))).unwrap_or(false)
                                || max_date.map(|max| date.year > max.year || (date.year == max.year && (date.month > max.month || (date.month == max.month && date.day > max.day)))).unwrap_or(false);

                            let tabindex = if idx == 0 && !is_disabled { "0" } else { "-1" };

                            view! {
                                <button
                                    data-calendar-cell
                                    data-date={date_str}
                                    data-outside-month={outside.then_some("true")}
                                    data-selected={is_selected.then_some("true")}
                                    data-today={is_today.then_some("true")}
                                    data-disabled={is_disabled.then_some("true")}
                                    disabled={is_disabled}
                                    tabindex={tabindex}
                                >
                                    {d.to_string()}
                                </button>
                            }
                        })
                        .collect::<Vec<_>>()
                    }
                </div>
            </div>
        </div>
    }
}
