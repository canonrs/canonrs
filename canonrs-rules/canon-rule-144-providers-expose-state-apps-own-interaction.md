# Canon Rule #144 — Providers Expose State, Applications Own Interaction

**Status:** ENFORCED
**Severity:** HIGH
**Scope:** providers, state, architecture
**Version:** 1.0.0
**Date:** 2026-01-16

---
## Context
Providers are architectural primitives responsible for **state exposure**, not user interaction.
Embedding toggles, buttons, or UI logic inside Providers breaks ownership boundaries and causes coupling.

## Rule
Providers **MUST ONLY expose state and mutation APIs**.
They **MUST NOT** render interaction UI (toggles, buttons, menus).

## Mandatory Pattern
```rust
pub struct ThemeContext {
    pub mode: RwSignal<ThemeMode>,
}

impl ThemeContext {
    pub fn set_mode(&self, mode: ThemeMode) {
        self.mode.set(mode);
    }
}
```

UI decides *how* and *when* to call mutations.

## Forbidden Patterns
- `ThemeToggle` inside `ThemeProvider`
- Rendering buttons inside Providers
- Providers reacting to DOM events

## Rationale
- Enforces single ownership of behavior
- Keeps Providers SSR-safe
- Prevents implicit UX decisions
- Enables multiple UIs over the same state

## Scope
- Providers
- Context types

## Exception
None.

Providers expose state. Applications decide interaction.
