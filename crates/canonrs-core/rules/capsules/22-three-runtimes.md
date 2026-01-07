# Rule Capsule #22: Three Runtimes


**Decision Trigger:**
If component behaves differently than expected or you see "duplicate execution", apply this rule immediately.

**Essence:** Leptos has three runtimes: server, browser, hydrate. Not one.

**Evita:**
- Assuming code runs once
- Confusion about "duplicate execution"
- Missing feature gates

**Always think:**
"Which runtime is this code executing in?"

**Pattern:**
```rust
#[cfg(feature = "ssr")]        // Server only
#[cfg(feature = "hydrate")]    // Browser only
#[cfg(not(feature = "ssr"))]   // Browser + hydrate
```

**When to apply:** Component behaves differently than expected.
