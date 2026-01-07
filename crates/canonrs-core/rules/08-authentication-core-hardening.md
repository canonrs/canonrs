# Canon Rule #08: Authentication Core Hardening Standard

**Status:** Normative  
**Applies to:** All authentication/authorization core services (rs-core-auth, rs-core-*-auth)  
**Related:** Canon Rule #03 (BFF Mandatory Boundary), Canon Rule #05 (Core Services Port Non-Exposure), Canon Rule #07 (Environment Variable Classification)

---

## The Principle

**Authentication core services MUST implement specific technical standards that prevent compromise of the entire system.**

Auth cores are single points of failure - excellence is mandatory, not optional.

---

## The Problem

### Anti-pattern: Authentication as "Just Another Service"
```rust
// ❌ WRONG: Auth service with business logic
async fn login(req: LoginRequest) -> Result<Token> {
    let user = User::find(&req.email)?;
    
    // ❌ Product logic in auth core
    if user.subscription_expired() {
        return Err("Upgrade required");
    }
    
    // ❌ Weak token
    let token = format!("{}:{}", user.id, Uuid::new_v4());
    
    Ok(Token { value: token })
}

// ❌ No verification endpoint
// ❌ No revocation
// ❌ Tokens validated in multiple services
```

**Issues:**
- ❌ **Security:** Weak tokens, no cryptographic signing
- ❌ **Architecture:** Business logic couples auth to products
- ❌ **Scale:** Every service implements its own token validation
- ❌ **Revocation:** No way to invalidate compromised tokens
- ❌ **Auditability:** No centralized auth event log

---

## The Solution

### Correct: Hardened Authentication Core
```rust
// ✅ CORRECT: Pure authentication logic
pub struct AuthCore {
    jwt_signer: JwtSigner,      // RS256 / EdDSA
    session_store: SessionStore, // Redis with TTL
    audit_log: AuditLogger,      // Structured events
}

impl AuthCore {
    pub async fn login(&self, req: LoginRequest) -> Result<TokenPair> {
        // 1. Authenticate (ONLY auth logic)
        let user = self.verify_credentials(&req).await?;
        
        // 2. Generate cryptographically signed tokens
        let access_token = self.jwt_signer.sign(Claims {
            sub: user.id,
            exp: now() + Duration::minutes(15), // Short TTL
            iat: now(),
        })?;
        
        let refresh_token = self.jwt_signer.sign(RefreshClaims {
            sub: user.id,
            exp: now() + Duration::days(30),
            jti: Uuid::new_v4(), // Unique refresh ID
        })?;
        
        // 3. Store session for revocation
        self.session_store.create(user.id, &refresh_token.jti).await?;
        
        // 4. Audit
        self.audit_log.log(AuthEvent::Login {
            user_id: user.id,
            timestamp: now(),
            ip: req.ip,
        }).await?;
        
        Ok(TokenPair { access_token, refresh_token })
    }
    
    pub async fn verify(&self, token: &str) -> Result<Claims> {
        // ✅ Single source of truth for token validation
        self.jwt_signer.verify(token)
    }
    
    pub async fn revoke(&self, user_id: Uuid) -> Result<()> {
        // ✅ Centralized revocation
        self.session_store.delete_all(user_id).await?;
        self.audit_log.log(AuthEvent::Revoke { user_id }).await?;
        Ok(())
    }
}
```

---

## Mandatory Requirements

### 1. Zero Product Logic

**Authentication cores MUST contain ONLY:**
- ✅ Credential verification (password, OAuth, etc.)
- ✅ Token generation
- ✅ Token verification
- ✅ Session management
- ✅ Token revocation

**Authentication cores MUST NEVER contain:**
- ❌ Business rules (subscription checks, feature flags)
- ❌ Product-specific data (orders, profiles, analytics)
- ❌ Multi-service orchestration
- ❌ Frontend-specific logic

**Rationale:** Product logic creates coupling that prevents auth reuse and creates hidden vulnerabilities.

---

### 2. API Surface (Mandatory Endpoints)

