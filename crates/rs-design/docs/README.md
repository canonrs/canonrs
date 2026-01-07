# RS-Design Documentation

**Enterprise SSR-safe component system for Leptos**

---

## 📖 Documentation Structure

### [Canon Rules](/docs/canon/)
**Mandatory architectural rules** - violations block PRs

11 normative rules that prevent production bugs and ensure SSR safety.

### [Architecture](/docs/architecture/)
**6-layer system design** - separation of concerns

Token → Primitive → UI → Block → Pattern → Page hierarchy.

### [Patterns](/docs/patterns/)
**Implementation guides** - how to apply Canon in practice

Real-world examples of clipboard integration, modal state, DataTable actions.

### [ADRs](/docs/adr/) *(Optional)*
**Architecture Decision Records** - why we made specific choices

Reserved for major design decisions.

---

## 🎯 Quick Start

**New to RS-Design?** Start here:

1. Read [Canon Rules Index](/docs/canon/README.md)
2. Understand [Architecture Overview](/docs/architecture/README.md)
3. Browse [Patterns](/docs/patterns/) for your use case

**Building components?** Check:
- [Component Types (Rule #1)](/docs/canon/01-types.md)
- [Ownership (Rule #2)](/docs/canon/02-ownership.md)
- [SSR Effects (Rule #5)](/docs/canon/05-ssr-effects.md)

**Having issues?** Common problems:
- Hydration errors → [Rule #4](/docs/canon/04-hydration.md)
- Overlay bugs → [Rule #8](/docs/canon/08-overlay-islands.md)
- Multiple callbacks → [Rule #11](/docs/canon/11-multi-callback-ownership.md)

---

## 🔒 Normative Status

All Canon Rules are **normative** (mandatory).  
Violations require explicit approval and ADR documentation.

---

**Maintained by:** RS-Design Core Team  
**Last Updated:** 2025-12-30
