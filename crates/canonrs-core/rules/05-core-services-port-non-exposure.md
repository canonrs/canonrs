# Canon Rule #05: Core Services Port Non-Exposure

**Status:** Normative  
**Applies to:** All core services (rs-core-*, infrastructure services)  
**Related:** Canon Rule #03 (BFF Mandatory Boundary), Canon Rule #04 (Port Allocation Strategy)

---

## The Principle

**Core services MUST NEVER expose ports to the host, even in development.**

Access is exclusively through internal Docker networks (sha-net).

---

## The Problem

### Anti-pattern: Exposed Core Service
```yaml
# ❌ WRONG: Core service with host port
rs-core-auth:
  ports:
    - "3000:3000"  # ❌ CRITICAL VIOLATION
  networks:
    - sha-net
```

**Issues:**
- ❌ **Security:** Direct access bypasses BFF security layer
- ❌ **Architecture:** Breaks BFF mandatory boundary (Rule #03)
- ❌ **Attack Surface:** Core logic exposed to host network
- ❌ **Zero Trust:** No authentication/authorization enforcement
- ❌ **Monitoring:** Traffic not logged through BFF gateway
- ❌ **Rate Limiting:** Core services unprotected from abuse

---

## The Solution

### Correct: Internal-Only Exposure
```yaml
# ✅ CORRECT: Use expose, not ports
rs-core-auth:
  expose:
    - "3000"  # ✅ Internal network only
  networks:
    - sha-net
  # ❌ NO 'ports:' key allowed
```

**Benefits:**
- ✅ Zero host exposure
- ✅ Enforces BFF gateway pattern
- ✅ Reduces attack surface
- ✅ Centralized monitoring/logging
- ✅ Consistent in dev AND prod

---

## Architecture Enforcement
```
┌────────────────────────────────────────┐
│         Host Network (0.0.0.0)         │
│                                        │
│  ❌ NO direct access to core services  │
│                                        │
└──────────────┬─────────────────────────┘
               │
               │ Only Traefik + BFFs exposed
               ▼
     ┌─────────────────┐
     │   Traefik:443   │
     └────────┬────────┘
              │
    ┌─────────┼─────────┐
    ▼         ▼         ▼
  Port 8000  Port 8100  Port 3100
┌─────────┐┌─────────┐┌──────────┐
│Frontend ││Frontend ││Backend   │
│(Leptos) ││(Next)   ││API (BFF) │
└────┬────┘└────┬────┘└────┬─────┘
     │          │          │
     └──────────┼──────────┘
                │
                │ sha-net (internal)
                ▼
    ┌───────────────────────┐
    │   Core Services Zone  │
    │                       │
    │  🔒 NO host ports     │
    │                       │
    │  rs-core-auth:3000    │ expose only
    │  rs-core-payments:3001│ expose only
    │  Redis:6380           │ expose only
    │  PostgreSQL:5432      │ expose only
    │                       │
    └───────────────────────┘
```

---

## Mandatory Requirements

### 1. Core Service Definition

Services classified as **core** if they:
- Provide shared business logic across products
- Store/manage critical data (auth, payments, users)
- Are named with `rs-core-*` prefix
- Live in `infrastructure/` directory

**Examples:**
- `rs-core-auth`
- `rs-core-payments`
- `rs-core-notifications`
- `PostgreSQL`
- `Redis`
- `Kafka`

### 2. Compose Configuration

**Core services MUST use:**
```yaml
rs-core-auth:
  expose:
    - "3000"        # ✅ Internal only
  networks:
    - sha-net       # ✅ Infrastructure network
  # ports: key FORBIDDEN
```

**Core services MUST NOT use:**
```yaml
rs-core-auth:
  ports:            # ❌ FORBIDDEN
    - "3000:3000"
  networks:
    - sha-net
```

### 3. Development Consistency

**Rule applies equally to:**
- `docker-compose.yml` (production)
- `docker-compose.dev.yml` (development)
- Local developer environments
- CI/CD pipelines

**No exceptions for "debugging".**

---

## Access Patterns

### ✅ Correct: BFF Gateway Access
```rust
// products/core-auth/backend-api/src/services/auth.rs

pub async fn validate_token(token: &str) -> Result<User, AuthError> {
    let rs_core_url = env::var("RS_CORE_AUTH_URL")
        .expect("RS_CORE_AUTH_URL required");
    
    // ✅ BFF calls core via internal network
    let response = reqwest::Client::new()
        .post(format!("{}/api/v1/auth/validate", rs_core_url))
        .bearer_auth(token)
        .send()
        .await?;
    
    Ok(response.json().await?)
}
```

### ❌ Wrong: Direct Core Access
```rust
// products/core-auth/frontend-leptos/src/api/auth.rs

pub async fn login(email: String, password: String) -> Result<Token> {
    // ❌ Frontend calling core directly
    let response = reqwest::Client::new()
        .post("http://rs-core-auth:3000/api/v1/auth/login")
        .json(&LoginRequest { email, password })
        .send()
        .await?;
    
    Ok(response.json().await?)
}
```

---

## Debugging Without Host Ports

### Scenario: "I need to test rs-core-auth directly"

**❌ Wrong Approach:**
```yaml
# Don't do this
rs-core-auth:
  ports:
    - "3000:3000"  # ❌ Violates rule for convenience
```

**✅ Correct Approaches:**

#### Option 1: Use Docker Exec
```bash
# Access from BFF container
docker compose exec backend-api curl http://rs-core-auth:3000/health

# Access from dedicated debug container
docker run --rm --network sha-net curlimages/curl \
  curl http://rs-core-auth:3000/api/v1/users
```

#### Option 2: Temporary Port Forward (Terminal Only)
```bash
# Forward port in terminal session only (not in compose)
docker run --rm -p 3000:3000 --network sha-net \
  alpine/socat TCP-LISTEN:3000,fork TCP:rs-core-auth:3000

# Test in another terminal
curl http://localhost:3000/health

# Kill socat when done (Ctrl+C)
```

#### Option 3: Debug via BFF
```bash
# Add temporary debug endpoint in BFF
# products/core-auth/backend-api/src/routes/debug.rs
#[cfg(debug_assertions)]
pub async fn proxy_to_core(req: Request) -> Response {
    // Proxy request to core for debugging
}
```

---

## Security Rationale

### Attack Vector Prevention

**Without this rule:**
```bash
# Attacker scans host
nmap -p 3000-3099 production-server.com

# Finds exposed core service
PORT      STATE SERVICE
3000/tcp  open  rs-core-auth

# Bypasses BFF authentication
curl http://production-server.com:3000/api/v1/admin/users
# ❌ Direct access to sensitive endpoint
```

**With this rule:**
```bash
# Attacker scans host
nmap -p 3000-3099 production-server.com

# No core services found
All 100 scanned ports are filtered

# Only entry point is Traefik (443)
PORT      STATE SERVICE
443/tcp   open  https

# Must go through BFF → proper auth/logging
curl https://app.production-server.com/api/users
# ✅ BFF enforces authentication, rate limiting, audit
```

### Compliance Benefits

- **Zero Trust:** All access mediated by BFF
- **Least Privilege:** Core services unreachable from host
- **Defense in Depth:** Multiple layers (Traefik → BFF → Core)
- **Audit Trail:** All traffic logged through BFF gateway

---

## Automated Enforcement

### Pre-Commit Hook
```bash
#!/bin/bash
# .git/hooks/pre-commit

set -e

echo "🔍 Checking core services for port exposure..."

# Find core services with 'ports:' key
VIOLATIONS=$(grep -A5 "rs-core-" infrastructure/docker-compose*.yml products/*/docker-compose*.yml 2>/dev/null | grep -B2 "ports:" || true)

if [[ -n "$VIOLATIONS" ]]; then
  echo "❌ CRITICAL: Core services cannot expose host ports"
  echo ""
  echo "$VIOLATIONS"
  echo ""
  echo "Use 'expose:' instead of 'ports:'"
  exit 1
fi

echo "✅ Port exposure check passed"
```

### CI/CD Pipeline
```bash
#!/bin/bash
# core-services/_rules/scripts/validate-core-exposure.sh

set -e

echo "🔍 Validating core services have no host port exposure..."

# Find all core services
CORE_SERVICES=$(find infrastructure products -name "docker-compose*.yml" -exec grep -l "rs-core-" {} \;)

for file in $CORE_SERVICES; do
  # Extract core service definitions
  EXPOSED=$(yq eval '.services | to_entries | .[] | select(.key | test("rs-core-")) | select(.value.ports != null) | .key' "$file" 2>/dev/null || true)
  
  if [[ -n "$EXPOSED" ]]; then
    echo "❌ VIOLATION: Core services with host ports in $file"
    echo "$EXPOSED"
    exit 1
  fi
done

echo "✅ Core services exposure validation passed"
```

### GitHub Actions
```yaml
# .github/workflows/security-check.yml
name: Security Check

on: [pull_request]

jobs:
  validate-core-exposure:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install yq
        run: |
          wget -qO /usr/local/bin/yq https://github.com/mikefarah/yq/releases/latest/download/yq_linux_amd64
          chmod +x /usr/local/bin/yq
      
      - name: Validate Core Services
        run: |
          chmod +x core-services/_rules/scripts/validate-core-exposure.sh
          ./core-services/_rules/scripts/validate-core-exposure.sh
```

---

## Migration Guide

### Existing Exposed Core → Internal Only

**Step 1: Identify violations**
```bash
grep -A3 "rs-core-" infrastructure/docker-compose.yml | grep "ports:"
```

**Step 2: Update compose files**
```bash
# Before
rs-core-auth:
  ports:
    - "3000:3000"
  networks:
    - sha-net

# After
rs-core-auth:
  expose:
    - "3000"
  networks:
    - sha-net
```

**Step 3: Update BFF connection strings**
```bash
# No changes needed - internal DNS still works
# backend-api still calls http://rs-core-auth:3000
```

**Step 4: Remove port forwarding from documentation**
```bash
# Update README.md, remove localhost:3000 references
sed -i '/localhost:3000/d' products/*/README.md
```

**Step 5: Test via BFF**
```bash
# Verify BFF can still reach core
docker compose exec backend-api curl http://rs-core-auth:3000/health
```

---

## Exception Process

### Requesting Exception

Exceptions are **rarely granted** and require:

1. Written justification from product owner
2. Architecture review approval
3. Time-bound (max 7 days)
4. Documented in issue tracker
5. Automated revert after expiry

**Valid reasons:**
- Emergency production debugging (time-limited)
- Migration period (coordinated downtime)

**Invalid reasons:**
- Developer convenience
- "It's easier to test this way"
- "We'll fix it later"

---

## Comparison Table

| Aspect | With Host Ports ❌ | Internal Only ✅ |
|--------|-------------------|------------------|
| **Security** | Direct attack surface | Mediated by BFF |
| **Architecture** | Breaks BFF pattern | Enforces gateway |
| **Monitoring** | Scattered logs | Centralized in BFF |
| **Rate Limiting** | Unprotected | BFF enforces limits |
| **Dev/Prod Parity** | Often different | Always identical |
| **Debugging** | curl localhost:3000 | docker exec + curl |
| **Compliance** | Fails audits | Passes Zero Trust |

---

## Normative Status

- Violations **MUST** block all deployments
- PR reviews **MUST** reject exposed core services
- CI/CD **MUST** enforce via automated checks
- Pre-commit hooks **SHOULD** catch violations locally
- Exceptions require architecture committee approval
- Rule applies equally to dev and prod environments

---

## Examples

### ✅ Correct: Full Stack Isolation
```yaml
# infrastructure/docker-compose.core.yml
services:
  rs-core-auth:
    expose: ["3000"]
    networks: [sha-net]
  
  rs-core-payments:
    expose: ["3001"]
    networks: [sha-net]
  
  redis:
    expose: ["6379"]
    networks: [sha-net]

# products/core-auth/docker-compose.yml
services:
  backend-api:
    ports: ["3100:3100"]  # ✅ BFF can expose
    networks:
      - sha-net
      - product-internal
    environment:
      - RS_CORE_AUTH_URL=http://rs-core-auth:3000
  
  frontend-leptos:
    ports: ["8000:3000"]  # ✅ Frontend can expose
    networks: [product-internal]
    environment:
      - BFF_API_URL=http://backend-api:3100
```

### ❌ Wrong: Core Service Exposed
```yaml
services:
  rs-core-auth:
    ports:               # ❌ CRITICAL VIOLATION
      - "3000:3000"
    networks:
      - sha-net
```

---

**Author:** DevOps Working Group  
**Date:** 2025-01-06  
**Version:** 1.0  
**Replaces:** None  
**Related:** Canon Rule #03 (BFF Mandatory Boundary), Canon Rule #04 (Port Allocation Strategy)