**Health & Observability:**
```rust
GET /health        // Liveness probe
GET /ready         // Readiness probe (checks dependencies)
GET /metrics       // Prometheus metrics
```

**Core Authentication:**
```rust
POST   /auth/login       // Issue tokens
POST   /auth/verify      // Validate access token
POST   /auth/refresh     // Refresh access token
POST   /auth/logout      // Revoke session
DELETE /auth/sessions/:user_id  // Revoke all user sessions
```

**Admin (Internal Only):**
```rust
GET /auth/sessions      // List active sessions (ops)
GET /auth/audit         // Query auth events (security)
```

**Forbidden Endpoints:**
```rust
// ❌ NEVER implement in auth core:
GET  /users/:id/profile      // Product data
GET  /users/:id/orders       // Business logic
POST /users/:id/upgrade      // Product operation
```

---

### 3. Token Standards

**Access Token (Short-Lived):**
```rust
// JWT with RS256 or EdDSA signature
{
  "sub": "user_uuid",           // Subject (user ID)
  "exp": 1735689600,            // Expiration (15 minutes)
  "iat": 1735688700,            // Issued at
  "iss": "rs-core-auth",        // Issuer
  "aud": ["bff-api"],           // Audience
  "scope": ["read", "write"]    // Optional: scopes
}
```

**Requirements:**
- ✅ MUST use asymmetric cryptography (RS256 / EdDSA)
- ✅ MUST have expiration ≤ 15 minutes
- ✅ MUST include `sub`, `exp`, `iat`, `iss`
- ✅ MUST be validated ONLY by auth core
- ❌ NEVER include sensitive data (passwords, PII)

**Refresh Token:**
```rust
{
  "sub": "user_uuid",
  "exp": 1738281600,            // 30 days
  "jti": "unique_refresh_id",   // For revocation
  "type": "refresh"
}
```

**Requirements:**
- ✅ MUST have unique `jti` for revocation
- ✅ MUST be stored in session store
- ✅ MUST be single-use (rotate on refresh)
- ✅ MUST expire ≤ 30 days

---

### 4. Cryptographic Standards

**Key Management:**
```rust
// Private key (NEVER leaves auth core)
ED25519_PRIVATE_KEY=<vault:secret/rs-core-auth/signing_key>

// Public key (distributed to BFFs for verification)
ED25519_PUBLIC_KEY=<from-auth-core-endpoint>
```

**Requirements:**
- ✅ Private key MUST be in Vault (Critical level)
- ✅ Private key MUST never be in logs/errors
- ✅ Public key MUST be served via `/auth/public-key`
- ✅ Key rotation MUST be supported (key ID in JWT header)
- ❌ NEVER use symmetric signing (HS256) in production

**Algorithms (Priority Order):**
1. **EdDSA (Ed25519)** - Recommended
2. **RS256 (RSA 2048+)** - Acceptable
3. ❌ **HS256** - Forbidden in production

---

### 5. Fail-Closed Security

**Every auth operation MUST fail securely:**
```rust
pub async fn verify(&self, token: &str) -> Result<Claims> {
    // Default: DENY
    let claims = self.jwt_signer.verify(token)
        .map_err(|e| {
            // ❌ NEVER return partial success
            error!("Token verification failed: {}", e);
            AuthError::InvalidToken
        })?;
    
    // Check expiration explicitly
    if claims.exp < now() {
        return Err(AuthError::TokenExpired);
    }
    
    // Check session still valid (not revoked)
    if let Some(jti) = claims.jti {
        if !self.session_store.exists(&jti).await? {
            return Err(AuthError::SessionRevoked);
        }
    }
    
    // Only return claims if ALL checks pass
    Ok(claims)
}
```

**Requirements:**
- ✅ MUST deny on ANY validation failure
- ✅ MUST check token expiration explicitly
- ✅ MUST verify session not revoked
- ✅ MUST log all failures (audit trail)
- ❌ NEVER return partial/degraded auth state
- ❌ NEVER assume token valid by default

---

### 6. Session Management

