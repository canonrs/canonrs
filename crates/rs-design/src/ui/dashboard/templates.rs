use super::types::{WidgetDef, WidgetPosition, WidgetContent};
use leptos::prelude::*;

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

/// Analytics template (4 widgets)
fn analytics_template() -> Vec<WidgetDef> {
    vec![
        WidgetDef::new("visitors", "Total Visitors")
            .position(0, 0, 3, 1)
            .content(WidgetContent::Custom(|| {
                view! {
                    <div class="text-center">
                        <div class="text-3xl font-bold text-primary">"24,531"</div>
                        <div class="text-xs text-muted-foreground">"↑ 12% from last month"</div>
                    </div>
                }.into_any()
            })),
        
        WidgetDef::new("pageviews", "Page Views")
            .position(3, 0, 3, 1)
            .content(WidgetContent::Custom(|| {
                view! {
                    <div class="text-center">
                        <div class="text-3xl font-bold text-primary">"182,492"</div>
                        <div class="text-xs text-muted-foreground">"↑ 8% from last month"</div>
                    </div>
                }.into_any()
            })),
        
        WidgetDef::new("bounce-rate", "Bounce Rate")
            .position(6, 0, 3, 1)
            .content(WidgetContent::Custom(|| {
                view! {
                    <div class="text-center">
                        <div class="text-3xl font-bold text-destructive">"42.3%"</div>
                        <div class="text-xs text-muted-foreground">"↓ 3% from last month"</div>
                    </div>
                }.into_any()
            })),
        
        WidgetDef::new("avg-session", "Avg Session")
            .position(9, 0, 3, 1)
            .content(WidgetContent::Custom(|| {
                view! {
                    <div class="text-center">
                        <div class="text-3xl font-bold text-primary">"3m 42s"</div>
                        <div class="text-xs text-muted-foreground">"↑ 18% from last month"</div>
                    </div>
                }.into_any()
            })),
        
        WidgetDef::new("traffic-chart", "Traffic Over Time")
            .position(0, 1, 8, 3)
            .content(WidgetContent::Custom(|| {
                view! {
                    <div class="h-full flex items-center justify-center text-muted-foreground">
                        "📊 Chart placeholder"
                    </div>
                }.into_any()
            })),
        
        WidgetDef::new("top-pages", "Top Pages")
            .position(8, 1, 4, 3)
            .content(WidgetContent::Custom(|| {
                view! {
                    <div class="space-y-2 text-sm">
                        <div class="flex justify-between">
                            <span>"/dashboard"</span>
                            <span class="text-muted-foreground">"12,483"</span>
                        </div>
                        <div class="flex justify-between">
                            <span>"/products"</span>
                            <span class="text-muted-foreground">"8,291"</span>
                        </div>
                        <div class="flex justify-between">
                            <span>"/pricing"</span>
                            <span class="text-muted-foreground">"6,142"</span>
                        </div>
                    </div>
                }.into_any()
            })),
    ]
}

