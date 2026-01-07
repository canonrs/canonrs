# Canon Rule #11: Key Rotation & kid Enforcement

**Status:** `ACTIVE`  
**Enforcement:** `MANDATORY`  
**Scope:** All JWT-issuing services (Auth Core)

---

## 🎯 Objective

Enforce cryptographic key rotation and Key ID (`kid`) tracking to enable zero-downtime key updates, prevent token replay attacks, and support key compromise recovery.

---

## 📋 Requirements

### 1. Key ID (kid) in JWT Header

**MUST:**
- Every JWT MUST include `kid` in header
- `kid` MUST be unique and immutable per key
- `kid` MUST follow format: `{service}-{algorithm}-{timestamp}`

### 2. Key Storage in Vault Transit

**MUST:**
- Store all signing keys in Vault Transit engine
- Never store private keys in filesystem or env vars
- Use Vault's native rotation capabilities
- Maintain key version history

### 3. Key Rotation Policy

**MUST:**
- Rotate signing keys every 90 days (RECOMMENDED: 30 days)
- Support overlapping validity periods (grace period: 7 days)
- Maintain at least 2 active keys during rotation
- Old keys MUST verify tokens but NOT sign new ones

### 4. JWKS Endpoint

**MUST:**
- Expose public keys via `/.well-known/jwks.json`
- Include all active keys (current + grace period)
- Update JWKS atomically during rotation

### 5. Token Verification with kid

**MUST:**
- BFFs MUST fetch `kid` from token header
- BFFs MUST lookup corresponding public key from JWKS
- BFFs MUST reject tokens with unknown `kid`
- BFFs MUST cache JWKS with TTL ≤ 5 minutes

---

## 🚫 Prohibited Patterns

**NEVER:**
- Issue tokens without `kid`
- Use single hardcoded key indefinitely
- Store private keys in code/config
- Rotate keys without grace period
- Skip kid validation in BFFs

---

## 🔍 Validation

Run: `./core-services/_rules/scripts/validate-key-rotation.sh`

---

**Rationale:** Key rotation limits the blast radius of key compromise and enables zero-downtime updates. The `kid` mechanism allows multiple keys to coexist during rotation.
