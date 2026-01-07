# Canon Rule #04: Port Allocation Strategy

**Status:** Normative  
**Applies to:** All services in monorepo  
**Related:** Canon Rule #03 (BFF Mandatory Boundary)

---

## The Principle

**Port ranges define service visibility and access control boundaries.**

Every service MUST use ports from its designated range based on function and exposure level.

---

## The Problem

### Anti-pattern: Random Port Assignment
```yaml
# ❌ WRONG: No pattern
frontend: 
  ports: ["3050:3000"]
backend-api: 
  ports: ["8123:8000"]
rs-core-auth: 
  ports: ["5555:3000"]  # ❌ Core service exposed
worker: 
  ports: ["9999:9999"]
```

**Issues:**
- ❌ Impossible to identify service type by port
- ❌ Accidental exposure of internal services
- ❌ Port conflicts unpredictable
- ❌ No semantic meaning for monitoring/firewall rules
- ❌ Documentation becomes outdated immediately

---

## The Solution

### Port Ranges by Service Type

| Range       | Service Type           | Exposure    | Network      | Examples              |
|-------------|------------------------|-------------|--------------|----------------------|
| 8000-8099   | Frontend SSR (Leptos)  | 🌍 Public   | Traefik      | Leptos apps          |
| 8100-8199   | Frontend SSR (Next.js) | 🌍 Public   | Traefik      | Next.js apps         |
| 3000-3099   | Core APIs (Rust)       | 🔒 Internal | sha-net      | rs-core-auth:3000    |
| 3100-3199   | BFF APIs (Node/Rust)   | 🔒 Internal | product nets | backend-api:3100     |
| 3200-3299   | Workers/Consumers      | 🔒 Internal | sha-net      | rs-worker-email:3200 |
| 3300-3399   | Analytics/Tools        | 🔒 Internal | sha-net      | metrics:3300         |
| 443         | Public Gateway         | 🌍 Public   | Host         | Traefik HTTPS        |

### Infrastructure Ports (Fixed)
| Port | Service    | Exposure    | Access        |
|------|------------|-------------|---------------|
| 5432 | PostgreSQL | 🔒 Internal | sha-net       |
| 6380 | Redis      | 🔒 Internal | sha-net       |
| 9092 | Kafka      | 🔒 Internal | sha-net       |

---

## Mandatory Rules

### 1. Port Range Assignment

**Frontend Public Services (8000-8199):**
```yaml
# ✅ CORRECT
frontend-leptos:
  ports:
    - "8000:3000"  # Host:Container
  networks:
    - product-internal
```

**Core Services (3000-3099):**
```yaml
# ✅ CORRECT: No ports, only expose
rs-core-auth:
  expose:
    - "3000"  # Internal only
  networks:
    - sha-net
  # ❌ NEVER use 'ports:' for core services
```

**BFF Services (3100-3199):**
```yaml
# ✅ CORRECT
backend-api:
  ports:
    - "3100:3100"
  networks:
    - sha-net
    - product-internal
```

### 2. DEV = PROD Port Consistency

**MUST use same port in both environments:**
```yaml
# ✅ CORRECT
services:
  frontend:
    ports:
      - "8000:3000"  # Same in dev AND prod
```

**Rationale:**
- Eliminates "works in dev" bugs
- Simplifies documentation
- Consistent healthcheck URLs
- Traefik resolves environment via routing, not ports

### 3. Exposure Control

**Public Services:**
```yaml
# Only 8000-8199 can be exposed to host
frontend:
  ports:
    - "8000:3000"  # ✅ Public range
  labels:
    - "traefik.enable=true"
```

**Internal Services:**
```yaml
# 3000-3399 NEVER exposed
rs-core-auth:
  expose:
    - "3000"  # ✅ Internal only
  # ❌ NO ports: mapping
  # ❌ NO traefik labels
```

---

