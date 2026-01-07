# Rule Capsule #05: Core Services Port Non-Exposure

**Decision Trigger:**
If configuring docker-compose ports, apply this rule immediately.

**Essence:** Core services NEVER expose ports to host. Internal only.

**Evita:**
- Security holes
- Accidental direct access
- Bypassing Traefik

**Always think:**
"Is this a core service? Then NO ports: section."

**Pattern:**
```yaml
# ❌ WRONG
ports:
  - "8001:8001"

# ✅ CORRECT
expose:
  - 8001
```

**When to apply:** Writing docker-compose for core services.
