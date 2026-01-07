# Rule Capsule #10: mTLS Core-to-Core

**Decision Trigger:**
If core services communicate, apply this rule immediately.

**Essence:** Internal core-to-core = mTLS only. No plaintext.

**Evita:**
- HTTP between cores
- Missing certificate validation
- Trust without verification

**Always think:**
"Is this core talking to core? Then mTLS."

**Pattern:**
```yaml
# ❌ WRONG
http://other-core:8080

# ✅ CORRECT
https://other-core:8443
+ client certificates
+ CA validation
```

**When to apply:** Any internal service-to-service communication.
