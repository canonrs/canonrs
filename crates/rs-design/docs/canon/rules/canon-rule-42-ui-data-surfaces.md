# Canon Rule #42: UI Data Surfaces

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-02

## Principle
Data source strategies (streaming, pagination, infinite scroll) are **adapters**, not components. A single primitive (`VirtualTable`) handles all rendering, while adapters provide data.

## The Problem

### ❌ WRONG: Strategy as Component
```
ui/
├── virtual_table/     // For client-side Vec<T>
├── streaming_table/   // For server streaming
├── paginated_table/   // For page/limit API
└── infinite_table/    // For cursor-based
```

**Why this fails:**
- 4 components doing the same job (render rows)
- Duplicate viewport logic
- Duplicate row rendering
- Duplicate column handling
- Different APIs for same concept
- Forces domain code to know infrastructure

### ✅ CORRECT: Strategy as Adapter
```
ui/
└── virtual_table/
    ├── mod.rs
    ├── virtual_table.rs      // Single renderer
    ├── adapters/
    │   ├── client_vec.rs     // Vec<T> adapter
    │   ├── server_paged.rs   // page/limit adapter
    │   ├── server_stream.rs  // streaming adapter
    │   └── server_cursor.rs  // cursor-based adapter
    ├── viewport.rs
    ├── row.rs
    └── column.rs
```

**Why this works:**
- Single component renders ALL strategies
- Adapters implement `DataSource` trait
- Domain code agnostic to data strategy
- Change strategy without changing UI
- Test rendering independently of data

## Rule Contract

### Component Classification

| Layer | What Goes Here | Never Contains |
|-------|---------------|----------------|
| `ui/` | Rendering primitives (table, grid, list) | Domain knowledge, data fetching |
| `components/` | Domain-aware compositions (workflow, users) | Rendering logic, viewport math |
| `blocks/` | Opinionated layouts (dashboard_card) | Data sources, business rules |

### Data Surface Pattern
```rust
// ui/virtual_table/mod.rs

/// Generic data source trait
pub trait DataSource<T> {
    fn total_count(&self) -> usize;
    fn fetch_range(&self, start: usize, end: usize) -> Vec<T>;
    fn on_scroll(&mut self, position: usize);
}

/// Single renderer for ALL strategies
#[component]
pub fn VirtualTable<T, D>(
    data_source: D,
    render_row: impl Fn(T) -> View + 'static,
) -> impl IntoView 
where
    D: DataSource<T>
{
    // Viewport calculation
    // Row rendering
    // Scroll handling
    // SAME for all strategies
}
```

### Adapter Examples

#### Client Vec Adapter
```rust
// ui/virtual_table/adapters/client_vec.rs

pub struct ClientVecSource<T> {
    data: Vec<T>,
}

impl<T: Clone> DataSource<T> for ClientVecSource<T> {
    fn total_count(&self) -> usize {
        self.data.len()
    }
    
    fn fetch_range(&self, start: usize, end: usize) -> Vec<T> {
        self.data[start..end].to_vec()
    }
    
    fn on_scroll(&mut self, _position: usize) {
        // No-op for client data
    }
}
```

#### Server Stream Adapter
```rust
// ui/virtual_table/adapters/server_stream.rs

pub struct ServerStreamSource<T> {
    fetched: Vec<T>,
    total_estimate: usize,
    fetch_fn: Box<dyn Fn(usize, usize) -> Future<Vec<T>>>,
}

impl<T> DataSource<T> for ServerStreamSource<T> {
    fn total_count(&self) -> usize {
        self.total_estimate
    }
    
    fn fetch_range(&self, start: usize, end: usize) -> Vec<T> {
        // Return cached or trigger fetch
        if start >= self.fetched.len() {
            // Trigger async fetch (via signal)
        }
        self.fetched[start..end].to_vec()
    }
    
    fn on_scroll(&mut self, position: usize) {
        // Prefetch next page
        if position > self.fetched.len() * 0.8 {
            // Trigger fetch
        }
    }
}
```

## Domain Usage

### Workflow Component (Correct)
```rust
// components/workflow/mod.rs

use rs_design::ui::virtual_table::{VirtualTable, adapters::ServerStreamSource};

#[component]
pub fn WorkflowAuditLog() -> impl IntoView {
    let data_source = ServerStreamSource::new(
        fetch_audit_entries, // Server function
        1000, // Estimated total
    );
    
    view! {
        <VirtualTable
            data_source=data_source
            render_row=|entry| view! {
                <div>{entry.timestamp} - {entry.action}</div>
            }
        />
    }
}
```

**Key points:**
- Workflow knows WHAT to display (audit entries)
- Workflow knows WHERE data comes from (server)
- Workflow does NOT know HOW to render viewport
- VirtualTable handles ALL rendering

## Migration Path

### Phase 1: Identify Duplicates
```sql
-- Find components doing same job
SELECT name, location, line_count
FROM components
WHERE name LIKE '%table%'
  AND has_viewport_logic = true;
```

### Phase 2: Extract Adapters
```bash
# For each duplicate table component:
1. Extract data fetching → adapter
2. Keep only rendering → primitives
3. Delete duplicate component
4. Update imports
```