**Session Store Requirements:**
```rust
pub trait SessionStore {
    async fn create(&self, user_id: Uuid, refresh_jti: &str) -> Result<()>;
    async fn exists(&self, refresh_jti: &str) -> Result<bool>;
    async fn delete(&self, refresh_jti: &str) -> Result<()>;
    async fn delete_all(&self, user_id: Uuid) -> Result<()>;
}
```

**Implementation (Redis):**
```rust
// Key pattern: session:{user_id}:{jti}
// TTL: Match refresh token expiration

impl SessionStore for RedisStore {
    async fn create(&self, user_id: Uuid, jti: &str) -> Result<()> {
        let key = format!("session:{}:{}", user_id, jti);
        self.redis
            .set_ex(key, "1", 30 * 24 * 3600) // 30 days
            .await?;
        Ok(())
    }
    
    async fn delete_all(&self, user_id: Uuid) -> Result<()> {
        // Revoke all user sessions
        let pattern = format!("session:{}:*", user_id);
        let keys: Vec<String> = self.redis.keys(pattern).await?;
        
        for key in keys {
            self.redis.del(key).await?;
        }
        
        Ok(())
    }
}
```

**Requirements:**
- ✅ MUST support revocation (per-session and per-user)
- ✅ MUST use TTL matching token expiration
- ✅ MUST be atomic operations
- ✅ MUST handle Redis failures gracefully (fail-closed)

---

### 7. Audit Logging

**Structured Auth Events:**
```rust
#[derive(Serialize)]
pub enum AuthEvent {
    Login {
        user_id: Uuid,
        timestamp: DateTime<Utc>,
        ip: IpAddr,
        user_agent: String,
    },
    LoginFailed {
        email: String,
        reason: String,
        ip: IpAddr,
        timestamp: DateTime<Utc>,
    },
    TokenRefresh {
        user_id: Uuid,
        old_jti: String,
        new_jti: String,
    },
    Logout {
        user_id: Uuid,
        jti: String,
    },
    SessionRevoked {
        user_id: Uuid,
        reason: String,
        revoked_by: Uuid,
    },
}
```

**Requirements:**
- ✅ MUST log ALL auth events (success and failure)
- ✅ MUST include timestamp, user_id, IP
- ✅ MUST use structured logging (JSON)
- ✅ MUST be append-only (never delete)
- ✅ SHOULD integrate with SIEM
- ❌ NEVER log passwords or full tokens

---

### 8. Network Isolation

**Auth Core Network Configuration:**
```yaml
# infrastructure/docker-compose.core.yml
services:
  rs-core-auth:
    expose:
      - "3000"  # ✅ Internal only
    networks:
      - sha-net  # ✅ Core services network ONLY
    # ❌ NEVER add traefik labels
    # ❌ NEVER expose ports to host
    # ❌ NEVER join product-specific networks
```

**Allowed Callers:**
- ✅ BFF services (via sha-net)
- ✅ Other core services (authentication chaining)
- ❌ NEVER frontend directly
- ❌ NEVER Traefik directly
- ❌ NEVER external services

**Enforcement:** See Canon Rule #03 (BFF Mandatory Boundary) and Rule #05 (Core Services Port Non-Exposure)

---

### 9. Observability Requirements

**Metrics (Prometheus):**
```rust
// Required metrics
auth_login_total{status="success|failure"}
auth_verify_total{status="valid|invalid|expired"}
auth_refresh_total{status="success|failure"}
auth_session_revoke_total
auth_token_generation_duration_seconds
auth_verify_duration_seconds
auth_active_sessions_gauge
```

**Health Checks:**
```rust
GET /health
{
  "status": "healthy",
  "version": "1.0.0",
  "uptime_seconds": 3600
}

GET /ready
{
  "status": "ready",
  "checks": {
    "redis": "ok",
    "vault": "ok"
  }
}
```

**Requirements:**
- ✅ MUST expose `/health` (liveness)
- ✅ MUST expose `/ready` (readiness)
- ✅ MUST expose `/metrics` (Prometheus)
- ✅ MUST include dependency checks in `/ready`
- ✅ MUST track auth operation latencies

---

## Implementation Checklist

### Minimum Viable Auth Core

