# Rule Capsule #09: Token Audience Validation

**Decision Trigger:**
If token validation fails unexpectedly, apply this rule immediately.

**Essence:** Every service validates `aud` claim. No exceptions.

**Evita:**
- Accepting tokens meant for other services
- Token reuse attacks
- Missing audience checks

**Always think:**
"Did I validate the token is FOR THIS SERVICE?"

**Pattern:**
```rust
// ❌ WRONG
verify_signature(token);

// ✅ CORRECT
verify_signature(token)?;
verify_audience(token, "core-service-x")?;
```

**When to apply:** Implementing any JWT verification.
