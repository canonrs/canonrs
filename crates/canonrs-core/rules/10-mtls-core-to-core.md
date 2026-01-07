# Canon Rule #10: mTLS Core-to-Core Communication

**Status:** `ACTIVE`  
**Enforcement:** `MANDATORY`  
**Scope:** All inter-service communication between Core Services

---

## 🎯 Objective

Enforce mutual TLS (mTLS) authentication for all Core-to-Core communication, ensuring cryptographic identity verification and preventing unauthorized service impersonation.

---

## 📋 Requirements

### 1. Certificate Infrastructure

**MUST:**
- Use Vault PKI as single source of truth for certificates
- Store certificates in `/luks-data/monorepo/{core-services|products}/<service>/certs/`
- Separate `server/` and `client/` certificates by role
- Include full CA chain (`ca-chain.pem`)

**RECOMMENDED:**
- Prefer `SAN=URI:spiffe://core/<service>` or `SAN=DNS:<service>.internal`
- CN alone is insufficient in modern PKI

**Structure:**
```
/luks-data/monorepo/core-services/rs-core-auth/certs/
├── server/
│   ├── server.crt
│   ├── server.key
│   └── ca-chain.pem
└── client/
    ├── client.crt
    ├── client.key
    └── ca-chain.pem
```

### 2. Server Configuration (Axum/Actix)

**MUST:**
- Configure TLS with `require_client_cert = true`
- Validate client certificate against CA chain
- Verify client CN/SAN matches expected service identity
- Reject connections without valid client cert

### 3. Client Configuration (reqwest)

**MUST:**
- Use client certificate for all outbound requests
- Verify server certificate against CA chain
- Set proper timeouts and retry logic
- Use connection pooling

### 4. Service Identity Validation

**MUST:**
- Define canonical service identifiers in `SERVICE_REGISTRY.md`
- Validate CN/SAN in certificates match registry
- Log certificate subject on each connection
- Reject certificates with unexpected identities

---

## 🚫 Prohibited Patterns

**NEVER:**
- Use plain HTTP between Core Services
- Skip certificate validation (`danger_accept_invalid_certs`)
- Use self-signed certs outside Vault PKI
- Share private keys between services
- Store certificates in git/artifacts
- Terminate mTLS at Traefik for core services

---

## 🔍 Validation

Run: `./core-services/_rules/scripts/validate-mtls-core.sh`

---

**Rationale:** mTLS provides defense-in-depth by ensuring both client and server cryptographically prove their identity, preventing lateral movement and service impersonation in case of network compromise.
