# Canon Rule #94: Leptos Workspace Features Must Be Explicit

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** leptos, workspace
**Version:** 1.0.0  
**Date:** 2025-01-15

---

## Principle

**Leptos features (especially `nightly`) MUST be declared explicitly in `[workspace.dependencies]`, not delegated to member crates.**

Without workspace-level feature declaration, the Leptos macro expansion fails with cascading E0282 "type annotations needed" errors in every `view!` macro, despite correct code.

This is a toolchain requirement, not a code quality issue.

---

## The Problem

When `features = ["nightly"]` is declared only in member crate dependencies:

- Leptos macros cannot infer types in closures
- Every `view!` block fails with E0282
- Error messages are misleading (point to view! code, not features)
- No compilation succeeds despite syntactically correct code

### Real Symptoms
```rust
view! {
    <Sidebar>
        <SidebarContent>
            {items.iter().map(|item| {  // ← E0282 here
                view! { <div>{item.label}</div> }
            }).collect::<Vec<_>>()}
        </SidebarContent>
    </Sidebar>
}
```

Error:
```
error[E0282]: type annotations needed
  --> src/components/sidebar.rs:12:21
   |
12 |                     view! {
   |                     ^^^^^^ cannot infer type
```

---

## Anti-Pattern (FORBIDDEN)
```toml
# ❌ FORBIDDEN: Features only in member crate
[workspace.dependencies]
leptos = "0.8"  # NO FEATURES

# shell/Cargo.toml
[dependencies]
leptos = { workspace = true, features = ["nightly"] }
```

This fails because macro expansion happens at workspace resolution time, before member features are applied.

---

## Canonical Pattern (REQUIRED)
```toml
# ✅ REQUIRED: Features in workspace root
[workspace.dependencies]
leptos = { version = "0.8", features = ["nightly"] }
leptos_router = { version = "0.8", features = ["nightly"] }

# Member crates inherit correctly
[dependencies]
leptos = { workspace = true }
leptos_router = { workspace = true }
```

---

## Why This Happens

Leptos `view!` macro uses advanced type inference that relies on:
1. Nightly Rust features (type system extensions)
2. Workspace-level feature resolution
3. Consistent feature flags across all crates

When `nightly` is not workspace-level:
- Macro sees stable Rust type system
- Cannot infer closure return types
- Cascades to all nested view! blocks

---

## Diagnostic Checklist

If you see E0282 in view! macros:
```bash
# 1. Check workspace features
grep -A 3 "leptos.*=" Cargo.toml

# 2. Verify nightly is present
# Should see: features = ["nightly"]

# 3. Clean and rebuild
cargo clean
cargo build

# 4. If still failing, check rust-toolchain
cat rust-toolchain.toml  # Should be "nightly"
```

---

## Enforcement

- All Leptos workspaces MUST declare `features = ["nightly"]` in workspace root
- Member crates MUST NOT redeclare features (inherit from workspace)
- CI MUST fail if workspace.dependencies lacks nightly feature
- New projects MUST use canonical template with correct feature declaration

---

## Canonical Justification

> **Macro expansion is workspace-scoped, not crate-scoped.**  
> Feature resolution happens before member crate dependencies are processed.  
> This is Cargo's designed behavior, not a Leptos bug.

This rule exists to:
- Eliminate 2+ hours of debugging time per setup
- Encode Cargo's workspace feature resolution as governance
- Prevent misdiagnosis of "type inference bugs"
- Align with Leptos 0.8's architecture assumptions

---

## Related Canon Rules

- Canon Rule #93 — Leptos WASM Dev Builds Must Use Release Mode
- Canon Rule #95 — SSR Requires Complete HTML Shell
- Canon Rule #96 — SSR Requires Explicit Provider Tree

---

## Version History

- **1.0.0** (2025-01-15): Initial rule based on Workbench migration E0282 debugging
