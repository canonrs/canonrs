use super::types::{WidgetDef, WidgetContent};

/// Pre-configured dashboard templates
#[derive(Clone, Debug, PartialEq)]
pub enum DashboardTemplate {
    Analytics,
    Sales,
    DevOps,
    Executive,
}

impl DashboardTemplate {
    /// Get widgets for this template
    pub fn widgets(&self) -> Vec<WidgetDef> {
        match self {
            Self::Analytics => analytics_template(),
            Self::Sales => sales_template(),
            Self::DevOps => devops_template(),
            Self::Executive => executive_template(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Analytics => "Analytics Dashboard",
            Self::Sales => "Sales Dashboard",
            Self::DevOps => "DevOps Dashboard",
            Self::Executive => "Executive Dashboard",
        }
    }
}

/// Analytics template (6 widgets)
fn analytics_template() -> Vec<WidgetDef> {
    vec![
        WidgetDef::new("visitors", "Total Visitors")
            .position(0, 0, 3, 1)
            .content(WidgetContent::Html(
                r#"<div data-metric="visitors">
                    <div data-metric-value>24,531</div>
                    <div data-metric-change>↑ 12% from last month</div>
                </div>"#.to_string()
            )),

        WidgetDef::new("pageviews", "Page Views")
            .position(3, 0, 3, 1)
            .content(WidgetContent::Html(
                r#"<div data-metric="pageviews">
                    <div data-metric-value>182,492</div>
                    <div data-metric-change>↑ 8% from last month</div>
                </div>"#.to_string()
            )),

        WidgetDef::new("bounce-rate", "Bounce Rate")
            .position(6, 0, 3, 1)
            .content(WidgetContent::Html(
                r#"<div data-metric="bounce-rate">
                    <div data-metric-value data-state="warning">42.3%</div>
                    <div data-metric-change>↓ 3% from last month</div>
                </div>"#.to_string()
            )),

        WidgetDef::new("avg-session", "Avg Session")
            .position(9, 0, 3, 1)
            .content(WidgetContent::Html(
                r#"<div data-metric="avg-session">
                    <div data-metric-value>3m 42s</div>
                    <div data-metric-change>↑ 18% from last month</div>
                </div>"#.to_string()
            )),

        WidgetDef::new("traffic-chart", "Traffic Over Time")
            .position(0, 1, 8, 3)
            .content(WidgetContent::Html(
                r#"<div data-chart-placeholder>📊 Chart placeholder</div>"#.to_string()
            )),

        WidgetDef::new("top-pages", "Top Pages")
            .position(8, 1, 4, 3)
            .content(WidgetContent::Html(
                r#"<div data-list="top-pages">
                    <div data-list-item><span>/dashboard</span><span>12,483</span></div>
                    <div data-list-item><span>/products</span><span>8,291</span></div>
                    <div data-list-item><span>/pricing</span><span>6,142</span></div>
                </div>"#.to_string()
            )),
    ]
}

/// Sales template (5 widgets)
fn sales_template() -> Vec<WidgetDef> {
    vec![
        WidgetDef::new("revenue", "Total Revenue")
            .position(0, 0, 4, 1)
            .content(WidgetContent::Html(
                r#"<div data-metric="revenue">
                    <div data-metric-value>$142,583</div>
                    <div data-metric-change>↑ 24% MoM</div>
                </div>"#.to_string()
            )),

        WidgetDef::new("orders", "Orders")
            .position(4, 0, 4, 1)
            .content(WidgetContent::Html(
                r#"<div data-metric="orders">
                    <div data-metric-value>1,428</div>
                    <div data-metric-change>↑ 16% MoM</div>
                </div>"#.to_string()
            )),

        WidgetDef::new("avg-order", "Avg Order Value")
            .position(8, 0, 4, 1)
            .content(WidgetContent::Html(
                r#"<div data-metric="avg-order">
                    <div data-metric-value>$99.85</div>
                    <div data-metric-change>↑ 7% MoM</div>
                </div>"#.to_string()
            )),

        WidgetDef::new("sales-funnel", "Sales Funnel")
            .position(0, 1, 6, 3)
            .content(WidgetContent::Html(
                r#"<div data-funnel>
                    <div data-funnel-stage><span>Leads</span><span>10,000</span><div data-progress data-value="100"></div></div>
                    <div data-funnel-stage><span>Qualified</span><span>4,200</span><div data-progress data-value="42"></div></div>
                    <div data-funnel-stage><span>Proposals</span><span>1,680</span><div data-progress data-value="16.8"></div></div>
                    <div data-funnel-stage><span>Closed</span><span>420</span><div data-progress data-value="4.2" data-state="success"></div></div>
                </div>"#.to_string()
            )),

        WidgetDef::new("top-products", "Top Products")
            .position(6, 1, 6, 3)
            .content(WidgetContent::Html(
                r#"<div data-list="products">
                    <div data-list-item><div><div>Enterprise Plan</div><div data-meta>142 sales</div></div><div data-value>$42,600</div></div>
                    <div data-list-item><div><div>Pro Plan</div><div data-meta>286 sales</div></div><div data-value>$28,600</div></div>
                    <div data-list-item><div><div>Starter Plan</div><div data-meta>512 sales</div></div><div data-value>$15,360</div></div>
                </div>"#.to_string()
            )),
    ]
}

/// DevOps template (6 widgets)
fn devops_template() -> Vec<WidgetDef> {
    vec![
        WidgetDef::new("uptime", "Uptime")
            .position(0, 0, 3, 1)
            .content(WidgetContent::Html(
                r#"<div data-metric="uptime">
                    <div data-metric-value data-state="success">99.97%</div>
                    <div data-metric-change>Last 30 days</div>
                </div>"#.to_string()
            )),

        WidgetDef::new("deploys", "Deployments")
            .position(3, 0, 3, 1)
            .content(WidgetContent::Html(
                r#"<div data-metric="deploys">
                    <div data-metric-value>247</div>
                    <div data-metric-change>This month</div>
                </div>"#.to_string()
            )),

        WidgetDef::new("incidents", "Incidents")
            .position(6, 0, 3, 1)
            .content(WidgetContent::Html(
                r#"<div data-metric="incidents">
                    <div data-metric-value data-state="warning">3</div>
                    <div data-metric-change>Open incidents</div>
                </div>"#.to_string()
            )),

        WidgetDef::new("response-time", "Avg Response Time")
            .position(9, 0, 3, 1)
            .content(WidgetContent::Html(
                r#"<div data-metric="response-time">
                    <div data-metric-value data-state="success">142ms</div>
                    <div data-metric-change>↓ 8% from yesterday</div>
                </div>"#.to_string()
            )),

        WidgetDef::new("cpu-usage", "CPU Usage")
            .position(0, 1, 6, 2)
            .content(WidgetContent::Html(
                r#"<div data-server-list>
                    <div data-server-item><span>prod-server-01</span><span data-state="success">24%</span><div data-progress data-value="24" data-state="success"></div></div>
                    <div data-server-item><span>prod-server-02</span><span data-state="warning">68%</span><div data-progress data-value="68" data-state="warning"></div></div>
                    <div data-server-item><span>prod-server-03</span><span data-state="error">92%</span><div data-progress data-value="92" data-state="error"></div></div>
                </div>"#.to_string()
            )),

        WidgetDef::new("recent-deploys", "Recent Deployments")
            .position(6, 1, 6, 2)
            .content(WidgetContent::Html(
                r#"<div data-deploy-list>
                    <div data-deploy-item><div><div>frontend v2.4.1</div><div data-meta>2 minutes ago</div></div><span data-badge data-state="success">Success</span></div>
                    <div data-deploy-item><div><div>api v1.8.3</div><div data-meta>1 hour ago</div></div><span data-badge data-state="success">Success</span></div>
                    <div data-deploy-item><div><div>worker v0.9.2</div><div data-meta>3 hours ago</div></div><span data-badge data-state="error">Failed</span></div>
                </div>"#.to_string()
            )),
    ]
}

/// Executive template (simple overview)
fn executive_template() -> Vec<WidgetDef> {
    vec![
        WidgetDef::new("revenue-exec", "Revenue")
            .position(0, 0, 6, 2)
            .content(WidgetContent::Html(
                r#"<div data-metric="revenue-exec" data-size="large">
                    <div data-metric-value>$1.2M</div>
                    <div data-metric-label>Q4 2025 Revenue</div>
                    <div data-metric-change data-state="success">↑ 32% YoY</div>
                </div>"#.to_string()
            )),

        WidgetDef::new("customers-exec", "Customers")
            .position(6, 0, 6, 2)
            .content(WidgetContent::Html(
                r#"<div data-metric="customers-exec" data-size="large">
                    <div data-metric-value>8,420</div>
                    <div data-metric-label>Active Customers</div>
                    <div data-metric-change data-state="success">↑ 18% MoM</div>
                </div>"#.to_string()
            )),

        WidgetDef::new("kpis-exec", "Key Metrics")
            .position(0, 2, 12, 2)
            .content(WidgetContent::Html(
                r#"<div data-kpi-grid>
                    <div data-kpi><div data-kpi-value>94%</div><div data-kpi-label>Customer Satisfaction</div></div>
                    <div data-kpi><div data-kpi-value>$142</div><div data-kpi-label>Avg Deal Size</div></div>
                    <div data-kpi><div data-kpi-value>2.8%</div><div data-kpi-label>Churn Rate</div></div>
                    <div data-kpi><div data-kpi-value>42</div><div data-kpi-label>Days to Close</div></div>
                </div>"#.to_string()
            )),
    ]
}
