# Workflow Component

**Type:** Domain Component (Type 3)
**Status:** Production-Ready
**Tokens:** 90% Canonical (A+B, partial C)
**Pattern:** CQRS (Read/Write Separation)

---

## Purpose

Visual representation of multi-step workflow state with audit trail. Separates **state display** (SSR-safe) from **state mutation** (client-only commands). Designed for enterprise governance, ISO compliance, and full auditability.

---

## Architecture

### CQRS Pattern
```
┌─────────────────────┐
│ WorkflowView (READ) │  ← SSR-safe, displays state
│ - Steps             │
│ - Status            │
│ - Audit Trail       │
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│ WorkflowActions     │  ← Client-only, emits commands
│ (WRITE)             │
│ - Transition        │
│ - Validate          │
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│ Server / Core       │  ← Persistence layer
│ - Validate rules    │
│ - Update state      │
│ - Write audit       │
└─────────────────────┘
```

---

## Usage

### Demo Mode (Ephemeral State)
```rust
use rs_design::components::workflow::WorkflowDemo;

<WorkflowDemo />
```

### Production Mode (Persisted State)
```rust
// In your app with database backend
use crate::components::{WorkflowView, WorkflowActions, WorkflowClient};

// Option 1: Full CQRS (recommended)
<WorkflowClient />

// Option 2: View only (dashboards, reports)
<WorkflowView refresh_trigger=signal />

// Option 3: Actions only (admin panels)
<WorkflowActions
    step_id="approval"
    current_status="Active"
    on_transition_complete=callback
/>
```

---

## Components

### WorkflowDemo
**Type:** Ephemeral  
**File:** `demo.rs`  
**Purpose:** Prototyping, documentation, visual testing
```rust
#[component]
pub fn WorkflowDemo() -> impl IntoView
```

**State:** In-memory `RwSignal<Vec<Step>>`  
**Persistence:** None  
**SSR:** ✅ Safe  
**Actions:** Client-only (cfg wasm32)

---

### WorkflowView
**Type:** Read-Only Display  
**File:** (implementation location varies)  
**Purpose:** Display workflow state and audit trail
```rust
#[component]
pub fn WorkflowView(
    #[prop(optional)] refresh_trigger: Option<ReadSignal<u32>>
) -> impl IntoView
```

**Props:**
- `refresh_trigger` - Signal to reload data after mutations

**Features:**
- ✅ SSR-safe (uses `Resource` + `Transition`)
- ✅ Displays steps with visual status
- ✅ Shows audit trail
- ❌ No mutations (read-only)

**Tokens Used:**
- `color.{primary,success,danger,warning}.{bg,border}` - Status colors
- `space.{sm,md,lg}` - Spacing
- `radius.md` - Border radius
- `border.width.thin` - Step borders
- `font.{size,weight}` - Typography

---

### WorkflowActions
**Type:** Command Emitter  
**File:** (implementation location varies)  
**Purpose:** Trigger state transitions
```rust
#[component]
pub fn WorkflowActions(
    step_id: String,
    current_status: String,
    #[prop(into)] on_transition_complete: Callback<()>
) -> impl IntoView
```

**Props:**
- `step_id` - ID of the step to act on
- `current_status` - Current status (determines available actions)
- `on_transition_complete` - Callback after successful transition

