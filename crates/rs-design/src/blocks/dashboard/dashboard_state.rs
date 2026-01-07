//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Legacy dashboard components
//! @canon-owner: dashboard-team
//! @canon-target-date: 2025-03-15

//! DashboardState - Estados tipados para dashboard enterprise

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashboardState {
    Loading,
    Ready,
    Error,
    Empty,
}

impl DashboardState {
    pub fn is_loading(&self) -> bool {
        matches!(self, Self::Loading)
    }

    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Ready)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}
