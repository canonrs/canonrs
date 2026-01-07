# Canon Rule #03: BFF Mandatory Boundary

**Status:** Normative  
**Applies to:** All products with frontend + backend architecture  
**Related:** Canon Rule #02 (One Compose per Product)

---

## The Principle

**Frontend MUST have ZERO network visibility of core services.**

All communication MUST flow through the BFF (Backend-For-Frontend).

---

## The Problem

### Anti-pattern: Frontend → Core Services
```yaml
# ❌ WRONG: Frontend calls rs-core-auth directly
frontend:
  environment:
    - RS_CORE_AUTH_URL=http://rs-core-auth:3000  # ❌ Direct access
  networks:
    - sha-net  # ❌ Exposed to infrastructure network
```

**Issues:**
- ❌ Security: Frontend has unnecessary network access
- ❌ Coupling: Frontend depends on internal service contracts
- ❌ Auth: Core services must validate frontend tokens
- ❌ Evolution: Can't change core APIs without breaking frontends
- ❌ Monitoring: Traffic not centralized through BFF

---

## The Solution

### Correct: Frontend → BFF → Core Services
```yaml
backend-api:  # BFF
  environment:
    - RS_CORE_AUTH_URL=http://rs-core-auth:3000  # ✅ BFF calls core
  networks:
    - sha-net              # ✅ Access to infrastructure
    - product-internal     # ✅ Access to frontend
  ports:
    - "3100:3100"         # ✅ Exposed to host

frontend:
  environment:
    - BFF_API_URL=http://backend-api:3100  # ✅ Only knows BFF
  depends_on:
    backend-api:
      condition: service_healthy
  networks:
    - product-internal     # ✅ Private network only
  ports:
    - "3101:3000"         # ✅ User-facing port
```

---

## Architecture Pattern
```
┌──────────────────────────────────────────────┐
│           sha-net (infrastructure)           │
│                                              │
│  ┌──────────────┐         ┌──────────────┐  │
│  │ rs-core-auth │◄────────┤ Backend-API  │  │
│  │   (Core)     │  Auth   │    (BFF)     │  │
│  └──────────────┘         └──────┬───────┘  │
│                                   │          │
│  ┌──────────────┐                │          │
│  │    Redis     │◄────────────────┘          │
│  └──────────────┘         mTLS/internal     │
│                                              │
└──────────────────────────────────────────────┘
                    │
        product-internal (isolated)
                    │
          ┌─────────▼────────┐
          │   Frontend-SSR   │
          │    (Leptos)      │
          └──────────────────┘
                    ▲
                    │
              User Browser
           (Port 3101 only)
```

---

## Mandatory Requirements

### 1. Network Isolation

**BFF Network Access:**
```yaml
backend-api:
  networks:
    - sha-net              # ✅ Core services access
    - product-internal     # ✅ Frontend communication
```

**Frontend Network Access:**
```yaml
frontend:
  networks:
    - product-internal     # ✅ BFF only
    # ❌ NEVER add sha-net here
```

### 2. Environment Variables

**BFF `.env.example`:**
```bash
# Core Services (BFF access only)
RS_CORE_AUTH_URL=http://rs-core-auth:3000
REDIS_URL=redis://172.32.6.51:6379

# Public-facing
BFF_PORT=3100

# JWT/Auth
JWT_SECRET=change_me_in_production_minimum_32_chars
```

**Frontend `.env.example`:**
```bash
# Backend-For-Frontend (only external dependency)
BFF_API_URL=http://backend-api:3100

# Leptos Config
LEPTOS_SITE_ADDR=0.0.0.0:3000
LEPTOS_SITE_ROOT=/app

# ❌ NEVER include RS_CORE_AUTH_URL here
# ❌ NEVER include REDIS_URL here
```

### 3. Service Dependencies
```yaml
frontend:
  depends_on:
    backend-api:
      condition: service_healthy  # ✅ Wait for BFF ready
```

### 4. Port Exposure

**BFF Ports:**
```yaml
backend-api:
  ports:
    - "3100:3100"  # ✅ API endpoint for frontend
```

**Frontend Ports:**
```yaml
frontend:
  ports:
    - "3101:3000"  # ✅ User-facing only
```

**Core Services:**
```yaml
rs-core-auth:
  # ❌ NO ports exposed to host
  # Only accessible via sha-net internally
```

---

## BFF Responsibilities

### MUST Implement

1. **Authentication Proxy**
   - Validate user sessions/tokens
   - Refresh tokens before expiry
   - Transform core auth to frontend-friendly format

2. **Authorization**
   - Check user permissions before core service calls
   - Aggregate permission checks from multiple services

3. **API Aggregation**
   - Combine multiple core service calls into single frontend endpoint
   - Reduce chattiness (1 frontend call → N core calls)

4. **Data Transformation**
   - Convert core DTOs to frontend-optimized formats
   - Hide internal implementation details

5. **Error Handling**
   - Translate core service errors to user-friendly messages
   - Log detailed errors, return sanitized responses

6. **Rate Limiting**
   - Protect core services from frontend abuse
   - Implement per-user quotas

---

## Example: BFF Endpoint
```rust
// products/core-auth/backend-api/src/routes/auth.rs

use axum::{extract::State, Json};
use crate::AppState;
use crate::dto::{LoginRequest, LoginResponse};

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // 1. Validate request
    req.validate()?;

    // 2. Call core service (BFF → Core)
    let auth_response = state
        .http_client
        .post(format!("{}/api/v1/auth/login", state.rs_core_auth_url))
        .json(&req)
        .send()
        .await?;

    // 3. Transform response for frontend
    let core_data = auth_response.json::<CoreAuthResponse>().await?;
    
    // 4. Create session in Redis
    let session_id = state.session_service
        .create(core_data.user_id)
        .await?;

    // 5. Return frontend-optimized response
    Ok(Json(LoginResponse {
        session_id,
        user: UserDto {
            id: core_data.user_id,
            email: core_data.email,
            // Hide internal fields
        },
    }))
}
```

