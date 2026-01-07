use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ChartDataPoint {
    pub date: String,
    pub desktop: f64,
    pub mobile: f64,
}

#[derive(Clone)]
pub struct ChartConfig {
    pub desktop_label: String,
    pub desktop_color: String,
    pub mobile_label: String,
    pub mobile_color: String,
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self {
            desktop_label: "Desktop".to_string(),
            desktop_color: "#2563eb".to_string(),
            mobile_label: "Mobile".to_string(),
            mobile_color: "#10b981".to_string(),
        }
    }
}

#[component]
pub fn ChartContainer(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div class=format!("w-full {}", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn AreaChart(
    data: Vec<ChartDataPoint>,
    #[prop(default = ChartConfig::default())] config: ChartConfig,
    #[prop(default = 250.0)] height: f64,
) -> impl IntoView {
    let width = 800.0;
    let padding = 40.0;
    let chart_width = width - (padding * 2.0);
    let chart_height = height - (padding * 2.0);

    // Calcular valores máximos
    let max_value = data
        .iter()
        .map(|d| d.desktop.max(d.mobile))
        .fold(0.0, f64::max);

    // Gerar path para desktop
    let desktop_path =
        generate_area_path(&data, chart_width, chart_height, max_value, |d| d.desktop);

    // Gerar path para mobile
    let mobile_path = generate_area_path(&data, chart_width, chart_height, max_value, |d| d.mobile);

    view! {
        <svg
            width=format!("{}px", width)
            height=format!("{}px", height)
            viewBox=format!("0 0 {} {}", width, height)
            class="w-full"
        >
            <defs>
                <linearGradient id="fillDesktop" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stop-color=config.desktop_color.clone() stop-opacity="0.8"/>
                    <stop offset="95%" stop-color=config.desktop_color.clone() stop-opacity="0.1"/>
                </linearGradient>
                <linearGradient id="fillMobile" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stop-color=config.mobile_color.clone() stop-opacity="0.8"/>
                    <stop offset="95%" stop-color=config.mobile_color.clone() stop-opacity="0.1"/>
                </linearGradient>
            </defs>

            // Grid
            <g transform=format!("translate({}, {})", padding, padding)>
                {(0..5).map(|i| {
                    let y = (chart_height / 4.0) * i as f64;
                    view! {
                        <line
                            x1="0"
                            y1=y
                            x2=chart_width
                            y2=y
                            stroke="#e5e7eb"
                            stroke-width="1"
                        />
                    }
                }).collect::<Vec<_>>()}
            </g>

            // Areas
            <g transform=format!("translate({}, {})", padding, padding)>
                <path
                    d=mobile_path
                    fill="url(#fillMobile)"
                    stroke=config.mobile_color.clone()
                    stroke-width="2"
                />
                <path
                    d=desktop_path
                    fill="url(#fillDesktop)"
                    stroke=config.desktop_color
                    stroke-width="2"
                />
            </g>

            // X Axis labels
            <g transform=format!("translate({}, {})", padding, height - padding + 10.0)>
                {data.iter().enumerate().filter(|(i, _)| i % 10 == 0).map(|(i, d)| {
                    let x = (chart_width / (data.len() - 1) as f64) * i as f64;
                    view! {
                        <text
                            x=x
                            y="10"
                            text-anchor="middle"
                            class="text-xs fill-gray-500"
                        >
                            {format_date(&d.date)}
                        </text>
                    }
                }).collect::<Vec<_>>()}
            </g>
        </svg>
    }
}

fn generate_area_path<F>(
    data: &[ChartDataPoint],
    width: f64,
    height: f64,
    max_value: f64,
    accessor: F,
) -> String
where
    F: Fn(&ChartDataPoint) -> f64,
{
    if data.is_empty() {
        return String::new();
    }

    let mut path = String::new();
    let step = width / (data.len() - 1) as f64;

    // Start at bottom left
    path.push_str(&format!("M 0,{} ", height));

    // Draw line through points
    for (i, point) in data.iter().enumerate() {
        let x = step * i as f64;
        let value = accessor(point);
        let y = height - (value / max_value * height);

        if i == 0 {
            path.push_str(&format!("L {},{}  ", x, y));
        } else {
            path.push_str(&format!("L {},{}  ", x, y));
        }
    }

    // Close path at bottom right
    path.push_str(&format!("L {},{}  Z", width, height));
    path
}

fn format_date(date: &str) -> String {
    // Simplificado - apenas pega mês e dia
    if let Some(parts) = date.split('-').collect::<Vec<_>>().get(1..3) {
        format!("{}/{}", parts[0], parts[1])
    } else {
        date.to_string()
    }
}

#[component]
pub fn BarChart(
    data: Vec<ChartDataPoint>,
    #[prop(default = ChartConfig::default())] config: ChartConfig,
    #[prop(default = 250.0)] height: f64,
    #[prop(default = "desktop".to_string())] active_key: String,
) -> impl IntoView {
    let width = 800.0;
    let padding = 40.0;
    let chart_width = width - (padding * 2.0);
    let chart_height = height - (padding * 2.0);

    // Calcular valor máximo
    let max_value = data
        .iter()
        .map(|d| {
            if active_key == "desktop" {
                d.desktop
            } else {
                d.mobile
            }
        })
        .fold(0.0, f64::max);

    let bar_width = (chart_width / data.len() as f64) * 0.7;
    let bar_gap = (chart_width / data.len() as f64) * 0.3;

    let active_color = if active_key == "desktop" {
        config.desktop_color.clone()
    } else {
        config.mobile_color.clone()
    };

    view! {
        <svg
            width=format!("{}px", width)
            height=format!("{}px", height)
            viewBox=format!("0 0 {} {}", width, height)
            class="w-full"
        >
            // Grid horizontal
            <g transform=format!("translate({}, {})", padding, padding)>
                {(0..5).map(|i| {
                    let y = (chart_height / 4.0) * i as f64;
                    view! {
                        <line
                            x1="0"
                            y1=y
                            x2=chart_width
                            y2=y
                            stroke="#e5e7eb"
                            stroke-width="1"
                        />
                    }
                }).collect::<Vec<_>>()}
            </g>

            // Barras
            <g transform=format!("translate({}, {})", padding, padding)>
                {data.iter().enumerate().map(|(i, point)| {
                    let x = ((bar_width + bar_gap) * i as f64) + (bar_gap / 2.0);
                    let value = if active_key == "desktop" { point.desktop } else { point.mobile };
                    let bar_height = (value / max_value) * chart_height;
                    let y = chart_height - bar_height;

                    view! {
                        <rect
                            x=x
                            y=y
                            width=bar_width
                            height=bar_height
                            fill=active_color.clone()
                            rx="4"
                            class="transition-all duration-300"
                        />
                    }
                }).collect::<Vec<_>>()}
            </g>

            // X Axis labels
            <g transform=format!("translate({}, {})", padding, height - padding + 10.0)>
                {data.iter().enumerate().filter(|(i, _)| i % 10 == 0).map(|(i, d)| {
                    let x = ((bar_width + bar_gap) * i as f64) + (bar_gap / 2.0) + (bar_width / 2.0);
                    view! {
                        <text
                            x=x
                            y="10"
                            text-anchor="middle"
                            class="text-xs fill-gray-500"
                        >
                            {format_date(&d.date)}
                        </text>
                    }
                }).collect::<Vec<_>>()}
            </g>
        </svg>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PieChartDataPoint {
    pub label: String,
    pub value: f64,
    pub color: String,
}

#[component]
pub fn PieChart(
    data: Vec<PieChartDataPoint>,
    #[prop(default = 250.0)] size: f64,
    #[prop(default = true)] show_labels: bool,
) -> impl IntoView {
    let center_x = size / 2.0;
    let center_y = size / 2.0;
    let radius = (size / 2.0) - 40.0;

    // Calcular total
    let total: f64 = data.iter().map(|d| d.value).sum();

    // Gerar arcos
    let mut current_angle = -90.0; // Começar no topo
    let slices: Vec<_> = data
        .iter()
        .map(|point| {
            let percentage = point.value / total;
            let angle_size = percentage * 360.0;

            let start_angle = current_angle;
            let end_angle = current_angle + angle_size;
            current_angle = end_angle;

            let path = create_pie_slice(center_x, center_y, radius, start_angle, end_angle);

            // Calcular posição do label
            let label_angle = start_angle + (angle_size / 2.0);
            let label_radius = radius * 0.7;
            let label_x = center_x + label_radius * label_angle.to_radians().cos();
            let label_y = center_y + label_radius * label_angle.to_radians().sin();

            (path, point.clone(), label_x, label_y, percentage)
        })
        .collect();

    view! {
        <svg
            width=format!("{}px", size)
            height=format!("{}px", size)
            viewBox=format!("0 0 {} {}", size, size)
            class="w-full"
        >
            <g>
                {slices.iter().map(|(path, point, label_x, label_y, percentage)| {
                    view! {
                        <>
                            <path
                                d=path.clone()
                                fill=point.color.clone()
                                stroke="white"
                                stroke-width="2"
                                class="transition-all duration-300 hover:opacity-80"
                            />
                            {show_labels.then(|| view! {
                                <text
                                    x=*label_x
                                    y=*label_y
                                    text-anchor="middle"
                                    class="text-xs font-medium fill-white"
                                >
                                    {format!("{:.1}%", percentage * 100.0)}
                                </text>
                            })}
                        </>
                    }
                }).collect::<Vec<_>>()}
            </g>

            // Center circle (donut style - opcional)
            <circle
                cx=center_x
                cy=center_y
                r=radius * 0.4
                fill="white"
            />
        </svg>
    }
}

fn create_pie_slice(cx: f64, cy: f64, radius: f64, start_angle: f64, end_angle: f64) -> String {
    let start_rad = start_angle.to_radians();
    let end_rad = end_angle.to_radians();

    let x1 = cx + radius * start_rad.cos();
    let y1 = cy + radius * start_rad.sin();
    let x2 = cx + radius * end_rad.cos();
    let y2 = cy + radius * end_rad.sin();

    let large_arc = if (end_angle - start_angle) > 180.0 {
        1
    } else {
        0
    };

    format!(
        "M {},{} L {},{} A {},{} 0 {} 1 {},{} Z",
        cx, cy, x1, y1, radius, radius, large_arc, x2, y2
    )
}
