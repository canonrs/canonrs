# Canon Rule #02: One Compose per Product

**Status:** Normative  
**Applies to:** All products in monorepo with multiple services  
**Related:** Canon Rule #01 (Docker Build Cache Invalidation)

---

## The Principle

**One `docker-compose.yml` per PRODUCT, not per service.**

A product is defined as:
- A user-facing application boundary
- Services that deploy atomically together
- Components with coupled lifecycle and versioning
- A unit of ownership by a single team

---

## The Problem

### Anti-pattern: One compose per service
```bash
products/core-auth/
├── backend-api/
│   └── docker-compose.yml      # ❌ Separate compose
└── frontend-leptos/
    └── docker-compose.yml      # ❌ Separate compose
```

**Issues:**
- ❌ Version skew (frontend v2 + backend v1)
- ❌ Deployment complexity (2 separate commands)
- ❌ Network configuration duplicated
- ❌ Unclear ownership boundaries
- ❌ Difficult rollback (which service?)
- ❌ Secrets scattered across composes

---

## The Solution

### Correct: One compose per product
```bash
products/core-auth/
├── docker-compose.yml          # ✅ Product boundary
├── docker-compose.dev.yml      # ✅ Dev overrides
├── .env.example
├── backend-api/
│   ├── Dockerfile
│   ├── .env
│   └── src/
└── frontend-leptos/
    ├── Dockerfile
    └── src/
```

**Benefits:**
- ✅ Atomic deployment
- ✅ Consistent versioning
- ✅ Single network topology
- ✅ Clear ownership
- ✅ Simple rollback
- ✅ Centralized secrets

---

## Architecture Pattern

### Product Compose Structure
```yaml
version: '3.9'

services:
  # Backend (BFF pattern)
  backend-api:
    build: ...
    networks:
      - sha-net          # External: core services
      - product-internal # Internal: product services
    healthcheck: ...
    
  # Frontend (SSR)
  frontend-leptos:
    build: ...
    depends_on:
      backend-api:
        condition: service_healthy
    networks:
      - product-internal # Internal only
    healthcheck: ...

networks:
  sha-net:
    external: true       # Shared with infrastructure
  
  product-internal:
    driver: bridge
    internal: true       # Zero Trust: no external access
```

### Network Topology
```
┌─────────────────────────────────────────────────┐
│          sha-net (infrastructure)               │
│  ┌──────────────┐         ┌──────────────┐     │
│  │ Core Service │◄────────┤ Backend-API  │     │
│  │              │  mTLS   │     (BFF)    │     │
│  └──────────────┘         └──────┬───────┘     │
│                                   │             │
│       product-internal (private network)       │
│                                   │             │
│                         ┌─────────▼────────┐   │
│                         │   Frontend-SSR   │   │
│                         └──────────────────┘   │
└─────────────────────────────────────────────────┘
         ▲                          ▲
         │                          │
    Port 3100 (API)           Port 3101 (User)
```

---

## Mandatory Requirements

### 1. Service Dependencies
```yaml
frontend:
  depends_on:
    backend:
      condition: service_healthy  # ✅ Wait for backend ready
```

**Rationale:** Prevents startup race conditions.

### 2. Healthchecks
```yaml
backend:
  healthcheck:
    test: ["CMD", "curl", "-f", "http://localhost:3100/health"]
    interval: 10s
    timeout: 5s
    retries: 3
    start_period: 30s  # ✅ Grace period for slow starts

frontend:
  healthcheck:
    test: ["CMD", "curl", "-f", "http://localhost:3000/"]
    interval: 10s
    timeout: 5s
    retries: 3
    start_period: 40s  # ✅ After backend stabilizes
```

**Rationale:** Ensures `depends_on` waits for actual readiness.

### 3. Network Isolation
```yaml
networks:
  product-internal:
    internal: true  # ✅ Zero external access to internal network
```

**Rationale:** Defense in depth - only exposed ports accessible.

### 4. Environment Validation

Backend `main.rs` MUST validate required env vars on startup:
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Panic if critical env missing
    let _ = std::env::var("RS_CORE_AUTH_URL")
        .expect("RS_CORE_AUTH_URL must be set");
    let _ = std::env::var("REDIS_URL")
        .expect("REDIS_URL must be set");
    
    // Continue startup...
}
```

**Rationale:** Fail fast prevents "green but broken" containers.

---

## Development vs Production

### Production
```bash
# Start infrastructure first
cd /opt/docker/monorepo/infrastructure
docker compose -f docker-compose.core.yml up -d