/// Sales template (5 widgets)
fn sales_template() -> Vec<WidgetDef> {
    vec![
        WidgetDef::new("revenue", "Total Revenue")
            .position(0, 0, 4, 1)
            .content(WidgetContent::Custom(|| {
                view! {
                    <div class="text-center">
                        <div class="text-3xl font-bold text-primary">"$142,583"</div>
                        <div class="text-xs text-muted-foreground">"↑ 24% MoM"</div>
                    </div>
                }.into_any()
            })),
        
        WidgetDef::new("orders", "Orders")
            .position(4, 0, 4, 1)
            .content(WidgetContent::Custom(|| {
                view! {
                    <div class="text-center">
                        <div class="text-3xl font-bold text-primary">"1,428"</div>
                        <div class="text-xs text-muted-foreground">"↑ 16% MoM"</div>
                    </div>
                }.into_any()
            })),
        
        WidgetDef::new("avg-order", "Avg Order Value")
            .position(8, 0, 4, 1)
            .content(WidgetContent::Custom(|| {
                view! {
                    <div class="text-center">
                        <div class="text-3xl font-bold text-primary">"$99.85"</div>
                        <div class="text-xs text-muted-foreground">"↑ 7% MoM"</div>
                    </div>
                }.into_any()
            })),
        
        WidgetDef::new("sales-funnel", "Sales Funnel")
            .position(0, 1, 6, 3)
            .content(WidgetContent::Custom(|| {
                view! {
                    <div class="space-y-3">
                        <div>
                            <div class="flex justify-between text-xs mb-1">
                                <span>"Leads"</span>
                                <span>"10,000"</span>
                            </div>
                            <div class="h-2 bg-muted rounded-full overflow-hidden">
                                <div class="h-full bg-primary" style="width: 100%"></div>
                            </div>
                        </div>
                        <div>
                            <div class="flex justify-between text-xs mb-1">
                                <span>"Qualified"</span>
                                <span>"4,200"</span>
                            </div>
                            <div class="h-2 bg-muted rounded-full overflow-hidden">
                                <div class="h-full bg-primary" style="width: 42%"></div>
                            </div>
                        </div>
                        <div>
                            <div class="flex justify-between text-xs mb-1">
                                <span>"Proposals"</span>
                                <span>"1,680"</span>
                            </div>
                            <div class="h-2 bg-muted rounded-full overflow-hidden">
                                <div class="h-full bg-primary" style="width: 16.8%"></div>
                            </div>
                        </div>
                        <div>
                            <div class="flex justify-between text-xs mb-1">
                                <span>"Closed"</span>
                                <span>"420"</span>
                            </div>
                            <div class="h-2 bg-muted rounded-full overflow-hidden">
                                <div class="h-full bg-success" style="width: 4.2%"></div>
                            </div>
                        </div>
                    </div>
                }.into_any()
            })),
        
        WidgetDef::new("top-products", "Top Products")
            .position(6, 1, 6, 3)
            .content(WidgetContent::Custom(|| {
                view! {
                    <div class="space-y-3 text-sm">
                        <div class="flex items-center justify-between">
                            <div>
                                <div class="font-medium">"Enterprise Plan"</div>
                                <div class="text-xs text-muted-foreground">"142 sales"</div>
                            </div>
                            <div class="text-primary font-semibold">"$42,600"</div>
                        </div>
                        <div class="flex items-center justify-between">
                            <div>
                                <div class="font-medium">"Pro Plan"</div>
                                <div class="text-xs text-muted-foreground">"286 sales"</div>
                            </div>
                            <div class="text-primary font-semibold">"$28,600"</div>
                        </div>
                        <div class="flex items-center justify-between">
                            <div>
                                <div class="font-medium">"Starter Plan"</div>
                                <div class="text-xs text-muted-foreground">"512 sales"</div>
                            </div>
                            <div class="text-primary font-semibold">"$15,360"</div>
                        </div>
                    </div>
                }.into_any()
            })),
    ]
}

