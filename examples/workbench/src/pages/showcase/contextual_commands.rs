use leptos::prelude::*;
use rs_design::ui::command::{Command, CommandCallback, CommandGroup, CommandRegistry};
use super::selection_context::SelectionContext;

/// Creates contextual commands based on current selection
pub fn create_contextual_commands(
    context: Signal<SelectionContext>,
    on_action: Callback<String>,
) -> Signal<CommandRegistry> {
    Signal::derive(move || {
        let ctx = context.get();
        let mut registry = CommandRegistry::new();
        
        // Commands available when a step is selected
        if ctx.is_type("step") {
            registry = registry.add_group(
                CommandGroup::new("step-actions", "Step Actions")
                    .add_command(Command {
                        id: "step.complete".into(),
                        label: format!("Complete: {}", ctx.label.as_deref().unwrap_or("Unknown")),
                        group: Some("Step Actions".into()),
                        icon: Some("✅".into()),
                        shortcut: Some("Ctrl+Enter".into()),
                        callback: CommandCallback::new({
                            let id = ctx.selected_id.clone().unwrap_or_default();
                            move || on_action.run(format!("complete:{}", id))
                        }),
                    })
                    .add_command(Command {
                        id: "step.fail".into(),
                        label: format!("Fail: {}", ctx.label.as_deref().unwrap_or("Unknown")),
                        group: Some("Step Actions".into()),
                        icon: Some("❌".into()),
                        shortcut: Some("Ctrl+F".into()),
                        callback: CommandCallback::new({
                            let id = ctx.selected_id.clone().unwrap_or_default();
                            move || on_action.run(format!("fail:{}", id))
                        }),
                    })
                    .add_command(Command {
                        id: "step.reset".into(),
                        label: format!("Reset: {}", ctx.label.as_deref().unwrap_or("Unknown")),
                        group: Some("Step Actions".into()),
                        icon: Some("🔄".into()),
                        shortcut: None,
                        callback: CommandCallback::new({
                            let id = ctx.selected_id.clone().unwrap_or_default();
                            move || on_action.run(format!("reset:{}", id))
                        }),
                    })
            );
        }
        
        // Commands available when a workflow is selected
        if ctx.is_type("workflow") {
            registry = registry.add_group(
                CommandGroup::new("workflow-actions", "Workflow Actions")
                    .add_command(Command {
                        id: "workflow.add_step".into(),
                        label: "Add Step".into(),
                        group: Some("Workflow Actions".into()),
                        icon: Some("➕".into()),
                        shortcut: None,
                        callback: CommandCallback::new({
                            let id = ctx.selected_id.clone().unwrap_or_default();
                            move || on_action.run(format!("add_step:{}", id))
                        }),
                    })
                    .add_command(Command {
                        id: "workflow.archive".into(),
                        label: "Archive Workflow".into(),
                        group: Some("Workflow Actions".into()),
                        icon: Some("📦".into()),
                        shortcut: None,
                        callback: CommandCallback::new({
                            let id = ctx.selected_id.clone().unwrap_or_default();
                            move || on_action.run(format!("archive:{}", id))
                        }),
                    })
                    .add_command(Command {
                        id: "workflow.export".into(),
                        label: "Export Report".into(),
                        group: Some("Workflow Actions".into()),
                        icon: Some("📄".into()),
                        shortcut: None,
                        callback: CommandCallback::new({
                            let id = ctx.selected_id.clone().unwrap_or_default();
                            move || on_action.run(format!("export:{}", id))
                        }),
                    })
            );
        }
        
        // Commands available when a user is selected
        if ctx.is_type("user") {
            registry = registry.add_group(
                CommandGroup::new("user-actions", "User Actions")
                    .add_command(Command {
                        id: "user.edit".into(),
                        label: "Edit User".into(),
                        group: Some("User Actions".into()),
                        icon: Some("✏️".into()),
                        shortcut: None,
                        callback: CommandCallback::new({
                            let id = ctx.selected_id.clone().unwrap_or_default();
                            move || on_action.run(format!("edit_user:{}", id))
                        }),
                    })
                    .add_command(Command {
                        id: "user.disable".into(),
                        label: "Disable User".into(),
                        group: Some("User Actions".into()),
                        icon: Some("🚫".into()),
                        shortcut: None,
                        callback: CommandCallback::new({
                            let id = ctx.selected_id.clone().unwrap_or_default();
                            move || on_action.run(format!("disable_user:{}", id))
                        }),
                    })
            );
        }
        
        // Global commands (always available)
        registry = registry.add_group(
            CommandGroup::new("navigation", "Navigation")
                .add_command(Command {
                    id: "nav.dashboard".into(),
                    label: "Go to Dashboard".into(),
                    group: Some("Navigation".into()),
                    icon: Some("🏠".into()),
                    shortcut: Some("Ctrl+H".into()),
                    callback: CommandCallback::new(move || {
                        on_action.run("nav:dashboard".into())
                    }),
                })
                .add_command(Command {
                    id: "nav.settings".into(),
                    label: "Go to Settings".into(),
                    group: Some("Navigation".into()),
                    icon: Some("⚙️".into()),
                    shortcut: Some("Ctrl+,".into()),
                    callback: CommandCallback::new(move || {
                        on_action.run("nav:settings".into())
                    }),
                })
        );
        
        registry
    })
}
