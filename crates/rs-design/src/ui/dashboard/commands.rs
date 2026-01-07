use crate::commands::Command;
use super::types::{WidgetDef, WidgetPosition};
use leptos::prelude::*;

#[derive(Debug, Clone)]
pub struct MoveWidgetCommand {
    pub widgets: RwSignal<Vec<WidgetDef>>,
    pub widget_id: String,
    pub from_position: WidgetPosition,
    pub to_position: WidgetPosition,
}

impl Command for MoveWidgetCommand {
    fn execute(&self) {
        self.widgets.update(|ws| {
            if let Some(widget) = ws.iter_mut().find(|w| w.id == self.widget_id) {
                widget.position = self.to_position.clone();
            }
        });
    }
    
    fn undo(&self) {
        self.widgets.update(|ws| {
            if let Some(widget) = ws.iter_mut().find(|w| w.id == self.widget_id) {
                widget.position = self.from_position.clone();
            }
        });
    }
    
    fn description(&self) -> String {
        format!("Move widget {} to ({}, {})", 
            self.widget_id, self.to_position.x, self.to_position.y)
    }
}

#[derive(Debug, Clone)]
pub struct RemoveWidgetCommand {
    pub widgets: RwSignal<Vec<WidgetDef>>,
    pub widget: WidgetDef,
    pub index: usize,
}

impl Command for RemoveWidgetCommand {
    fn execute(&self) {
        self.widgets.update(|ws| {
            ws.retain(|w| w.id != self.widget.id);
        });
    }
    
    fn undo(&self) {
        self.widgets.update(|ws| {
            ws.insert(self.index, self.widget.clone());
        });
    }
    
    fn description(&self) -> String {
        format!("Remove widget {}", self.widget.title)
    }
}

#[derive(Debug, Clone)]
pub struct AddWidgetCommand {
    pub widgets: RwSignal<Vec<WidgetDef>>,
    pub widget: WidgetDef,
}

impl Command for AddWidgetCommand {
    fn execute(&self) {
        self.widgets.update(|ws| ws.push(self.widget.clone()));
    }
    
    fn undo(&self) {
        self.widgets.update(|ws| {
            ws.retain(|w| w.id != self.widget.id);
        });
    }
    
    fn description(&self) -> String {
        format!("Add widget {}", self.widget.title)
    }
}
