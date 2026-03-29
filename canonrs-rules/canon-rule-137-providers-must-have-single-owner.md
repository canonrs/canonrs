# Canon Rule #137: Providers Must Have a Single Owner

**Status:** ENFORCED
**Severity:** HIGH
**Scope:** state, providers
**Version:** 1.0.0
**Date:** 2025-01-23

---

## Principle

**Every Provider MUST have exactly one owning component responsible for its lifecycle.**

---

## The Problem

When Providers are instantiated in multiple places:

- State becomes duplicated
- Signals diverge silently
- UI desynchronizes
- Debugging becomes impossible
- Ownership is unclear

This was observed when Providers were placed in both Layouts and Blocks.

---

## Forbidden Patterns

### ❌ Forbidden
```rust
// Provider inside a Block
#[component]
pub fn SidebarBlock(children: Children) -> impl IntoView {
    view! {
        <SidebarProvider>
            {children()}
        </SidebarProvider>
    }
}
```

Blocks MUST NOT own Providers.

---

## Canonical Pattern

### ✅ Canonical
```rust
// Provider owned by App-level composition
<SidebarProvider>
    <Sidebar />
    <SidebarInset />
</SidebarProvider>
```

```rust
// Blocks consume context only
#[component]
pub fn SidebarBlock(children: Children) -> impl IntoView {
    view! {
        <Sidebar>{children()}</Sidebar>
    }
}
```

---

## Rationale

Providers define **state boundaries**.

A single owner ensures:
- Predictable state flow
- Deterministic reactivity
- Clear architectural layering
- Compatibility with SSR/CSR split

This is a core invariant of Canon UI architecture.

---

## Enforcement

- CI: forbid Provider instantiation in Blocks
- Static analysis: detect duplicated Providers
- Code review ownership checks

---

## Exceptions

No exceptions. This rule is absolute.

---

## Version History

- **1.0.0** — Initial canonical version