/// DevOps template (6 widgets)
fn devops_template() -> Vec<WidgetDef> {
    vec![
        WidgetDef::new("uptime", "Uptime")
            .position(0, 0, 3, 1)
            .content(WidgetContent::Custom(|| {
                view! {
                    <div class="text-center">
                        <div class="text-3xl font-bold text-success">"99.97%"</div>
                        <div class="text-xs text-muted-foreground">"Last 30 days"</div>
                    </div>
                }.into_any()
            })),
        
        WidgetDef::new("deploys", "Deployments")
            .position(3, 0, 3, 1)
            .content(WidgetContent::Custom(|| {
                view! {
                    <div class="text-center">
                        <div class="text-3xl font-bold text-primary">"247"</div>
                        <div class="text-xs text-muted-foreground">"This month"</div>
                    </div>
                }.into_any()
            })),
        
        WidgetDef::new("incidents", "Incidents")
            .position(6, 0, 3, 1)
            .content(WidgetContent::Custom(|| {
                view! {
                    <div class="text-center">
                        <div class="text-3xl font-bold text-warning">"3"</div>
                        <div class="text-xs text-muted-foreground">"Open incidents"</div>
                    </div>
                }.into_any()
            })),
        
        WidgetDef::new("response-time", "Avg Response Time")
            .position(9, 0, 3, 1)
            .content(WidgetContent::Custom(|| {
                view! {
                    <div class="text-center">
                        <div class="text-3xl font-bold text-success">"142ms"</div>
                        <div class="text-xs text-muted-foreground">"↓ 8% from yesterday"</div>
                    </div>
                }.into_any()
            })),
        
        WidgetDef::new("cpu-usage", "CPU Usage")
            .position(0, 1, 6, 2)
            .content(WidgetContent::Custom(|| {
                view! {
                    <div class="space-y-2">
                        <div class="flex justify-between text-sm">
                            <span>"prod-server-01"</span>
                            <span class="text-success">"24%"</span>
                        </div>
                        <div class="h-2 bg-muted rounded-full overflow-hidden">
                            <div class="h-full bg-success" style="width: 24%"></div>
                        </div>
                        <div class="flex justify-between text-sm">
                            <span>"prod-server-02"</span>
                            <span class="text-warning">"68%"</span>
                        </div>
                        <div class="h-2 bg-muted rounded-full overflow-hidden">
                            <div class="h-full bg-warning" style="width: 68%"></div>
                        </div>
                        <div class="flex justify-between text-sm">
                            <span>"prod-server-03"</span>
                            <span class="text-destructive">"92%"</span>
                        </div>
                        <div class="h-2 bg-muted rounded-full overflow-hidden">
                            <div class="h-full bg-destructive" style="width: 92%"></div>
                        </div>
                    </div>
                }.into_any()
            })),
        
        WidgetDef::new("recent-deploys", "Recent Deployments")
            .position(6, 1, 6, 2)
            .content(WidgetContent::Custom(|| {
                view! {
                    <div class="space-y-2 text-sm">
                        <div class="flex items-center justify-between">
                            <div>
                                <div class="font-medium">"frontend v2.4.1"</div>
                                <div class="text-xs text-muted-foreground">"2 minutes ago"</div>
                            </div>
                            <span class="text-xs bg-success/10 text-success px-2 py-1 rounded">"Success"</span>
                        </div>
                        <div class="flex items-center justify-between">
                            <div>
                                <div class="font-medium">"api v1.8.3"</div>
                                <div class="text-xs text-muted-foreground">"1 hour ago"</div>
                            </div>
                            <span class="text-xs bg-success/10 text-success px-2 py-1 rounded">"Success"</span>
                        </div>
                        <div class="flex items-center justify-between">
                            <div>
                                <div class="font-medium">"worker v0.9.2"</div>
                                <div class="text-xs text-muted-foreground">"3 hours ago"</div>
                            </div>
                            <span class="text-xs bg-destructive/10 text-destructive px-2 py-1 rounded">"Failed"</span>
                        </div>
                    </div>
                }.into_any()
            })),
    ]
}

/// Executive template (simple overview)
fn executive_template() -> Vec<WidgetDef> {
    vec![
        WidgetDef::new("revenue-exec", "Revenue")
            .position(0, 0, 6, 2)
            .content(WidgetContent::Custom(|| {
                view! {
                    <div class="text-center">
                        <div class="text-4xl font-bold text-primary mb-2">"$1.2M"</div>
                        <div class="text-sm text-muted-foreground">"Q4 2025 Revenue"</div>
                        <div class="mt-4 text-success text-sm">"↑ 32% YoY"</div>
                    </div>
                }.into_any()
            })),
        
        WidgetDef::new("customers-exec", "Customers")
            .position(6, 0, 6, 2)
            .content(WidgetContent::Custom(|| {
                view! {
                    <div class="text-center">
                        <div class="text-4xl font-bold text-primary mb-2">"8,420"</div>
                        <div class="text-sm text-muted-foreground">"Active Customers"</div>
                        <div class="mt-4 text-success text-sm">"↑ 18% MoM"</div>
                    </div>
                }.into_any()
            })),
        
        WidgetDef::new("kpis-exec", "Key Metrics")
            .position(0, 2, 12, 2)
            .content(WidgetContent::Custom(|| {
                view! {
                    <div class="grid grid-cols-4 gap-4 text-center">
                        <div>
                            <div class="text-2xl font-bold">"94%"</div>
                            <div class="text-xs text-muted-foreground">"Customer Satisfaction"</div>
                        </div>
                        <div>
                            <div class="text-2xl font-bold">"$142"</div>
                            <div class="text-xs text-muted-foreground">"Avg Deal Size"</div>
                        </div>
                        <div>
                            <div class="text-2xl font-bold">"2.8%"</div>
                            <div class="text-xs text-muted-foreground">"Churn Rate"</div>
                        </div>
                        <div>
                            <div class="text-2xl font-bold">"42"</div>
                            <div class="text-xs text-muted-foreground">"Days to Close"</div>
                        </div>
                    </div>
                }.into_any()
            })),
    ]
}
