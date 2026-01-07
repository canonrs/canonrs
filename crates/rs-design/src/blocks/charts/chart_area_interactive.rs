//! @canon-level: loose
//! @canon-exceptions: [#24]
//! @canon-justification: Fixed chart dimensions
//! @canon-owner: charts-team
//! @canon-target-date: 2025-04-01

use crate::tokens::{CHART_COLORS, SPACING};
use crate::ui::{AreaChart, ChartContainer, 
};
use leptos::prelude::*;

#[component]
pub fn ChartAreaInteractive(
    data: Vec<ChartDataPoint>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] description: Option<String>,
) -> impl IntoView {
    let time_range = RwSignal::new("90d".to_string());

    let filtered_data = Memo::new(move |_| {
        let range = time_range.get();
        let days = match range.as_str() {
            "7d" => 7,
            "30d" => 30,
            _ => 90,
        };

        let data_clone = data.clone();
        let len = data_clone.len();
        if len > days {
            data_clone[len - days..].to_vec()
        } else {
            data_clone
        }
    });

    let config = ChartConfig {
        desktop_label: "Desktop".to_string(),
        desktop_color: CHART_COLORS.chart_1.to_string(),
        mobile_label: "Mobile".to_string(),
        mobile_color: CHART_COLORS.chart_2.to_string(),
    };

    view! {
        <Card class="pt-0".to_string()>
            <div class="flex items-center gap-2 border-b py-5 px-6 sm:flex-row">
                <div class="grid flex-1 gap-1">
                    <h3 class="text-lg font-semibold">
                        {title.unwrap_or("Area Chart - Interactive".to_string())}
                    </h3>
                    <p class="text-sm text-muted-foreground">
                        {description.unwrap_or("Showing total visitors".to_string())}
                    </p>
                </div>

    let time_range = RwSignal::new("90d".to_string());
                <Select value=time_range class="w-[160px]".to_string()>
                    <SelectTrigger>
                        <SelectValue placeholder="Last 3 months" />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectOption value="90d".to_string()>
                            "Last 3 months"
                        </SelectOption>
                        <SelectOption value="30d".to_string()>
                            "Last 30 days"
                        </SelectOption>
                        <SelectOption value="7d".to_string()>
                            "Last 7 days"
                        </SelectOption>
                    </SelectContent>
                </Select>
            </div>

            <div class=format!("px-2 pt-4 sm:px-6 sm:pt-6 pb-[{}]", SPACING.lg)>
                <ChartContainer class="aspect-auto h-[250px] w-full".to_string()>
                    <AreaChart
                        data=filtered_data.get()
                        config=config
                        height=250.0
                    />
                </ChartContainer>
            </div>
        </Card>
    }
}