- [ ] **API Endpoints**
  - [ ] POST /auth/login
  - [ ] POST /auth/verify
  - [ ] POST /auth/refresh
  - [ ] POST /auth/logout
  - [ ] GET /health
  - [ ] GET /ready
  - [ ] GET /metrics

- [ ] **Cryptography**
  - [ ] EdDSA or RS256 token signing
  - [ ] Private key in Vault
  - [ ] Public key endpoint
  - [ ] Key rotation support

- [ ] **Session Management**
  - [ ] Redis-backed sessions
  - [ ] Per-session revocation
  - [ ] Per-user revocation
  - [ ] TTL matching tokens

- [ ] **Security**
  - [ ] Fail-closed verification
  - [ ] Explicit expiration checks
  - [ ] Session revocation checks
  - [ ] No product logic

- [ ] **Observability**
  - [ ] Structured audit logging
  - [ ] Prometheus metrics
  - [ ] Health/ready endpoints
  - [ ] Dependency checks

- [ ] **Network Isolation**
  - [ ] Only sha-net
  - [ ] No ports exposed
  - [ ] No Traefik access
  - [ ] BFF-only consumers

---

## Testing Requirements

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_token_expiration_enforced() {
        let auth = AuthCore::new_test();
        let token = auth.generate_expired_token();
        
        let result = auth.verify(&token).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthError::TokenExpired);
    }
    
    #[tokio::test]
    async fn test_revoked_session_denied() {
        let auth = AuthCore::new_test();
        let token = auth.login(test_user()).await.unwrap();
        
        auth.revoke(test_user().id).await.unwrap();
        
        let result = auth.verify(&token.access_token).await;
        assert!(result.is_err());
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_full_auth_flow() {
    // 1. Login
    let tokens = auth_client.login("user@example.com", "password").await?;
    assert!(tokens.access_token.len() > 0);
    
    // 2. Verify
    let claims = auth_client.verify(&tokens.access_token).await?;
    assert_eq!(claims.sub, expected_user_id);
    
    // 3. Refresh
    let new_tokens = auth_client.refresh(&tokens.refresh_token).await?;
    assert_ne!(new_tokens.access_token, tokens.access_token);
    
    // 4. Logout
    auth_client.logout(&new_tokens.refresh_token).await?;
    
    // 5. Verify old token now invalid
    let result = auth_client.verify(&new_tokens.access_token).await;
    assert!(result.is_err());
}
```

---

## Automated Enforcement

### Validation Script
```bash
#!/bin/bash
# core-services/_rules/scripts/validate-auth-core.sh

set -e

echo "🔍 [Rule #08] Validating Authentication Core Hardening..."

VIOLATIONS=0

AUTH_SERVICES=$(find core-services infrastructure -name "Cargo.toml" -exec grep -l "rs-core-auth\|auth-core" {} \; | xargs dirname)

for service_dir in $AUTH_SERVICES; do
  echo "  → Checking $service_dir..."
  
  # 1. Verify mandatory endpoints exist
  ENDPOINTS=("/auth/login" "/auth/verify" "/auth/refresh" "/health" "/ready" "/metrics")
  
  for endpoint in "${ENDPOINTS[@]}"; do
    if ! grep -r "\"$endpoint\"" "$service_dir/src" >/dev/null 2>&1; then
      echo "❌ VIOLATION: Missing mandatory endpoint $endpoint"
      VIOLATIONS=$((VIOLATIONS + 1))
    fi
  done
  
  # 2. Check for product logic (forbidden patterns)
  PRODUCT_PATTERNS=("subscription" "billing" "order" "upgrade" "feature_flag")
  
  for pattern in "${PRODUCT_PATTERNS[@]}"; do
    if grep -ri "$pattern" "$service_dir/src" >/dev/null 2>&1; then
      echo "⚠️  WARNING: Possible product logic detected: $pattern"
    fi
  done
  
  # 3. Verify cryptographic signing (not HS256)
  if grep -r "Algorithm::HS256\|HS256" "$service_dir/src" >/dev/null 2>&1; then
    echo "❌ VIOLATION: HS256 symmetric signing forbidden in production"
    VIOLATIONS=$((VIOLATIONS + 1))
  fi
  
  # 4. Check session store exists
  if ! grep -r "SessionStore\|session_store" "$service_dir/src" >/dev/null 2>&1; then
    echo "❌ VIOLATION: No session store implementation found"
    VIOLATIONS=$((VIOLATIONS + 1))
  fi
  
  # 5. Verify audit logging
  if ! grep -r "AuthEvent\|audit_log" "$service_dir/src" >/dev/null 2>&1; then
    echo "⚠️  WARNING: No audit logging detected"
  fi
done

if [[ $VIOLATIONS -eq 0 ]]; then
  echo "✅ [Rule #08] Authentication Core Hardening validation passed"
  exit 0
else
  echo "❌ [Rule #08] Authentication Core Hardening validation FAILED with $VIOLATIONS violation(s)"
  exit 1
fi
```

---

## Migration Guide

### Existing Auth Service → Hardened Core

**Step 1: Extract product logic**
```bash
# Identify product-specific code
grep -r "subscription\|billing\|feature" src/

# Move to BFF layer
mv src/routes/subscription.rs ../bff-api/src/routes/
```

**Step 2: Implement cryptographic signing**
```rust
// Before: Weak tokens
let token = format!("{}:{}", user.id, Uuid::new_v4());

// After: Signed JWT
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

let token = encode(
    &Header::new(Algorithm::EdDSA),
    &claims,
    &EncodingKey::from_ed_pem(private_key)?
)?;
```

**Step 3: Add session store**
```rust
// Add Redis dependency
// Cargo.toml
redis = { version = "0.24", features = ["tokio-comp"] }

// Implement session management
pub struct SessionStore {
    redis: redis::Client,
}
```

**Step 4: Add mandatory endpoints**
```rust
// src/routes/mod.rs
pub fn auth_routes() -> Router {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/verify", post(verify))
        .route("/auth/refresh", post(refresh))
        .route("/auth/logout", post(logout))
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/metrics", get(metrics))
}
```

**Step 5: Implement fail-closed**
```rust
// Before: Optimistic validation
if token.is_valid() { /* allow */ }

// After: Fail-closed
token.verify()
    .and_then(|claims| check_expiration(claims))
    .and_then(|claims| check_not_revoked(claims))
    .map_err(|_| AuthError::Denied)?;
```

---

## Normative Status

- Violations **MUST** block deployment
- All authentication cores **MUST** follow this standard
- Existing auth services **MUST** migrate within 90 days
- New auth cores **MUST** pass validation before production
- Exceptions require security committee approval

---

## Examples

### ✅ Correct: Hardened Auth Core
```rust
// core-services/rs-core-auth/src/main.rs

pub struct AuthCore {
    jwt_signer: EdDsaSigner,
    session_store: RedisSessionStore,
    audit_log: StructuredLogger,
}

impl AuthCore {
    pub async fn verify(&self, token: &str) -> Result<Claims> {
        // Fail-closed: deny by default
        let claims = self.jwt_signer
            .verify(token)
            .map_err(|_| AuthError::InvalidToken)?;
        
        if claims.exp < now() {
            return Err(AuthError::TokenExpired);
        }
        
        if !self.session_store.exists(&claims.jti).await? {
            return Err(AuthError::SessionRevoked);
        }
        
        Ok(claims)
    }
}
```

### ❌ Wrong: Weak Auth Service
```rust
// ❌ Product logic mixed in
pub async fn login(req: LoginRequest) -> Result<Token> {
    let user = User::find(&req.email)?;
    
    if user.subscription_tier == "free" {  // ❌ Product logic
        return Err("Upgrade required");
    }
    
    let token = format!("{}:{}", user.id, random());  // ❌ Not signed
    Ok(Token { value: token })
}

// ❌ No verification endpoint
// ❌ No revocation support
```

---

**Author:** DevOps Working Group  
**Date:** 2025-01-06  
**Version:** 1.0  
**Replaces:** None  
**Related:** Canon Rule #03 (BFF Mandatory Boundary), Canon Rule #05 (Core Services Port Non-Exposure), Canon Rule #07 (Environment Variable Classification)
