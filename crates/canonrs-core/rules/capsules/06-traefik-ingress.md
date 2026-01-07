# Rule Capsule #06: Traefik Single Public Ingress

**Decision Trigger:**
If exposing service publicly, apply this rule immediately.

**Essence:** Traefik is the ONLY public entry. Everything else internal.

**Evita:**
- Multiple ingress points
- TLS management chaos
- Rate limiting bypass

**Always think:**
"Does external world need this? Then Traefik route only."

**Pattern:**
```yaml
labels:
  - "traefik.http.routers.X.rule=Host(`domain.com`)"
  - "traefik.http.routers.X.tls=true"
```

**When to apply:** Making any service publicly accessible.
