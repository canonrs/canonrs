# CanonRS — Operating Model

CanonRS is an enterprise-grade design system and runtime framework for Rust/Leptos applications.

## Core Principles (Immutable)

- **Determinism > Convenience** - Predictable behavior always wins
- **Fail Fast** - Errors at compile time, not runtime
- **Explicit > Magic** - No hidden behavior
- **SSR-First** - Server rendering with hydration safety
- **Zero-Trust** - Security by design, not by configuration

## Canon Rules Organization

Rules are numbered and grouped by domain:

- **#01-#09**: Rust Core & Build
- **#10-#19**: Docker & Containers
- **#20-#29**: Leptos Runtime & SSR
- **#30+**: UI, Themes, Security

Each rule is **self-contained** and **enforceable**.

## How to Use (Human or AI)

### For Developers
1. Read rule headers (TL;DR) for quick understanding
2. Cite rules by number in discussions
3. When blocked: identify which rule applies
4. If no rule exists: propose one

### For AI Assistants
When activated in CanonRS mode:
- **Never suggest hacks** - Find the correct pattern
- **Never mix paradigms** - Follow established rules
- **Never break previous rules** - Maintain consistency
- **Always cite rule number** - Make reasoning explicit

## Rule Header Format

Every Canon Rule follows this structure:
```markdown
# Canon Rule #XX — Title

## TL;DR
One-sentence essence

## When to Apply
Context where this rule is relevant

## Classic Symptom
How you know this rule was violated

## Decision
What the rule enforces

## Quick Reference
When to remember this rule
```

## Consultation Pattern

**Don't read all rules upfront** - Consult on-demand:

1. Start with this index
2. Identify relevant rule number
3. Read only that rule's header
4. Deep-dive only if needed

## Repository Structure

See `docs/ARCHITECTURE.md` for complete structure.

## Getting Started
```bash
# Clone
git clone https://github.com/your-org/canonrs

# Build
cargo build --workspace

# Run workbench
cd examples/workbench
trunk serve
```

## For New Chat Sessions

Recommended prompt:
```
You are operating under CanonRS.
Read only docs/canonrs/index.md initially.
Consult Canon Rules on-demand by number.
Follow enterprise patterns, not hacks.
```
