# Canon Rule #144 — Providers Expose State, Applications Own Interaction

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-01-16

**Category:** state-reactivity
**Tags:** providers, state, architecture, ui
**Language:** EN

---

**Intro:**
Embedding interaction logic inside providers creates coupling and breaks ownership boundaries. Providers must expose state only, leaving interaction decisions to the application layer.

**Problem:**
providers include interaction logic causing coupling and ownership violations

**Solution:**
limit providers to state exposure and mutation apis only

**Signals:**
- provider renders ui
- implicit interaction
- state coupling

**Search Intent:**
how to separate provider state and ui logic

**Keywords:**
provider state separation pattern, leptos provider architecture, state vs interaction boundary, context provider design rules

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