//! @canon-level: loose
//! @canon-exceptions: [#24]
//! @canon-justification: Fixed chart dimensions
//! @canon-owner: charts-team
//! @canon-target-date: 2025-04-01

use crate::primitives::icons::*;
use crate::ui::{Card, ChartContainer, PieChart, PieChartDataPoint};
use leptos::prelude::*;

#[component]
pub fn ChartPieLabel(
    data: Vec<PieChartDataPoint>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] description: Option<String>,
    #[prop(optional)] footer_text: Option<String>,
    #[prop(optional)] trend_percentage: Option<f64>,
) -> impl IntoView {
    let total: f64 = data.iter().map(|d| d.value).sum();
    let data_clone = data.clone();

    view! {
        <Card class="flex flex-col".to_string()>
            <div class="items-center pb-0 text-center px-6 pt-6">
                <h3 class="text-lg font-semibold mb-1">
                    {title.unwrap_or("Pie Chart - Label".to_string())}
                </h3>
                <p class="text-sm text-muted-foreground">
                    {description.unwrap_or("January - June 2024".to_string())}
                </p>
            </div>

            <div class="flex-1 pb-0 px-6 pt-4">
                <ChartContainer class="mx-auto aspect-square max-h-[250px] pb-0".to_string()>
                    <PieChart
                        data=data
                        size=250.0
                        show_labels=true
                    />
                </ChartContainer>
            </div>

            <div class="flex-col gap-2 text-sm px-6 pb-6 pt-4">
                {trend_percentage.map(|percentage| view! {
                    <div class="flex items-center justify-center gap-2 leading-none font-medium">
                        {format!("Trending up by {:.1}% this month", percentage)}
                        <IconTrendingUp class="h-4 w-4".to_string() />
                    </div>
                })}

                <div class="text-muted-foreground leading-none text-center mt-2">
                    {footer_text.unwrap_or(format!("Showing total of {:.0} visitors", total))}
                </div>

                // Legend
                <div class="grid grid-cols-2 gap-2 mt-4">
                    {data_clone.iter().map(|point| view! {
                        <div class="flex items-center gap-2">
                            <div
                                class="w-3 h-3 rounded-sm"
                                style=format!("background-color: {}", point.color)
                            />
                            <span class="text-xs text-muted-foreground">
                                {format!("{}: {:.0}", point.label, point.value)}
                            </span>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </Card>
    }
}
