# Canon Rule #06: Traefik as the Only Public Ingress

**Status:** Normative  
**Applies to:** All services exposed to the internet  
**Related:** Canon Rule #03 (BFF Mandatory Boundary), Canon Rule #05 (Core Services Port Non-Exposure)

---

## The Principle

**Traefik MUST be the ONLY service exposing ports 80 and 443 to the host.**

All public HTTP/HTTPS traffic flows through Traefik, which routes to internal services.

---

## The Problem

### Anti-pattern: Multiple Public Entry Points
```yaml
# ❌ WRONG: Frontend bypassing Traefik
frontend:
  ports:
    - "80:3000"      # ❌ Direct HTTP exposure
    - "443:3000"     # ❌ Direct HTTPS exposure

# ❌ WRONG: BFF exposed directly
backend-api:
  ports:
    - "8443:3100"    # ❌ Custom HTTPS port

# Traefik also exists
traefik:
  ports:
    - "80:80"
    - "443:443"
```

**Issues:**
- ❌ **Security:** Multiple TLS termination points
- ❌ **Certificates:** Manual cert management per service
- ❌ **Monitoring:** Traffic scattered across entry points
- ❌ **Rate Limiting:** No centralized enforcement
- ❌ **Logging:** Fragmented access logs
- ❌ **Maintenance:** Multiple ingress configurations

---

## The Solution

### Correct: Single Ingress Gateway
```yaml
# Traefik (ONLY service with 80/443)
traefik:
  ports:
    - "80:80"       # ✅ HTTP (redirects to HTTPS)
    - "443:443"     # ✅ HTTPS (TLS termination)
  volumes:
    - /var/run/docker.sock:/var/run/docker.sock:ro
    - ./traefik.yml:/traefik.yml:ro
    - ./certs:/certs:ro

# Frontend (routed via Traefik)
frontend:
  labels:
    - "traefik.enable=true"
    - "traefik.http.routers.frontend.rule=Host(`app.example.com`)"
    - "traefik.http.routers.frontend.entrypoints=websecure"
    - "traefik.http.services.frontend.loadbalancer.server.port=3000"
  # ❌ NO ports: 80 or 443

# Backend-API (routed via Traefik)
backend-api:
  labels:
    - "traefik.enable=true"
    - "traefik.http.routers.api.rule=Host(`api.example.com`)"
    - "traefik.http.routers.api.entrypoints=websecure"
  # ❌ NO ports: 80 or 443
```

---

## Architecture Pattern
```
┌─────────────────────────────────────────────────────┐
│                    Internet                         │
│             (all public traffic)                    │
└───────────────────────┬─────────────────────────────┘
                        │
                   Port 80/443
                        │
                        ▼
              ┌─────────────────┐
              │    Traefik      │ ← ONLY public ingress
              │   (Port 80/443) │
              └────────┬────────┘
                       │
        ┌──────────────┼──────────────┐
        │              │              │
        ▼              ▼              ▼
  app.example.com  api.example.com  admin.example.com
        │              │              │
        ▼              ▼              ▼
   ┌─────────┐  ┌──────────┐  ┌──────────┐
   │Frontend │  │Backend   │  │Admin     │
   │(8000)   │  │API(3100) │  │Panel(8001)│
   └─────────┘  └──────────┘  └──────────┘
   
   Internal ports, NOT exposed to host
   Only accessible via Traefik routing
```

---

## Mandatory Requirements

### 1. Traefik Configuration

**Traefik MUST be the only service with ports 80/443:**
```yaml
# infrastructure/docker-compose.traefik.yml
services:
  traefik:
    image: traefik:v2.10
    container_name: traefik
    restart: unless-stopped
    ports:
      - "80:80"       # ✅ HTTP
      - "443:443"     # ✅ HTTPS
      - "8080:8080"   # ✅ Dashboard (internal only)
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - ./traefik.yml:/traefik.yml:ro
      - ./dynamic:/dynamic:ro
      - ./certs:/certs:ro
    networks:
      - sha-net
    labels:
      - "traefik.enable=true"
```

**Traefik Static Configuration:**
```yaml
# infrastructure/traefik.yml
api:
  dashboard: true
  insecure: false

entryPoints:
  web:
    address: ":80"
    http:
      redirections:
        entryPoint:
          to: websecure
          scheme: https
  
  websecure:
    address: ":443"
    http:
      tls:
        certResolver: letsencrypt

providers:
  docker:
    endpoint: "unix:///var/run/docker.sock"
    exposedByDefault: false
    network: sha-net

certificatesResolvers:
  letsencrypt:
    acme:
      email: admin@example.com
      storage: /certs/acme.json
      httpChallenge:
        entryPoint: web
```

### 2. Service Configuration