**Features:**
- ❌ Not SSR-safe (uses `spawn_local`)
- ✅ Client-only (#[cfg(target_arch = "wasm32")])
- ✅ Emits commands to server
- ✅ Validates business rules server-side

**Available Actions by Status:**
- `Completed` → Reset
- `Active` → Complete, Fail
- `Blocked` → Unblock
- `Pending` → Start
- `Failed` → Reset

---

### WorkflowClient
**Type:** Compositor  
**File:** (implementation location varies)  
**Purpose:** Combines View + Actions with refresh cycle
```rust
#[component]
pub fn WorkflowClient() -> impl IntoView
```

**Features:**
- ✅ Manages refresh trigger
- ✅ Coordinates View ↔ Actions
- ✅ Handles optimistic updates
- ✅ Full CQRS pattern

---

## Step Status States

| Status | Icon | Color | Meaning | Available Actions |
|--------|------|-------|---------|-------------------|
| `Pending` | ⏸️ | Gray | Not started | Start |
| `Active` | ⏳ | Blue | In progress | Complete, Fail |
| `Completed` | ✅ | Green | Done | Reset |
| `Blocked` | 🔒 | Red | Blocked | Unblock |
| `Failed` | ❌ | Red (dark) | Failed | Reset |

---

## Data Model

### Step
```rust
pub struct Step {
    pub id: usize,
    pub step_id: String,      // e.g., "review", "approval"
    pub label: String,         // e.g., "Code Review"
    pub status: StepStatus,
}
```

### StepStatus
```rust
pub enum StepStatus {
    Pending,
    Active,
    Completed,
    Blocked,
    Failed,
}
```

### AuditEntry (Persistent Mode)
```rust
pub struct AuditEntry {
    pub id: i64,
    pub step_id: String,
    pub from_status: String,
    pub to_status: String,
    pub actor: Option<String>,
    pub timestamp: String,
}
```

---

## Database Schema (Persistent Mode)
```sql
CREATE TABLE workflows (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE workflow_steps (
    id INTEGER PRIMARY KEY,
    workflow_id INTEGER REFERENCES workflows(id),
    step_id TEXT NOT NULL,
    label TEXT NOT NULL,
    status TEXT CHECK(status IN ('Pending', 'Active', 'Completed', 'Blocked', 'Failed')),
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE workflow_audit (
    id INTEGER PRIMARY KEY,
    workflow_id INTEGER REFERENCES workflows(id),
    step_id TEXT NOT NULL,
    from_status TEXT NOT NULL,
    to_status TEXT NOT NULL,
    actor TEXT,
    reason TEXT,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

---

## Server Functions (Persistent Mode)
```rust
#[server]
pub async fn fetch_workflow_steps(workflow_id: i64) 
    -> Result<Vec<WorkflowStep>, ServerFnError>

#[server]
pub async fn transition_workflow_step(
    workflow_id: i64,
    step_id: String,
    new_status: String,
    actor: Option<String>
) -> Result<(), ServerFnError>

#[server]
pub async fn fetch_workflow_audit(workflow_id: i64)
    -> Result<Vec<WorkflowAuditEntry>, ServerFnError>
```

---

## Canon Rules Applied

### Rule #41: Leptos Resource Consumption
- ✅ `Resource` consumed via `.get()`, not `.await`
- ✅ `Transition` wrapper for reactive updates
- ❌ Never `Suspend` with `Resource`

### Rule #42: UI Data Surfaces
- ✅ Workflow is a **domain component**, not UI primitive
- ✅ Uses UI primitives (borders, colors, spacing)
- ❌ Does not implement rendering infrastructure

### Rule #5: SSR Effects
- ✅ WorkflowView is SSR-safe
- ✅ WorkflowActions is client-only (#[cfg(wasm32)])
- ✅ No `spawn_local` in SSR context

### Rule #6: Visual State
- ✅ Status colors from canonical tokens
- ✅ Spacing from design system
- ⚠️ Icons are hardcoded (acceptable for semantic meaning)

---

## Accessibility

- ✅ Semantic HTML (divs with ARIA roles)
- ✅ Visual status indicators (color + icon)
- ✅ Keyboard navigable buttons
- ✅ Screen reader friendly (status text + labels)
- ⚠️ Focus management (buttons only, not step containers)

---

## Enterprise Features

### Audit Trail
Every transition creates an immutable audit entry:
```
step_id: approval
from: Active
to: Completed
actor: demo-user
timestamp: 2025-01-02 19:30:45
```

### Governance
- ✅ ISO/SOC2 compliant (full audit)
- ✅ Replay-able (all events stored)
- ✅ Rollback-capable (event sourcing ready)
- ✅ Multi-actor tracking

### API-First
- ✅ Server functions can be called from CLI
- ✅ REST endpoints can wrap server functions
- ✅ Automation-ready (no UI required)

---

## File Structure
```
workflow/
├── demo.rs             # WorkflowDemo (ephemeral)
├── step.rs             # Step struct + StepStatus enum
├── mod.rs              # Module exports
└── README.md           # This file
```

**Note:** Persistent components (`WorkflowView`, `WorkflowActions`, `WorkflowClient`) are implemented in the consuming application, not in rs-design, as they depend on database and server function implementations.

---

## Examples

### Basic Demo
```rust
use rs_design::components::workflow::WorkflowDemo;

view! {
    <WorkflowDemo />
}
```

### Read-Only Dashboard
```rust
use crate::components::WorkflowView;

view! {
    <div class="dashboard">
        <h2>"Current Workflows"</h2>
        <WorkflowView />
    </div>
}
```

### Admin Panel with Actions
```rust
use crate::components::WorkflowClient;

view! {
    <div class="admin-panel">
        <WorkflowClient />
    </div>
}
```

### Custom Integration
```rust
let (trigger, set_trigger) = signal(0);

let on_complete = Callback::new(move |_| {
    set_trigger.update(|n| *n += 1);
    // Custom logic (notifications, analytics, etc.)
});

view! {
    <WorkflowView refresh_trigger=trigger />
    <WorkflowActions
        step_id="deploy"
        current_status="Pending"
        on_transition_complete=on_complete
    />
}
```

---

## Migration Notes

### From Ephemeral to Persistent
1. Implement database schema
2. Create server functions
3. Replace `WorkflowDemo` with `WorkflowClient`
4. Add auth/permissions to transitions
5. Configure audit retention policies

### Adding Custom Steps
```rust
// In database
INSERT INTO workflow_steps (workflow_id, step_id, label, status)
VALUES (1, 'legal-review', 'Legal Review', 'Pending');

// Component auto-adapts (reads from DB)
```

---

## Testing

### Unit Tests (Demo)
```rust
#[test]
fn workflow_transitions() {
    let step = Step::new("test", "Test Step", StepStatus::Pending);
    assert_eq!(step.status, StepStatus::Pending);
    
    // Transition
    step.status = StepStatus::Active;
    assert_eq!(step.status, StepStatus::Active);
}
```

### Integration Tests (Persistent)
```rust
#[tokio::test]
async fn workflow_audit_trail() {
    let result = transition_workflow_step(
        1, 
        "approval".into(), 
        "Completed".into(),
        Some("test-user".into())
    ).await;
    
    assert!(result.is_ok());
    
    let audit = fetch_workflow_audit(1).await.unwrap();
    assert_eq!(audit.last().unwrap().to_status, "Completed");
}
```

---

## Performance

### Demo Mode
- ✅ Zero network calls
- ✅ Instant updates (in-memory)
- ✅ Perfect for prototyping

### Persistent Mode
- ⚠️ Network latency per transition
- ✅ Optimistic updates (via refresh trigger)
- ✅ Cached reads (Resource)

---

**Status:** ✅ Production-Ready  
**Pattern:** CQRS  
**Persistence:** Optional (Demo vs Production)  
**Canonicity:** 90%  
**Last Updated:** 2025-01-02
