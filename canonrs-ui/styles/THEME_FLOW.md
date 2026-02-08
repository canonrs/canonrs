# Theme Flow

**Rule #174:** Tokens are compile-time contracts
**Rule #176:** Governance is single source of truth

---

## Flow
```
TypeScript preset
    ↓
token-engine build
    ↓
semantic.css (derived from presets under governance)
    ↓
canonrs.css imports
    ↓
dist/
```

---

## Steps

1. Define theme in `/src/tokens/themes/presets/*.ts`
2. Generate: `npx tsx token-engine/build.ts`
3. Output: `styles/tokens/semantic.css` (governed mappings)
4. Build: `npm run build` → copies to `dist/`
5. Apply: `<html data-theme="theme-id">`

---

**Themes map semantics only.** Primitives never change.

See `/CANONICAL_TOKENS.md` for token rules.
