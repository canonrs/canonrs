use leptos::prelude::*;
use super::chart_ui::{Chart, ChartData, ChartSeries, ChartType};

pub fn basic_example() -> impl IntoView {
    let data = ChartData {
        labels: vec![
            "Jan".into(), "Feb".into(), "Mar".into(),
            "Apr".into(), "May".into(), "Jun".into(),
        ],
        series: vec![
            ChartSeries {
                name: "Revenue".into(),
                data: vec![42.0, 58.0, 45.0, 72.0, 65.0, 88.0],
                color: None,
            },
            ChartSeries {
                name: "Expenses".into(),
                data: vec![28.0, 35.0, 32.0, 48.0, 41.0, 55.0],
                color: None,
            },
        ],
    };

    view! {
        <Chart
            id="chart-basic"
            data={data}
            chart_type={ChartType::Line}
            height={320}
        />
    }
}

pub fn chart_datatable_sync_example() -> impl IntoView {
    use crate::ui::data_table::{DataTableFull, DataTableColumn};

    #[derive(Clone)]
    struct Row {
        month: String,
        revenue: f64,
        expenses: f64,
    }

    let rows = vec![
        Row { month: "Jan".into(), revenue: 42.0, expenses: 28.0 },
        Row { month: "Feb".into(), revenue: 58.0, expenses: 35.0 },
        Row { month: "Mar".into(), revenue: 45.0, expenses: 32.0 },
        Row { month: "Apr".into(), revenue: 72.0, expenses: 48.0 },
        Row { month: "May".into(), revenue: 65.0, expenses: 41.0 },
        Row { month: "Jun".into(), revenue: 88.0, expenses: 55.0 },
    ];

    let labels: Vec<String> = rows.iter().map(|r| r.month.clone()).collect();
    let rev: Vec<f64>  = rows.iter().map(|r| r.revenue).collect();
    let exp: Vec<f64>  = rows.iter().map(|r| r.expenses).collect();

    let data = ChartData {
        labels,
        series: vec![
            ChartSeries { name: "Revenue".into(),  data: rev, color: None },
            ChartSeries { name: "Expenses".into(), data: exp, color: None },
        ],
    };

    let columns = vec![
        DataTableColumn::new("month",    "Month",    |r: &Row| r.month.clone()),
        DataTableColumn::new("revenue",  "Revenue",  |r: &Row| format!("{:.0}", r.revenue)),
        DataTableColumn::new("expenses", "Expenses", |r: &Row| format!("{:.0}", r.expenses)),
    ];

    view! {
        <div style="display:flex;flex-direction:column;gap:1.5rem;">
            <Chart
                id="chart-sync"
                data={data}
                chart_type={ChartType::Line}
                height={320}
                sync_scope="revenue-2026".to_string()
            />
            <DataTableFull
                id="datatable-sync"
                data={rows}
                columns={columns}
                page_size={6}
                sync_scope="revenue-2026".to_string()
            />
        </div>
    }
}