## Architecture Pattern
```
┌─────────────────────────────────────────────────────┐
│                    Internet                         │
└───────────────────────┬─────────────────────────────┘
                        │ 443 (HTTPS)
                        ▼
              ┌─────────────────┐
              │  Traefik (443)  │
              └────────┬────────┘
                       │
        ┌──────────────┼──────────────┐
        │              │              │
        ▼              ▼              ▼
   Port 8000      Port 8001      Port 8100
 ┌──────────┐  ┌──────────┐  ┌──────────┐
 │Frontend-1│  │Frontend-2│  │Frontend-3│
 │ (Leptos) │  │ (Leptos) │  │  (Next)  │
 └────┬─────┘  └────┬─────┘  └────┬─────┘
      │             │              │
      └─────────────┼──────────────┘
                    │ BFF_API_URL
                    ▼
            ┌──────────────┐
            │ Backend-API  │  Port 3100 (internal)
            │    (BFF)     │
            └──────┬───────┘
                   │ sha-net
        ┌──────────┼──────────┐
        ▼          ▼          ▼
   Port 3000  Port 3001  Port 6380
 ┌─────────┐┌─────────┐┌────────┐
 │rs-core- ││rs-core- ││ Redis  │
 │  auth   ││payments ││        │
 └─────────┘└─────────┘└────────┘
   🔒 Internal Only (no host ports)
```

---

## Service Type Definitions

### Frontend SSR (8000-8199)
- User-facing web applications
- Rendered on server (SSR)
- Exposed via Traefik
- Can call BFF only

**Example:**
```yaml
core-auth-frontend-leptos:
  ports: ["8000:3000"]
billing-frontend-next:
  ports: ["8100:3000"]
```

### Core APIs (3000-3099)
- Shared business logic services
- Never exposed to host
- Only accessible via sha-net
- Stateless, horizontally scalable

**Example:**
```yaml
rs-core-auth:
  expose: ["3000"]
rs-core-payments:
  expose: ["3001"]
```

### BFF APIs (3100-3199)
- Backend-For-Frontend pattern
- Gateway between frontend and core
- Exposed to host but not public
- Connected to sha-net + product network

**Example:**
```yaml
core-auth-backend-api:
  ports: ["3100:3100"]
billing-backend-api:
  ports: ["3101:3101"]
```

### Workers (3200-3299)
- Background job processors
- Message queue consumers
- No HTTP endpoints (typically)
- Internal metrics only

**Example:**
```yaml
rs-worker-email:
  expose: ["3200"]  # Metrics endpoint
rs-worker-reports:
  expose: ["3201"]
```

---

## Port Allocation Workflow

### Adding New Service

**Step 1: Identify service type**
```bash
# Is it user-facing? → 8000-8199
# Is it core business logic? → 3000-3099
# Is it BFF? → 3100-3199
# Is it worker? → 3200-3299
```

**Step 2: Find next available port**
```bash
# Check existing allocations
grep -r "ports:" products/*/docker-compose.yml | grep "8000-8099"

# Assign next free port
# e.g., 8002 for new Leptos frontend
```

**Step 3: Document in PORT_MAP.md**
```markdown
| Service             | Port | Type     |
|---------------------|------|----------|
| dashboard-frontend  | 8002 | Leptos   |
```

**Step 4: Configure compose**
```yaml
dashboard-frontend:
  ports:
    - "8002:3000"
```

---

## Debugging Checklist

### "Port already in use"
```bash
# 1. Find conflicting service
docker ps --format "{{.Names}}: {{.Ports}}" | grep "8000"

# 2. Check compose files
grep -r "8000:" products/*/docker-compose.yml

# 3. Assign next available in range
# e.g., 8000 taken → use 8001
```

### "Service not accessible"
```bash
# 1. Verify port mapping exists
docker compose ps service-name

# 2. Check if internal service mistakenly exposed
docker compose config | yq '.services.rs-core-auth.ports'
# Should be empty for core services

# 3. Verify network connectivity
docker compose exec frontend curl http://backend-api:3100/health
```

---

## Automated Enforcement

