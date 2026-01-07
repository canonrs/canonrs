use leptos::prelude::*;
use rs_design::ui::command::{Command, CommandCallback, CommandGroup, CommandRegistry};

#[cfg(target_arch = "wasm32")]
use super::database_server::transition_workflow_step;

/// Creates workflow command registry
/// 
/// **Client-only** - Commands execute async operations
pub fn create_workflow_commands(
    on_transition_complete: Callback<()>
) -> CommandRegistry {
    #[cfg(target_arch = "wasm32")]
    {
        CommandRegistry::new()
            .add_group(
                CommandGroup::new("workflow", "Workflow")
                    .add_command(Command {
                        id: "workflow.complete_step".into(),
                        label: "Complete Active Step".into(),
                        group: Some("Workflow".into()),
                        shortcut: Some("Ctrl+Enter".into()),
                        icon: Some("✅".into()),
                        callback: CommandCallback::new(move || {
                            use leptos::task::spawn_local;
                            let on_complete = on_transition_complete;
                            spawn_local(async move {
                                if transition_workflow_step(1, "approval".into(), "Completed".into(), Some("demo-user".into())).await.is_ok() {
                                    on_complete.run(());
                                }
                            });
                        }),
                    })
                    .add_command(Command {
                        id: "workflow.fail_step".into(),
                        label: "Fail Active Step".into(),
                        group: Some("Workflow".into()),
                        shortcut: Some("Ctrl+Shift+F".into()),
                        icon: Some("❌".into()),
                        callback: CommandCallback::new(move || {
                            use leptos::task::spawn_local;
                            let on_complete = on_transition_complete;
                            spawn_local(async move {
                                if transition_workflow_step(1, "approval".into(), "Failed".into(), Some("demo-user".into())).await.is_ok() {
                                    on_complete.run(());
                                }
                            });
                        }),
                    })
                    .add_command(Command {
                        id: "workflow.start_pending".into(),
                        label: "Start Pending Step".into(),
                        group: Some("Workflow".into()),
                        shortcut: Some("Ctrl+S".into()),
                        icon: Some("▶️".into()),
                        callback: CommandCallback::new(move || {
                            use leptos::task::spawn_local;
                            let on_complete = on_transition_complete;
                            spawn_local(async move {
                                if transition_workflow_step(1, "monitoring".into(), "Active".into(), Some("demo-user".into())).await.is_ok() {
                                    on_complete.run(());
                                }
                            });
                        }),
                    })
            )
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // SSR: empty registry
        CommandRegistry::new()
    }
}
