use leptos::prelude::*;
use super::StatCard;

pub fn basic_example() -> impl IntoView {
    view! {
        <StatCard
            label="Total Revenue".to_string()
            value="$48,295".to_string()
            change="+12.5%".to_string()
            trend="up".to_string()
        />
    }
}
