# Rule Capsule #04: Port Allocation Strategy

**Decision Trigger:**
If defining new service ports, apply this rule immediately.

**Essence:** Ports follow strict ranges. No random allocation.

**Evita:**
- Port conflicts
- Unclear service identification
- Traefik routing confusion

**Always think:**
"What port range does this service category use?"

**Pattern:**
```
Core services: 8000-8099
BFFs: 3000-3099
Frontends: 5000-5099
Infrastructure: 9000-9099
```

**When to apply:** Creating new service in docker-compose.