# Start product
cd /opt/docker/monorepo/products/core-auth
docker compose up -d

# Verify health
docker compose ps
curl http://localhost:3100/health
curl http://localhost:3101/
```

### Development
```bash
cd /opt/docker/monorepo/products/core-auth

# Hot reload mode
docker compose -f docker-compose.yml -f docker-compose.dev.yml up

# Rebuild after dependency changes (not code changes - use hot reload)
docker compose -f docker-compose.yml -f docker-compose.dev.yml up --build
```

**Key:** Dev uses volume mounts + `cargo watch`, NOT Docker rebuilds.

---

## File Structure Requirements

### Mandatory Files
```
products/{product}/
├── docker-compose.yml          # ✅ Production config
├── docker-compose.dev.yml      # ✅ Dev overrides
├── .env.example                # ✅ Documented env template
├── README.md                   # ✅ Product documentation
└── {service}/
    ├── Dockerfile              # ✅ Production build
    ├── Dockerfile.dev          # ✅ Dev build (optional)
    ├── .env                    # ❌ NOT in git
    └── src/
```

### README.md Template
```markdown
# Product: {product}

## Architecture

- Backend-API (BFF): Port 3100
- Frontend-Leptos (SSR): Port 3101

## Dependencies

- rs-core-auth (infrastructure)
- Redis (infrastructure)

## Quick Start

\`\`\`bash
# Production
docker compose up -d

# Development
docker compose -f docker-compose.yml -f docker-compose.dev.yml up
\`\`\`

## Environment Variables

See `.env.example` in each service directory.
```

---

## CI/CD Integration

### Build Pipeline
```yaml
# .github/workflows/deploy-core-auth.yml
name: Deploy core-auth

on:
  push:
    paths:
      - 'products/core-auth/**'

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Build product images
        run: |
          cd products/core-auth
          docker compose build --no-cache
      
      - name: Deploy product
        run: |
          cd products/core-auth
          docker compose up -d
      
      - name: Health check
        run: |
          timeout 60 bash -c 'until curl -f http://localhost:3100/health; do sleep 2; done'
          timeout 60 bash -c 'until curl -f http://localhost:3101/; do sleep 2; done'
```

---

## When to Create New Compose

### ✅ Create new compose when:

- New user-facing product
- Services deploy together atomically
- Shared lifecycle and versioning
- Single team ownership

### ❌ Don't create new compose when:

- Just adding a service to existing product
- Splitting for "organizational" reasons
- Microservice within same product boundary
- Shared library or package

---

## Exceptions

### Allowed: Multiple composes for same product
```
products/core-auth/
├── docker-compose.yml          # Main product
├── docker-compose.staging.yml  # Staging overrides
├── docker-compose.dev.yml      # Dev overrides
└── docker-compose.test.yml     # Integration tests
```

**Rule:** Base `docker-compose.yml` + environment-specific overrides.

---

## Comparison with Alternatives

| Approach | Pros | Cons | Verdict |
|----------|------|------|---------|
| **1 compose/product** | Atomic, clear ownership | Larger files | ✅ **Recommended** |
| 1 compose/service | Flexible | Version skew, complexity | ❌ Anti-pattern |
| Monolithic compose | Simple initially | Scales poorly | ❌ Not enterprise |
| Kubernetes | Ultimate flexibility | Overkill for Docker | ⚠️ Future consideration |

---

## Normative Status

- Violations **MUST** block PRs
- All new products **MUST** follow this pattern
- Existing products **SHOULD** migrate gradually
- Exceptions require architecture review approval

---

## Examples

### ✅ Correct: Product boundary
```yaml
# products/billing/docker-compose.yml
services:
  billing-api:
    ...
  billing-dashboard:
    ...
  billing-worker:
    ...
```

All billing services in one compose.

### ❌ Wrong: Service boundary
```yaml
# products/billing/api/docker-compose.yml
services:
  billing-api:
    ...

# products/billing/dashboard/docker-compose.yml
services:
  billing-dashboard:
    ...
```

Services split unnecessarily.

---

**Author:** DevOps Working Group  
**Date:** 2025-01-06  
**Version:** 1.0  
**Replaces:** None  
**Related:** Canon Rule #01 (Docker Build Cache Invalidation)
