# Rule Capsule #27: Hydration Determinism


**Decision Trigger:**
If hydration mismatch errors appear in console, apply this rule immediately.

**Essence:** Server HTML must match client hydration exactly.

**Evita:**
- Math.random() for IDs
- Date.now() without sync
- HashMap iteration (non-deterministic order)
- Unguarded window/document access

**Always think:**
"Will server and client render the same output?"

**Pattern:**
```rust
// ❌ WRONG
let id = format!("item-{}", js_sys::Math::random());

// ✅ CORRECT
let id = format!("item-{}", item.database_id);
```

**When to apply:** Hydration mismatch errors in console.
