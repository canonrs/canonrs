# Canon Rule #19: Docker CLI vs Compose - Never Mix

**Status:** `ACTIVE`  
**Enforcement:** `MANDATORY`  
**Scope:** All container debugging and orchestration

---

## 🎯 Objective

Prevent orchestration conflicts by maintaining strict separation between manual Docker CLI usage and Docker Compose management.

---

## 📋 The Problem

### Conflict Scenario:
```bash
# 1. Debug with Docker CLI
docker run -d --name backend -p 3000:3000 backend-image

# 2. Later, try Compose
docker compose up -d
# Error: container name "backend" already in use

# 3. Now you have:
# - Orphaned CLI container
# - Compose can't manage it
# - Port conflicts
# - Invisible state
```

---

## 🧠 Why This Breaks

**Docker Compose Expectations:**
- Creates containers with specific naming convention
- Manages container lifecycle
- Tracks state in internal database
- Cannot adopt existing containers

**Docker CLI Containers:**
- Independent of Compose
- Not tracked by Compose
- Block Compose from creating containers
- Create port/network conflicts

---

## ✅ Correct Discipline

### During Debugging:
```bash
# Use ONLY Docker CLI
docker run --rm -it backend-image
docker run -d --name test-backend backend-image
docker logs test-backend
docker exec test-backend /bin/sh
```

**Rules:**
- Always use `--rm` for temporary containers
- Use unique names (`test-*`) to avoid conflicts
- Clean up when done: `docker rm -f test-backend`

### During Normal Operation:
```bash
# Use ONLY Docker Compose
docker compose up -d
docker compose logs backend
docker compose exec backend /bin/sh
docker compose down
```

**Rules:**
- Never manually create containers Compose expects to manage
- Let Compose handle all lifecycle operations

---

## 🚫 Never Do This
```bash
# ❌ Start manually
docker run -d --name backend backend-image

# ❌ Then try to manage with Compose
docker compose up -d
# Conflict!

# ❌ Mix commands on same container
docker compose up -d backend
docker stop backend  # Confuses Compose state
docker compose up -d  # Unexpected behavior
```

---

## 🔧 Recovery from Mixed State

### Symptoms:
- `docker compose up` shows "already in use"
- Containers exist but not shown in `docker compose ps`
- Port conflicts despite `docker compose down`

### Clean Up:
```bash
# 1. Stop ALL containers
docker stop $(docker ps -aq)

# 2. Remove ALL containers
docker rm -f $(docker ps -aq)

# 3. Clean Compose state
docker compose down --remove-orphans

# 4. Verify clean state
docker ps -a
# Should be empty

# 5. Start fresh with Compose ONLY
docker compose up -d
```

---

## 📝 Debugging Workflow

### Scenario: Backend not working in Compose
```bash
# ❌ WRONG: Mix CLI and Compose
docker compose up -d
docker run --rm backend-image  # Different environment!

# ✅ CORRECT: Debug within Compose context
docker compose up -d
docker compose logs backend
docker compose exec backend /bin/sh

# OR: Debug entirely with CLI
docker compose down
docker run --rm -it \
  -p 3000:3000 \
  -e ENV_VAR=value \
  backend-image
```

---

## 🏗️ Architecture Pattern

### Development Setup:
```yaml
# docker-compose.dev.yml
services:
  backend:
    build: .
    volumes:
      - ./src:/app/src
    ports:
      - "3000:3000"
    # Compose manages lifecycle
```

**Use:** `docker compose -f docker-compose.dev.yml up`

### Production Setup:
```yaml
# docker-compose.yml
services:
  backend:
    image: backend:v1.0.0
    ports:
      - "3000:3000"
    restart: unless-stopped
```

**Use:** `docker compose up -d --no-build`

### Isolated Testing:
```bash
# Not using Compose at all
docker run --rm \
  --name isolated-test \
  -p 3001:3000 \
  backend:v1.0.0
```

**Use:** One-off testing, no interference with Compose.

---

## ✅ Decision Matrix

| Scenario | Use Docker CLI | Use Docker Compose |
|----------|----------------|-------------------|
| Quick test of image | ✅ | ❌ |
| Debug single container | ✅ | ❌ |
| Run multi-container stack | ❌ | ✅ |
| Production deployment | ❌ | ✅ |
| CI/CD pipeline | ❌ | ✅ |
| Network testing | ❌ | ✅ |

---

## 🔍 Verification
```bash
# Check for orphaned containers
docker ps -a --filter "name=backend" --format "{{.Names}}\t{{.Image}}\t{{.Status}}"

# Check Compose state
docker compose ps
If they don't match: CLEANUP NEEDED

---

## 🔗 Related Rules

- **Canon Rule #18** (Docker Cache): Consistent build approach
- **Canon Rule #16** (Healthcheck): Orchestration dependencies

---

**Key Insight:** Pick one tool per environment. Docker CLI for ad-hoc testing, Compose for anything involving multiple containers or persistent state.

---

**Last Updated:** 2025-01-07  
**Version:** 1.0
