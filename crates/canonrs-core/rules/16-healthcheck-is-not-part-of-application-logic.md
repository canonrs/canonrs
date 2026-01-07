# Canon Rule #16: Healthcheck Is Not Part of Application Logic

**Status:** `ACTIVE`  
**Enforcement:** `RECOMMENDED`  
**Scope:** All containerized services in orchestrated environments

---

## 🎯 Objective

Prevent orchestration deadlocks caused by poorly designed healthcheck dependencies. Understand that healthchecks are infrastructure concerns, not application requirements.

---

## 📋 The Anti-Pattern

### Problematic Pattern:

```yaml
services:
  backend:
    image: service # scratch image with no curl
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/health"]

  frontend:
    depends_on:
      backend:
        condition: service_healthy # ❌ Blocks forever
```

**What Happens:**

1. Backend starts successfully
2. Healthcheck tries to execute `curl`
3. `curl` doesn't exist in scratch image
4. Healthcheck fails indefinitely
5. Backend marked "unhealthy"
6. Frontend never starts (waiting for healthy backend)
7. **Stack deadlocked**

---

## 🧠 Core Principle

**Healthchecks are for orchestrators, not for applications.**

- **Application responsibility:** Start, listen, respond to requests
- **Orchestrator responsibility:** Determine if service is ready
- **NOT application responsibility:** Block other services from starting

---

## ✅ Correct Patterns

### Pattern 1: No Healthcheck Dependency (Recommended)

```yaml
services:
  backend:
    image: backend
    # No healthcheck needed

  frontend:
    image: frontend
    # No depends_on, handles backend unavailability gracefully
    environment:
      BACKEND_URL: http://backend:3000
```

**Why:** Frontend should retry/backoff when backend unavailable, not block startup.

### Pattern 2: Process-Based Dependency Only

```yaml
services:
  backend:
    image: backend

  frontend:
    depends_on:
      backend:
        condition: service_started # ✅ Not service_healthy
```

**Why:** Ensures backend container exists, but doesn't wait for readiness.

### Pattern 3: Application-Level Readiness

```rust
// Frontend handles backend unavailability
async fn call_backend() -> Result<Response> {
    for attempt in 1..=5 {
        match reqwest::get("http://backend:3000/api").await {
            Ok(resp) => return Ok(resp),
            Err(e) if attempt < 5 => {
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            }
            Err(e) => return Err(e),
        }
    }
}
```

**Why:** Resilience at application layer, not orchestration layer.

---

## 🚫 When Healthcheck Dependencies Fail

### Failure Mode 1: Incompatible Image

```yaml
backend:
  image: service # FROM scratch
  healthcheck:
    test: ["CMD", "curl", "http://localhost:3000/health"]
    # ❌ curl doesn't exist
```

**Symptom:** Container healthy by process state, marked unhealthy by Docker.

### Failure Mode 2: Circular Dependencies

```yaml
service-a:
  depends_on:
    service-b:
      condition: service_healthy

service-b:
  depends_on:
    service-a:
      condition: service_healthy
```

**Result:** Deadlock - neither service starts.

### Failure Mode 3: Slow Startup

```yaml
database:
  healthcheck:
    start_period: 10s # Too short for large DB restore
```

**Result:** Service marked unhealthy during legitimate initialization.

---

## ✅ When Healthcheck IS Appropriate

### Valid Use Cases:

1. **Load Balancer Registration:**

```yaml
# Traefik only sends traffic to healthy instances
labels:
  - "traefik.http.services.backend.loadbalancer.healthcheck.path=/health"
```

2. **Auto-Restart on Crash:**

```yaml
healthcheck:
  test: ["CMD-SHELL", "pgrep -x service || exit 1"]
restart: unless-stopped
```

3. **Zero-Downtime Deployment:**

```bash
   # Rolling update waits for health before proceeding
   docker service update --update-delay 10s backend
```

---

## 🏗️ Architecture Guidelines

### Microservices Startup Order

**❌ WRONG: Orchestration-Enforced Order**

```yaml
services:
  db:
    healthcheck: [...]

  backend:
    depends_on:
      db: { condition: service_healthy }

  frontend:
    depends_on:
      backend: { condition: service_healthy }
```

**Problem:** Fragile, slow, doesn't handle transient failures.

**✅ CORRECT: Self-Healing Services**

```yaml
services:
  db:
    # No healthcheck

  backend:
    # No depends_on
    # Retries DB connection internally

  frontend:
    # No depends_on
    # Shows "connecting" UI while backend unavailable
```

**Benefits:** Resilient, fast startup, handles runtime failures.

---

## 📝 Complete Example

### Before (Problematic):

```yaml
# docker-compose.yml
services:
  postgres:
    image: postgres:16
    healthcheck:
      test: ["CMD", "pg_isready"]

  backend:
    image: backend # scratch
    depends_on:
      postgres: { condition: service_healthy }
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/health"]

  frontend:
    image: frontend
    depends_on:
      backend: { condition: service_healthy }
```

**Issues:**

- Backend healthcheck fails (no curl in scratch)
- Frontend blocked indefinitely
- No resilience to transient failures

### After (Correct):

```yaml
# docker-compose.yml
services:
  postgres:
    image: postgres:16
    # No healthcheck - process state sufficient

  backend:
    image: backend
    environment:
      DATABASE_URL: postgres://postgres:5432/db
      # Application retries DB connection
    restart: unless-stopped

  frontend:
    image: frontend
    environment:
      BACKEND_URL: http://backend:3000
      # Application handles backend unavailability
    restart: unless-stopped
```

**Benefits:**

- All services start immediately
- Application-level retry logic handles temporary unavailability
- Resilient to transient failures
- Simple orchestration

---

## 🔍 Debugging Healthcheck Issues

```bash
# Check healthcheck status
docker ps --format "table {{.Names}}\t{{.Status}}"

# View healthcheck logs
docker inspect --format='{{json .State.Health}}' backend | jq

# Test healthcheck manually
docker exec backend /healthcheck-command

# Bypass healthcheck temporarily
docker-compose up --no-deps backend
```

---

## ✅ Best Practices

1. **Default to No Healthcheck:**

   - Process running = healthy
   - Add healthcheck only when specifically needed

2. **Never Block Critical Path:**

   - Databases can be `service_started`
   - Backends should not block frontends
   - Use `depends_on` for ordering, not readiness

3. **Application-Level Resilience:**

```rust
   // Better than orchestration dependency
   let client = reqwest::Client::builder()
       .timeout(Duration::from_secs(5))
       .build()?;

   // Retry with exponential backoff
   backoff::retry(|| client.get(url).send()).await?
```

4. **Document Healthcheck Rationale:**

```yaml
healthcheck:
  # Required for Kubernetes liveness probe
  # Not used for startup ordering
  test: ["CMD-SHELL", "exit 0"]
```

---

## 🔗 Related Rules

- **Canon Rule #15** (Scratch Images): Why curl healthchecks fail
- **Canon Rule #19** (Orchestration): Proper service dependency patterns

---

**Key Insight:** Readiness is the client's responsibility, not the server's obligation. Services should start fast and handle unavailable dependencies gracefully.

---

**Violations:** May cause orchestration failures  
**Last Updated:** 2025-01-07  
**Version:** 1.0

---