**Services MUST use Traefik labels, not direct port exposure:**
```yaml
# products/core-auth/docker-compose.yml
services:
  frontend-leptos:
    build: ...
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.core-auth-frontend.rule=Host(`auth.example.com`)"
      - "traefik.http.routers.core-auth-frontend.entrypoints=websecure"
      - "traefik.http.routers.core-auth-frontend.tls.certresolver=letsencrypt"
      - "traefik.http.services.core-auth-frontend.loadbalancer.server.port=3000"
    networks:
      - sha-net
      - product-internal
    # ❌ NO ports: 80 or 443
    # ❌ NO ports: 8000 (if publicly accessible)
```

### 3. Development vs Production

**Development (with direct ports for debugging):**
```yaml
# docker-compose.dev.yml
services:
  frontend:
    ports:
      - "8000:3000"  # ✅ OK for dev debugging
    labels:
      - "traefik.enable=false"  # Disable Traefik in dev
```

**Production (Traefik only):**
```yaml
# docker-compose.yml
services:
  frontend:
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.frontend.rule=Host(`app.example.com`)"
    # ❌ NO ports: key in production
```

---

## Traefik Routing Patterns

### Basic HTTP Routing
```yaml
labels:
  - "traefik.enable=true"
  - "traefik.http.routers.myapp.rule=Host(`app.example.com`)"
  - "traefik.http.routers.myapp.entrypoints=websecure"
  - "traefik.http.routers.myapp.tls.certresolver=letsencrypt"
  - "traefik.http.services.myapp.loadbalancer.server.port=3000"
```

### Path-Based Routing
```yaml
labels:
  - "traefik.enable=true"
  - "traefik.http.routers.api.rule=Host(`example.com`) && PathPrefix(`/api`)"
  - "traefik.http.middlewares.api-strip.stripprefix.prefixes=/api"
  - "traefik.http.routers.api.middlewares=api-strip"
```

### Multiple Domains
```yaml
labels:
  - "traefik.enable=true"
  - "traefik.http.routers.app.rule=Host(`app.example.com`) || Host(`app.example.net`)"
```

### HTTPS Redirect
```yaml
labels:
  - "traefik.http.routers.app-http.rule=Host(`app.example.com`)"
  - "traefik.http.routers.app-http.entrypoints=web"
  - "traefik.http.routers.app-http.middlewares=redirect-to-https"
  - "traefik.http.middlewares.redirect-to-https.redirectscheme.scheme=https"
```

---

## Security Benefits

### Centralized Security Enforcement

**1. TLS Termination**
- Single point for certificate management
- Automatic Let's Encrypt renewal
- Consistent TLS configuration
- No per-service cert complexity

**2. Rate Limiting**
```yaml
# Traefik middleware
http:
  middlewares:
    rate-limit:
      rateLimit:
        average: 100
        burst: 50
```

**3. Authentication**
```yaml
# Traefik middleware
http:
  middlewares:
    basic-auth:
      basicAuth:
        users:
          - "admin:$apr1$xyz..."
```

**4. IP Whitelisting**
```yaml
# Traefik middleware
http:
  middlewares:
    ip-whitelist:
      ipWhiteList:
        sourceRange:
          - "10.0.0.0/8"
          - "192.168.0.0/16"
```

---

## Monitoring & Observability

### Access Logs
```yaml
# traefik.yml
accessLog:
  filePath: "/logs/access.log"
  format: json
  fields:
    defaultMode: keep
    headers:
      defaultMode: keep
```

### Metrics
```yaml
# traefik.yml
metrics:
  prometheus:
    entryPoint: metrics
    addEntryPointsLabels: true
    addServicesLabels: true

entryPoints:
  metrics:
    address: ":8082"
```

### Dashboard
```yaml
labels:
  - "traefik.http.routers.dashboard.rule=Host(`traefik.example.com`)"
  - "traefik.http.routers.dashboard.service=api@internal"
  - "traefik.http.routers.dashboard.middlewares=auth"
```

---

## Debugging Checklist

### "Service not accessible via domain"
```bash
# 1. Verify Traefik is running
docker ps | grep traefik

# 2. Check Traefik logs
docker logs traefik --tail 50

# 3. Verify service has correct labels
docker inspect <service> | jq '.[0].Config.Labels'

# 4. Check Traefik routing table
curl http://localhost:8080/api/http/routers

# 5. Test DNS resolution
nslookup app.example.com

# 6. Verify service is in sha-net
docker network inspect sha-net | jq '.[0].Containers'
```

### "Certificate not working"
```bash
# 1. Check acme.json permissions
ls -la infrastructure/certs/acme.json
# Should be 600

# 2. Verify Let's Encrypt logs
docker logs traefik | grep acme

# 3. Check certificate resolver config
docker exec traefik cat /traefik.yml | grep -A10 certificatesResolvers

# 4. Test HTTPS manually
curl -vI https://app.example.com
```

---

## Migration Guide

### Existing Direct Exposure → Traefik

**Step 1: Install Traefik**
```bash
cd infrastructure
docker compose -f docker-compose.traefik.yml up -d
```

