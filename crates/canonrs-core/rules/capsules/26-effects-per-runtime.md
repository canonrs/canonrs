# Rule Capsule #26: Effects Per Runtime


**Decision Trigger:**
If effects seem to "run twice" or logs duplicate, apply this rule immediately.

**Essence:** Effects execute once PER RUNTIME, not once total.

**Evita:**
- Hydration mismatch
- Duplicate logs confusing debug
- Hacks with static flags

**Always think:**
"This effect runs in server AND browser. Is that intended?"

**Pattern:**
```rust
create_effect(move |_| {
    // Runs in: browser runtime only
    // Does NOT run in: server
});
```

**When to apply:** Effect seems to "run twice" or logs duplicate.
