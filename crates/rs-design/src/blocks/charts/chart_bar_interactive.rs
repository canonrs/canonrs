//! @canon-level: loose
//! @canon-exceptions: [#24]
//! @canon-justification: Fixed chart dimensions
//! @canon-owner: charts-team
//! @canon-target-date: 2025-04-01

use crate::tokens::{CHART_COLORS};
use crate::ui::{BarChart, Card, ChartConfig, ChartContainer, ChartDataPoint};
use leptos::prelude::*;

#[component]
pub fn ChartBarInteractive(
    data: Vec<ChartDataPoint>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] description: Option<String>,
) -> impl IntoView {
    let active_chart = RwSignal::new("desktop".to_string());

    // Calcular totais
    let total_desktop: f64 = data.iter().map(|d| d.desktop).sum();
    let total_mobile: f64 = data.iter().map(|d| d.mobile).sum();

    let config = ChartConfig {
        desktop_label: "Desktop".to_string(),
        desktop_color: CHART_COLORS.chart_2.to_string(),
        mobile_label: "Mobile".to_string(),
        mobile_color: CHART_COLORS.chart_1.to_string(),
    };

    view! {
        <Card class="py-0".to_string()>
            <div class="flex flex-col items-stretch border-b !p-0 sm:flex-row">
                <div class="flex flex-1 flex-col justify-center gap-1 px-6 pt-4 pb-3">
                    <h3 class="text-lg font-semibold">
                        {title.unwrap_or("Bar Chart - Interactive".to_string())}
                    </h3>
                    <p class="text-sm text-muted-foreground">
                        {description.unwrap_or("Showing total visitors for the last 3 months".to_string())}
                    </p>
                </div>

                <div class="flex">
                    // Desktop button
                    <button
                        class=move || format!(
                            "relative z-30 flex flex-1 flex-col justify-center gap-1 border-t px-6 py-4 text-left sm:border-t-0 sm:border-l sm:px-8 sm:py-6 {}",
                            if active_chart.get() == "desktop" { "bg-muted/50" } else { "" }
                        )
                        on:click=move |_| active_chart.set("desktop".to_string())
                    >
                        <span class="text-muted-foreground text-xs">
                            "Desktop"
                        </span>
                        <span class="text-lg leading-none font-bold sm:text-3xl">
                            {format!("{:.0}", total_desktop)}
                        </span>
                    </button>

                    // Mobile button
                    <button
                        class=move || format!(
                            "relative z-30 flex flex-1 flex-col justify-center gap-1 border-t border-l px-6 py-4 text-left sm:border-t-0 sm:px-8 sm:py-6 {}",
                            if active_chart.get() == "mobile" { "bg-muted/50" } else { "" }
                        )
                        on:click=move |_| active_chart.set("mobile".to_string())
                    >
                        <span class="text-muted-foreground text-xs">
                            "Mobile"
                        </span>
                        <span class="text-lg leading-none font-bold sm:text-3xl">
                            {format!("{:.0}", total_mobile)}
                        </span>
                    </button>
                </div>
            </div>

            <div class="px-2 sm:p-6">
                <ChartContainer class="aspect-auto h-[250px] w-full".to_string()>
                    <BarChart
                        data=data.clone()
                        config=config
                        height=250.0
                        active_key=active_chart.get()
                    />
                </ChartContainer>
            </div>
        </Card>
    }
}