---

## Frontend Implementation
```rust
// products/core-auth/frontend-leptos/src/api/auth.rs

use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    session_id: String,
    user: UserDto,
}

pub async fn login(email: String, password: String) -> Result<LoginResponse, ApiError> {
    // ✅ ONLY calls BFF, never core services
    let bff_url = std::env::var("BFF_API_URL")
        .unwrap_or_else(|_| "http://backend-api:3100".to_string());

    let response = reqwest::Client::new()
        .post(format!("{}/api/auth/login", bff_url))
        .json(&LoginRequest { email, password })
        .send()
        .await?;

    Ok(response.json().await?)
}
```

---

## Security Benefits

### Defense in Depth

1. **Network Segmentation**
   - Frontend can't reach core services (no sha-net access)
   - BFF is single point of entry (monitoring/logging)

2. **Credential Isolation**
   - Frontend never sees Redis credentials
   - Frontend never sees core service URLs
   - JWT secrets only in BFF

3. **Attack Surface Reduction**
   - Core services don't expose ports to host
   - BFF validates/sanitizes all inputs
   - Rate limiting protects core infrastructure

**Compliance:** ISO 27001 (least privilege), Zero Trust Architecture, OWASP ASVS

---

## Debugging Checklist

### "Frontend can't reach BFF"
```bash
# 1. Verify BFF is healthy
docker compose ps backend-api
curl http://localhost:3100/health

# 2. Verify frontend env
docker compose exec frontend env | grep BFF_API_URL

# 3. Check network connectivity
docker compose exec frontend curl http://backend-api:3100/health
```

### "BFF can't reach core services"
```bash
# 1. Verify BFF networks
docker inspect core-auth-backend-api | jq '.[0].NetworkSettings.Networks | keys'

# 2. Verify core service URL
docker compose exec backend-api env | grep RS_CORE_AUTH_URL

# 3. Test connectivity
docker compose exec backend-api curl http://rs-core-auth:3000/health
```

---

## Migration Guide

### Existing Direct Access → BFF Pattern

**Step 1: Audit frontend core service calls**
```bash
grep -r "RS_CORE_AUTH_URL" products/*/frontend*/src/
```

**Step 2: Create BFF endpoints**
```rust
// For each frontend → core call, create BFF route
// backend-api/src/routes/mod.rs
```

**Step 3: Update frontend to call BFF**
```rust
// Replace direct calls with BFF calls
- let url = format!("{}/api/users", env!("RS_CORE_AUTH_URL"));
+ let url = format!("{}/api/users", env!("BFF_API_URL"));
```

**Step 4: Remove frontend core access**
```yaml
# docker-compose.yml
frontend:
  networks:
-   - sha-net  # ❌ Remove
    - product-internal
```

---

## Automated Enforcement

### CI/CD Checks

PRs SHOULD be rejected if:

- `frontend` service joins `sha-net`
- `RS_CORE_*` env vars appear in frontend code
- Frontend containers expose multiple networks

### Detection Scripts
```bash
# 1. Detect frontend accessing core network
if grep -r "sha-net" products/*/frontend*/docker-compose* 2>/dev/null; then
  echo "❌ VIOLATION: Frontend cannot join sha-net"
  exit 1
fi

# 2. Detect forbidden env vars in frontend
if grep -rE "RS_CORE_|REDIS_URL" products/*/frontend*/src/ 2>/dev/null | grep -v ".env.example"; then
  echo "❌ VIOLATION: Frontend contains core service credentials"
  exit 1
fi

# 3. Verify BFF is gateway
frontend_services=$(docker compose config | yq '.services | keys | .[] | select(. == "*frontend*")')
for svc in $frontend_services; do
  networks=$(docker compose config | yq ".services.$svc.networks | keys")
  if echo "$networks" | grep -q "sha-net"; then
    echo "❌ VIOLATION: $svc exposes sha-net"
    exit 1
  fi
done

echo "✅ BFF boundary enforcement passed"
```

### Integration
```yaml
# .github/workflows/pr-check.yml
- name: Enforce BFF Boundary
  run: |
    chmod +x core-services/_rules/scripts/check-bff-boundary.sh
    ./core-services/_rules/scripts/check-bff-boundary.sh
```

---

## Normative Status

- Violations **MUST** block deployment
- All new products **MUST** use BFF pattern
- Direct frontend → core calls **MUST** be rejected in PR review
- BFF **MUST** be the only service with sha-net + product-internal access
- CI/CD **MUST** enforce boundary checks automatically

---

## Examples

### ✅ Correct: BFF Gateway
```yaml
services:
  backend-api:
    networks:
      - sha-net
      - product-internal
    environment:
      - RS_CORE_AUTH_URL=http://rs-core-auth:3000

  frontend:
    networks:
      - product-internal  # ✅ Isolated
    environment:
      - BFF_API_URL=http://backend-api:3100
```

### ❌ Wrong: Direct Core Access
```yaml
services:
  frontend:
    networks:
      - sha-net  # ❌ Exposed to core
    environment:
      - RS_CORE_AUTH_URL=http://rs-core-auth:3000  # ❌ Direct call
```

---

**Author:** DevOps Working Group  
**Date:** 2025-01-06  
**Version:** 1.0  
**Replaces:** None  
**Related:** Canon Rule #02 (One Compose per Product)
