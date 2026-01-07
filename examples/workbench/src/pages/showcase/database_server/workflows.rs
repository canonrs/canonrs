use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowStep {
    pub id: i64,
    pub workflow_id: i64,
    pub step_id: String,
    pub label: String,
    pub status: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowAuditEntry {
    pub id: i64,
    pub workflow_id: i64,
    pub step_id: String,
    pub from_status: String,
    pub to_status: String,
    pub actor: Option<String>,
    pub reason: Option<String>,
    pub timestamp: String,
}

#[server]
pub async fn fetch_workflow_steps(workflow_id: i64) -> Result<Vec<WorkflowStep>, ServerFnError> {
    use rusqlite::Connection;
    let conn = Connection::open("/opt/docker/cores/monorepo-core/apps/core-auth/frontend-leptos/data/canonrs.db")?;
    let mut stmt = conn.prepare("SELECT id, workflow_id, step_id, label, status, updated_at FROM workflow_steps WHERE workflow_id = ? ORDER BY id")?;
    let steps = stmt.query_map([workflow_id], |row| {
        Ok(WorkflowStep {
            id: row.get(0)?,
            workflow_id: row.get(1)?,
            step_id: row.get(2)?,
            label: row.get(3)?,
            status: row.get(4)?,
            updated_at: row.get(5)?,
        })
    })?.collect::<Result<Vec<_>, _>>()?;
    Ok(steps)
}

#[server]
pub async fn transition_workflow_step(workflow_id: i64, step_id: String, new_status: String, actor: Option<String>) -> Result<(), ServerFnError> {
    use rusqlite::Connection;
    let conn = Connection::open("/opt/docker/cores/monorepo-core/apps/core-auth/frontend-leptos/data/canonrs.db")?;
    let old_status: String = conn.query_row("SELECT status FROM workflow_steps WHERE workflow_id = ? AND step_id = ?", [&workflow_id.to_string(), &step_id], |row| row.get(0))?;
    conn.execute("UPDATE workflow_steps SET status = ?, updated_at = datetime('now') WHERE workflow_id = ? AND step_id = ?", rusqlite::params![&new_status, workflow_id, &step_id])?;
    conn.execute("INSERT INTO workflow_audit (workflow_id, step_id, from_status, to_status, actor, timestamp) VALUES (?, ?, ?, ?, ?, datetime('now'))", rusqlite::params![workflow_id, &step_id, &old_status, &new_status, actor])?;
    leptos::logging::log!("✅ Transition: {} {} -> {}", step_id, old_status, new_status);
    Ok(())
}

#[server]
pub async fn fetch_workflow_audit(workflow_id: i64) -> Result<Vec<WorkflowAuditEntry>, ServerFnError> {
    use rusqlite::Connection;
    let conn = Connection::open("/opt/docker/cores/monorepo-core/apps/core-auth/frontend-leptos/data/canonrs.db")?;
    let mut stmt = conn.prepare("SELECT id, workflow_id, step_id, from_status, to_status, actor, reason, timestamp FROM workflow_audit WHERE workflow_id = ? ORDER BY timestamp DESC LIMIT 50")?;
    let entries = stmt.query_map([workflow_id], |row| {
        Ok(WorkflowAuditEntry {
            id: row.get(0)?,
            workflow_id: row.get(1)?,
            step_id: row.get(2)?,
            from_status: row.get(3)?,
            to_status: row.get(4)?,
            actor: row.get(5)?,
            reason: row.get(6)?,
            timestamp: row.get(7)?,
        })
    })?.collect::<Result<Vec<_>, _>>()?;
    Ok(entries)
}
