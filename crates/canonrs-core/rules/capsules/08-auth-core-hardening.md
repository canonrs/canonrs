# Rule Capsule #08: Auth Core Hardening

**Decision Trigger:**
If auth service shows any exposure or weakness, apply this rule immediately.

**Essence:** Auth Core is fortress. No direct access, no shortcuts.

**Evita:**
- Direct HTTP calls to auth
- Bypassing token validation
- Weak secret management

**Always think:**
"Is this touching auth? Then it goes through the contract."

**Pattern:**
```rust
// ❌ WRONG
let response = reqwest::get("http://auth:8001/verify").await;

// ✅ CORRECT
let token = extract_from_header(req);
verify_with_jwks(token, audience).await;
```

**When to apply:** Any auth-related integration or token handling.
