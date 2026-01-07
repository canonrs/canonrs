//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Legacy dashboard components
//! @canon-owner: dashboard-team
//! @canon-target-date: 2025-03-15

//! DashboardConfig - Configuração do dashboard

use crate::tokens::Density;

#[derive(Debug, Clone)]
pub struct DashboardConfig {
    pub density: Density,
    pub show_charts: bool,
    pub show_activity: bool,
    pub refresh_interval: Option<u32>, // seconds
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            density: Density::Comfortable,
            show_charts: true,
            show_activity: true,
            refresh_interval: Some(30),
        }
    }
}