### CI/CD Validation
```bash
#!/bin/bash
# core-services/_rules/scripts/validate-ports.sh

set -e

echo "🔍 Validating port allocations..."

# 1. Check core services NEVER use 'ports:'
CORE_WITH_PORTS=$(grep -r "^  ports:" products/*/rs-core-*/docker-compose.yml 2>/dev/null || true)
if [[ -n "$CORE_WITH_PORTS" ]]; then
  echo "❌ VIOLATION: Core services cannot expose host ports"
  echo "$CORE_WITH_PORTS"
  exit 1
fi

# 2. Verify frontend uses 8000-8199
FRONTEND_PORTS=$(grep -A1 "frontend.*:" products/*/docker-compose.yml | grep "ports:" | grep -oP '"\K[0-9]+' | head -1)
for port in $FRONTEND_PORTS; do
  if [[ $port -lt 8000 || $port -gt 8199 ]]; then
    echo "❌ VIOLATION: Frontend port $port outside 8000-8199 range"
    exit 1
  fi
done

# 3. Verify BFF uses 3100-3199
BFF_PORTS=$(grep -A1 "backend-api.*:" products/*/docker-compose.yml | grep "ports:" | grep -oP '"\K[0-9]+' | head -1)
for port in $BFF_PORTS; do
  if [[ $port -lt 3100 || $port -gt 3199 ]]; then
    echo "❌ VIOLATION: BFF port $port outside 3100-3199 range"
    exit 1
  fi
done

# 4. Check for duplicate ports
DUPLICATES=$(grep -rh "ports:" products/*/docker-compose.yml | grep -oP '"\K[0-9]+(?=:)' | sort | uniq -d)
if [[ -n "$DUPLICATES" ]]; then
  echo "❌ VIOLATION: Duplicate host ports detected"
  echo "$DUPLICATES"
  exit 1
fi

echo "✅ Port allocation validation passed"
```

### Integration
```yaml
# .github/workflows/pr-check.yml
- name: Validate Port Allocation
  run: |
    chmod +x core-services/_rules/scripts/validate-ports.sh
    ./core-services/_rules/scripts/validate-ports.sh
```

---

## Migration Guide

### Existing Services → Standardized Ranges

**Step 1: Audit current ports**
```bash
grep -r "ports:" products/*/docker-compose.yml | sort
```

**Step 2: Map to standard ranges**
```bash
# Example migration:
# OLD: frontend-leptos: ["3050:3000"]
# NEW: frontend-leptos: ["8000:3000"]
```

**Step 3: Update compose files**
```bash
sed -i 's/"3050:3000"/"8000:3000"/' products/core-auth/docker-compose.yml
```

**Step 4: Update documentation**
```bash
# Update PORT_MAP.md
vim docs/PORT_MAP.md
```

**Step 5: Update Traefik routing**
```yaml
# Update Traefik rules to new ports
labels:
  - "traefik.http.services.frontend.loadbalancer.server.port=8000"
```

---

## Security Implications

### Defense in Depth

**1. Network Segmentation by Port**
- 8000-8199: Public-facing (firewall allows)
- 3000-3399: Internal only (firewall blocks)

**2. Monitoring & Alerting**
```bash
# Alert on unexpected port exposure
docker ps --format "{{.Ports}}" | grep -E "0.0.0.0:(3[0-3][0-9]{2})"
# Should return empty for core services
```

**3. Least Privilege**
- Core services have ZERO host exposure
- Only Traefik speaks to internet
- BFF isolates frontend from core

---

## Normative Status

- Violations **MUST** block deployment
- All new services **MUST** use designated ranges
- Port changes **MUST** go through architecture review
- CI/CD **MUST** enforce range compliance
- Core services **MUST NEVER** expose host ports

---

## Examples

### ✅ Correct: Range Compliance
```yaml
# Frontend (public)
dashboard-frontend:
  ports: ["8002:3000"]  # ✅ In 8000-8199

# BFF (internal gateway)
dashboard-backend-api:
  ports: ["3102:3102"]  # ✅ In 3100-3199

# Core (no exposure)
rs-core-analytics:
  expose: ["3002"]      # ✅ In 3000-3099, no ports:
```

### ❌ Wrong: Violations
```yaml
# ❌ Frontend outside range
frontend:
  ports: ["3050:3000"]  # Should be 8000-8199

# ❌ Core service exposed
rs-core-auth:
  ports: ["3000:3000"]  # Should use expose: only

# ❌ Random port
worker:
  ports: ["9999:9999"]  # Should be 3200-3299
```

---

## Related Documents

- [PORT_MAP.md](../docs/PORT_MAP.md) - Current port allocations
- [Canon Rule #03](./03-bff-mandatory-boundary.md) - BFF architecture
- [Canon Rule #05](./05-core-services-port-non-exposure.md) - Core service isolation

---

**Author:** DevOps Working Group  
**Date:** 2025-01-06  
**Version:** 1.0  
**Replaces:** None  
**Related:** Canon Rule #03 (BFF Mandatory Boundary)
