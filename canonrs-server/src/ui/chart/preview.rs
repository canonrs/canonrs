use leptos::prelude::*;
use super::chart_island::ChartIsland;
use canonrs_core::{ChartData, ChartSeries};
use canonrs_core::primitives::ChartType;

fn monthly_data() -> ChartData {
    ChartData {
        labels: vec![
            "Jan".to_string(), "Feb".to_string(), "Mar".to_string(),
            "Apr".to_string(), "May".to_string(), "Jun".to_string(),
        ],
        series: vec![
            ChartSeries { name: "Revenue".to_string(), data: vec![42.0, 58.0, 51.0, 73.0, 65.0, 89.0], color: None },
            ChartSeries { name: "Expenses".to_string(), data: vec![31.0, 40.0, 38.0, 52.0, 47.0, 61.0], color: None },
        ],
    }
}

fn bar_data() -> ChartData {
    ChartData {
        labels: vec!["Q1".to_string(), "Q2".to_string(), "Q3".to_string(), "Q4".to_string()],
        series: vec![
            ChartSeries { name: "Growth".to_string(), data: vec![24.0, 38.0, 55.0, 72.0], color: None },
        ],
    }
}

fn area_data() -> ChartData {
    ChartData {
        labels: vec!["Mon".to_string(), "Tue".to_string(), "Wed".to_string(), "Thu".to_string(), "Fri".to_string()],
        series: vec![
            ChartSeries { name: "Users".to_string(), data: vec![120.0, 145.0, 132.0, 178.0, 195.0], color: None },
        ],
    }
}

#[component]
pub fn ChartShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="" style="width:100%;">
                <ChartIsland data=monthly_data() chart_type=ChartType::Line height=280u32 value="revenue-monthly" aria_label="Monthly revenue and expenses line chart" />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Chart structure and data binding enforced via contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Bar"</span>
                <div data-rs-showcase-preview-row="" style="width:100%;">
                    <ChartIsland data=bar_data() chart_type=ChartType::Bar height=200u32 value="growth-quarterly" aria_label="Quarterly growth bar chart" />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Area"</span>
                <div data-rs-showcase-preview-row="" style="width:100%;">
                    <ChartIsland data=area_data() chart_type=ChartType::Area height=200u32 value="users-weekly" aria_label="Weekly active users area chart" />
                </div>
            </div>
        </div>
    }
}
