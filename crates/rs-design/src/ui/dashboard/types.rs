use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Widget position in grid
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WidgetPosition {
    pub x: usize,      // Grid column (0-based)
    pub y: usize,      // Grid row (0-based)
    pub width: usize,  // Columns span
    pub height: usize, // Rows span
}

impl WidgetPosition {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self { x, y, width, height }
    }
}

/// Widget definition
#[derive(Clone, PartialEq, Debug)]
pub struct WidgetDef {
    pub id: String,
    pub title: String,
    pub position: WidgetPosition,
    pub content: WidgetContent,
    pub resizable: bool,
    pub removable: bool,
}

impl WidgetDef {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            position: WidgetPosition::new(0, 0, 1, 1),
            content: WidgetContent::Empty,
            resizable: true,
            removable: true,
        }
    }
    
    pub fn position(mut self, x: usize, y: usize, width: usize, height: usize) -> Self {
        self.position = WidgetPosition::new(x, y, width, height);
        self
    }
    
    pub fn content(mut self, content: WidgetContent) -> Self {
        self.content = content;
        self
    }
    
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }
    
    pub fn removable(mut self, removable: bool) -> Self {
        self.removable = removable;
        self
    }
}

/// Widget content types
#[derive()]
pub enum WidgetContent {
    Empty,
    Custom(fn() -> AnyView),
}

/// Dashboard configuration
#[derive(Clone, PartialEq, Debug)]
pub struct DashboardConfig {
    pub columns: usize,
    pub row_height: usize, // pixels
    pub gap: usize,        // pixels
    pub margin: usize,     // pixels
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            columns: 12,
            row_height: 60,
            gap: 16,
            margin: 16,
        }
    }
}

impl DashboardConfig {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn columns(mut self, cols: usize) -> Self {
        self.columns = cols;
        self
    }
    
    pub fn row_height(mut self, height: usize) -> Self {
        self.row_height = height;
        self
    }
    
    pub fn gap(mut self, gap: usize) -> Self {
        self.gap = gap;
        self
    }
}

/// Widget drag event (Canon Rule #49: Events, not actions)
#[derive(Clone, Debug, PartialEq)]
pub struct WidgetDragEvent {
    pub widget_id: String,
    pub from_position: WidgetPosition,
    pub to_position: WidgetPosition,
}

/// Widget resize event
#[derive(Clone, Debug, PartialEq)]
pub struct WidgetResizeEvent {
    pub widget_id: String,
    pub from_size: (usize, usize), // (width, height)
    pub to_size: (usize, usize),
}

/// Widget remove event
#[derive(Clone, Debug, PartialEq)]
pub struct WidgetRemoveEvent {
    pub widget_id: String,
}

impl PartialEq for WidgetContent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Empty, Self::Empty) => true,
            (Self::Custom(_), Self::Custom(_)) => false, // fn pointers can't be compared
            _ => false,
        }
    }
}

impl Clone for WidgetContent {
    fn clone(&self) -> Self {
        match self {
            Self::Empty => Self::Empty,
            Self::Custom(f) => Self::Custom(*f), // Copy the fn pointer
        }
    }
}

impl std::fmt::Debug for WidgetContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "WidgetContent::Empty"),
            Self::Custom(_) => write!(f, "WidgetContent::Custom(<fn>)"),
        }
    }
}