### Phase 3: Consolidate
```
BEFORE:
- components/tokens_table.rs (300 lines)
- components/users_table.rs (280 lines)  
- components/audit_table.rs (320 lines)

AFTER:
- ui/virtual_table/ (200 lines)
- components/tokens.rs (50 lines, uses VirtualTable)
- components/users.rs (50 lines, uses VirtualTable)
- components/audit.rs (50 lines, uses VirtualTable)
```

**Result:** 900 lines → 350 lines

## Violation Examples

### Violation 1: Domain in UI
```rust
// ❌ WRONG: ui/tokens_table.rs
#[component]
pub fn TokensTable() -> impl IntoView {
    let tokens = Resource::new(|| (), |_| fetch_tokens()); // Domain!
    
    view! {
        <div class="viewport">
            {/* rendering */}
        </div>
    }
}
```

**Fix:**
```rust
// ✅ CORRECT: components/tokens.rs
use rs_design::ui::virtual_table::VirtualTable;

#[component]
pub fn TokensView() -> impl IntoView {
    let source = ServerStreamSource::new(fetch_tokens, 100);
    view! { <VirtualTable data_source=source /> }
}
```

### Violation 2: Rendering in Components
```rust
// ❌ WRONG: components/workflow.rs
#[component]
pub fn WorkflowSteps() -> impl IntoView {
    view! {
        <div class="viewport" style=format!("height: {}px", viewport_height)>
            <div style=format!("transform: translateY({}px)", offset)>
                {/* manual viewport math */}
            </div>
        </div>
    }
}
```

**Fix:**
```rust
// ✅ CORRECT: components/workflow.rs
use rs_design::ui::virtual_table::VirtualTable;

#[component]
pub fn WorkflowSteps() -> impl IntoView {
    view! { <VirtualTable data_source=steps_source /> }
}
```

## Testing Strategy

### Test Adapters Independently
```rust
#[test]
fn client_vec_adapter() {
    let data = vec![1, 2, 3, 4, 5];
    let source = ClientVecSource::new(data);
    
    assert_eq!(source.total_count(), 5);
    assert_eq!(source.fetch_range(1, 3), vec![2, 3]);
}

#[test]
fn server_stream_adapter() {
    let source = ServerStreamSource::new(mock_fetch, 100);
    
    // Test prefetch logic
    source.on_scroll(80);
    assert!(source.prefetch_triggered);
}
```

### Test Renderer Independently
```rust
#[test]
fn virtual_table_rendering() {
    let source = MockDataSource::new(vec![1, 2, 3]);
    
    let rendered = mount_to_body(|| view! {
        <VirtualTable
            data_source=source
            render_row=|n| view! { <div>{n}</div> }
        />
    });
    
    assert_eq!(rendered.find_all("div").len(), 3);
}
```

## Database Tracking
```sql
CREATE TABLE ui_primitives (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    category TEXT CHECK(category IN ('table', 'grid', 'list', 'form')),
    has_adapters BOOLEAN DEFAULT 0,
    adapter_count INTEGER DEFAULT 0,
    used_by_components INTEGER DEFAULT 0
);

CREATE TABLE component_dependencies (
    component_id INTEGER REFERENCES components(id),
    primitive_id INTEGER REFERENCES ui_primitives(id),
    adapter_type TEXT, -- 'client_vec', 'server_stream', etc.
    PRIMARY KEY (component_id, primitive_id)
);
```

## Adapter Registry
```rust
// ui/virtual_table/adapters/mod.rs

pub mod client_vec;
pub mod server_paged;
pub mod server_stream;
pub mod server_cursor;

pub use client_vec::ClientVecSource;
pub use server_paged::ServerPagedSource;
pub use server_stream::ServerStreamSource;
pub use server_cursor::ServerCursorSource;

/// Adapter capabilities matrix
pub const ADAPTERS: &[AdapterInfo] = &[
    AdapterInfo {
        name: "ClientVec",
        ssr_safe: true,
        supports_infinite: false,
        requires_server: false,
    },
    AdapterInfo {
        name: "ServerStream",
        ssr_safe: true,
        supports_infinite: true,
        requires_server: true,
    },
    // ...
];
```

## Related Rules

- [Canon Rule #3: Lists](./canon-rule-03-lists.md) - List rendering patterns
- [Canon Rule #14: DataTable vs VirtualTable](./canon-rule-14-datatable-vs-virtualtable.md) - When to virtualize
- [Canon Rule #15: Pagination vs Virtualization](./canon-rule-15-pagination-vs-virtualization.md) - Strategy selection
- [Canon Rule #16: Client vs Server Filtering](./canon-rule-16-client-vs-server-filtering.md) - Data location

## Summary

**Golden Rule:** Data strategy is configuration, not identity.

**Pattern:**
```
ONE renderer + MANY adapters > MANY renderers
```

**Architecture:**
```
ui/          → HOW to render
components/  → WHAT to render  
adapters/    → WHERE data comes from
```

**Never:**
- Duplicate viewport logic
- Mix domain and rendering
- Create component per data source

**Always:**
- Single renderer per primitive type
- Adapters implement standard trait
- Components choose adapter, not renderer

---

**Enforcement:** Linter + Architecture review  
**Violation:** Creating `X_table` for data strategy X  
**Correct:** Creating `XAdapter` for `VirtualTable`

---

**Last Updated:** 2025-01-02  
**Next Review:** 2025-Q2