**Step 2: Update service configuration**
```yaml
# Before
frontend:
  ports:
    - "80:3000"
    - "443:3000"

# After
frontend:
  labels:
    - "traefik.enable=true"
    - "traefik.http.routers.frontend.rule=Host(`app.example.com`)"
    - "traefik.http.routers.frontend.entrypoints=websecure"
    - "traefik.http.services.frontend.loadbalancer.server.port=3000"
  networks:
    - sha-net
```

**Step 3: Remove port bindings**
```bash
sed -i '/ports:/,+2d' products/*/docker-compose.yml
```

**Step 4: Add Traefik labels**
```bash
# Add to each public-facing service
vim products/core-auth/docker-compose.yml
```

**Step 5: Restart services**
```bash
docker compose down
docker compose up -d
```

---

## Automated Enforcement

### Validation Script
```bash
#!/bin/bash
# core-services/_rules/scripts/validate-traefik-ingress.sh

set -e

echo "🔍 [Rule #06] Validating Traefik Single Public Ingress..."

VIOLATIONS=0

# 1. Check only Traefik uses port 80
PORT_80=$(find infrastructure products -name "docker-compose*.yml" -exec grep -h "\"80:" {} \; 2>/dev/null | wc -l)
if [[ $PORT_80 -gt 1 ]]; then
  echo "❌ VIOLATION: Multiple services expose port 80"
  find infrastructure products -name "docker-compose*.yml" -exec grep -l "\"80:" {} \;
  VIOLATIONS=$((VIOLATIONS + 1))
fi

# 2. Check only Traefik uses port 443
PORT_443=$(find infrastructure products -name "docker-compose*.yml" -exec grep -h "\"443:" {} \; 2>/dev/null | wc -l)
if [[ $PORT_443 -gt 1 ]]; then
  echo "❌ VIOLATION: Multiple services expose port 443"
  find infrastructure products -name "docker-compose*.yml" -exec grep -l "\"443:" {} \;
  VIOLATIONS=$((VIOLATIONS + 1))
fi

# 3. Verify Traefik exists
if ! docker ps | grep -q traefik; then
  echo "⚠️  WARNING: Traefik container not running"
fi

# 4. Check public services use Traefik labels
PUBLIC_SERVICES=$(find products -name "docker-compose.yml" -exec grep -l "frontend\|public" {} \;)
for file in $PUBLIC_SERVICES; do
  HAS_TRAEFIK=$(grep -c "traefik.enable" "$file" || echo "0")
  HAS_PORTS=$(grep -c "\"80:\|\"443:" "$file" || echo "0")
  
  if [[ $HAS_PORTS -gt 0 ]]; then
    echo "⚠️  WARNING: Public service in $file exposes 80/443 directly"
  fi
  
  if [[ $HAS_TRAEFIK -eq 0 ]]; then
    echo "ℹ️  INFO: $file has no Traefik configuration"
  fi
done

if [[ $VIOLATIONS -eq 0 ]]; then
  echo "✅ [Rule #06] Traefik Single Public Ingress validation passed"
  exit 0
else
  echo "❌ [Rule #06] Traefik Single Public Ingress validation FAILED with $VIOLATIONS violation(s)"
  exit 1
fi
```

---

## Exception Policy

### When Direct Port Exposure is Allowed

**✅ Development Environment:**
```yaml
# docker-compose.dev.yml
services:
  frontend:
    ports:
      - "8000:3000"  # ✅ OK for local debugging
```

**✅ Non-HTTP Services:**
```yaml
# SSH, databases, etc. (not web traffic)
ssh-bastion:
  ports:
    - "2222:22"  # ✅ OK (not HTTP/HTTPS)
```

**❌ Never Allowed in Production:**
- Direct HTTP/HTTPS exposure
- Custom HTTPS ports (8443, etc.)
- Bypassing Traefik for web services

---

## Normative Status

- Violations **MUST** block production deployment
- Development environments **MAY** use direct ports
- Traefik **MUST** be running before deploying public services
- All web traffic **MUST** flow through Traefik
- Exceptions require architecture committee approval

---

## Examples

### ✅ Correct: Full Traefik Setup
```yaml
# infrastructure/docker-compose.traefik.yml
services:
  traefik:
    image: traefik:v2.10
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - ./traefik.yml:/traefik.yml:ro

# products/core-auth/docker-compose.yml
services:
  frontend:
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.frontend.rule=Host(`auth.example.com`)"
    networks:
      - sha-net
    # ❌ NO ports: 80 or 443
```

### ❌ Wrong: Direct HTTP Exposure
```yaml
services:
  frontend:
    ports:
      - "80:3000"   # ❌ Bypasses Traefik
      - "443:3000"  # ❌ Multiple TLS endpoints
```

---

**Author:** DevOps Working Group  
**Date:** 2025-01-06  
**Version:** 1.0  
**Replaces:** None  
**Related:** Canon Rule #03 (BFF Mandatory Boundary), Canon Rule #05 (Core Services Port Non-Exposure)
