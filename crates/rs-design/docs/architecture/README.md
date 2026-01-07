# CanonRS Architecture Documentation

## Core Documents (Normative)

These documents define the architectural rules of CanonRS. Violations may block PRs.

### Foundation
1. **[ARCHITECTURE.md](../ARCHITECTURE.md)** - 6-Layer Design System
   - tokens → primitives → ui → blocks → layouts → pages
   - Separation: Design ≠ Logic ≠ Business

### Canon Rules (src/rules/)
2. **[types.md](../../src/rules/types.md)** - Canon Rule #1: Component Types
   - Type 1 (Pure), Type 2 (Stateful), Type 3 (Interactive)

3. **[ownership.md](../../src/rules/ownership.md)** - Canon Rule #2: Ownership
   - StoredValue, ChildrenFn, Callback patterns

4. **[lists.md](../../src/rules/lists.md)** - Canon Rule #3: Lists & Iteration
   - Component isolation, `<For>` patterns

5. **[hydration.md](../../src/rules/hydration.md)** - Canon Rule #4: Hydration
   - SSR safety patterns

6. **[ssr_effects.md](../../src/rules/ssr_effects.md)** - Canon Rule #5: SSR Effects
   - Browser API guards, Effect safety

7. **[visual_state.md](../../src/rules/visual_state.md)** - Canon Rule #6: Visual State
   - data-attributes, tokens, no hardcoded colors

8. **[THEME-TOKEN-GOVERNANCE.md](../THEME-TOKEN-GOVERNANCE.md)** - Canon Rule #7: Token Governance
   - rs-design → rs-tailwind → apps pipeline
   - Theme system architecture

## Document Hierarchy
```
Normative (MUST follow)
├── ARCHITECTURE.md ............... Foundation
├── Canon Rules #1-#7 ............. Implementation rules
└── THEME-TOKEN-GOVERNANCE.md ..... Enterprise governance

Informative (SHOULD follow)
├── Examples ...................... Best practices
└── Migration guides .............. Upgrade paths
```

## For Contributors

Before submitting PRs:
1. Read [ARCHITECTURE.md](../ARCHITECTURE.md)
2. Check relevant Canon Rules
3. Run compliance checks (see each rule)
4. Reference Canon Rule # in PR description

## For AI Systems

These documents define the complete mental model of CanonRS:
- **Layer separation** (tokens → pages)
- **Ownership patterns** (StoredValue, Memo)
- **SSR safety** (`#[cfg(target_arch = "wasm32")]`)
- **Visual state** (data-attributes + tokens)
- **Token governance** (single source of truth)

When generating code, ALWAYS reference the appropriate Canon Rule.

---

**Status:** Architecture Index | Living Document
**Last Updated:** 2025-12-28 | Canon v1.3
