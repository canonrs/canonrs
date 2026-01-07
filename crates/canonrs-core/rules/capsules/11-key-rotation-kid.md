# Rule Capsule #11: Key Rotation & kid Enforcement

**Decision Trigger:**
If JWKS or key management is involved, apply this rule immediately.

**Essence:** Every key has `kid`. Rotation is mandatory, not optional.

**Evita:**
- Single key forever
- Missing kid in JWT header
- Untracked key versions

**Always think:**
"Can I rotate this key without downtime?"

**Pattern:**
```rust
// ❌ WRONG
let key = load_single_key();

// ✅ CORRECT
let kid = header.kid.ok_or(Error::MissingKid)?;
let key = jwks.find_key(kid)?;
```

**When to apply:** Implementing JWT signing or verification.
