# Rule Capsule #03: BFF Mandatory Boundary

**Decision Trigger:**
If frontend needs data, apply this rule immediately.

**Essence:** Frontend NEVER calls core directly. BFF is the gateway.

**Evita:**
- Direct core exposure
- CORS hell
- Token leakage to browser

**Always think:**
"Is this a frontend request? Then it goes through BFF."

**Pattern:**
```
❌ WRONG: Browser → Core Service
✅ CORRECT: Browser → BFF → Core Service(s)
```

**When to apply:** Any frontend API integration.
