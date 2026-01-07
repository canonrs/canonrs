# Canon Rule #07: Environment Variable Classification

**Status:** Normative  
**Applies to:** All services in monorepo  
**Related:** Canon Rule #03 (BFF Mandatory Boundary), Canon Rule #05 (Core Services Port Non-Exposure)

---

## The Principle

**Environment variables MUST be classified by sensitivity level and stored accordingly.**

Each classification has specific storage requirements, access controls, and deployment patterns.

---

## The Problem

### Anti-pattern: Undifferentiated Secrets Management
```bash
# .env (committed to git)
DATABASE_URL=postgresql://user:password@localhost:5432/db
API_KEY=sk_live_51A1b2c3d4e5f6g7h8i9j0
DEBUG_MODE=true
APP_NAME=my-app
REDIS_URL=redis://localhost:6379
JWT_SECRET=supersecretkey123
LOG_LEVEL=info
```

**Issues:**
- ❌ **Security:** Secrets in version control
- ❌ **Compliance:** No audit trail for secret access
- ❌ **Rotation:** Manual process across all environments
- ❌ **Leak Risk:** Secrets in logs, error messages, or backups
- ❌ **Sharing:** Same secrets across dev/staging/prod

---

## The Solution

### Classification System

| Level | Description | Storage | Access | Examples |
|-------|-------------|---------|--------|----------|
| **Public** | Non-sensitive configuration | `.env.example`, docs | Anyone | `APP_NAME`, `LOG_LEVEL` |
| **Internal** | Service URLs, non-secret config | `.env` (gitignored) | Team members | `REDIS_URL`, `BFF_API_URL` |
| **Secret** | Credentials, API keys, tokens | Vault, K8s secrets | Restricted | `JWT_SECRET`, `DB_PASSWORD` |
| **Critical** | Production secrets, encryption keys | Vault + HSM | Ops only | `ENCRYPTION_KEY`, `ROOT_TOKEN` |

---

## Classification Details

### Public Variables

**Definition:** Configuration that can be safely committed to version control.

**Storage:**
```bash
# .env.example (committed)
APP_NAME=core-auth
LOG_LEVEL=info
ENVIRONMENT=development
RUST_BACKTRACE=1
LEPTOS_SITE_ADDR=0.0.0.0:3000
```

**Rules:**
- ✅ MUST be in `.env.example`
- ✅ MUST be documented
- ✅ CAN be in docker-compose.yml
- ❌ NEVER contain credentials

**Examples:**
- Application name
- Log level
- Feature flags (non-sensitive)
- Port numbers
- Timeouts (non-critical)

---

### Internal Variables

**Definition:** Service endpoints and configuration that shouldn't be public but aren't credentials.

**Storage:**
```bash
# .env (gitignored)
RS_CORE_AUTH_URL=http://rs-core-auth:3000
REDIS_URL=redis://172.32.6.51:6379
BFF_API_URL=http://backend-api:3100
DATABASE_HOST=postgres.internal
```

**Rules:**
- ✅ MUST be in `.env` (gitignored)
- ✅ MUST have example in `.env.example`
- ✅ CAN be in docker-compose.yml for dev
- ❌ NOT in version control
- ⚠️ Example values MUST be clearly fake

**Examples:**
- Service URLs (internal network)
- Database hostnames (without credentials)
- Redis connection strings (without passwords)
- Internal API endpoints

---

### Secret Variables

**Definition:** Credentials, tokens, and keys that grant access or decrypt data.

**Storage:**
```bash
# Vault path: secret/core-auth/prod
JWT_SECRET=<generated-by-vault>
DATABASE_PASSWORD=<generated-by-vault>
API_KEY=<generated-by-vault>
OAUTH_CLIENT_SECRET=<generated-by-vault>
```

**Rules:**
- ✅ MUST be in Vault or equivalent secret manager
- ✅ MUST be injected at runtime
- ✅ MUST rotate regularly (30-90 days)
- ❌ NEVER in `.env` files
- ❌ NEVER in docker-compose.yml
- ❌ NEVER in version control
- ❌ NEVER in logs or error messages

**Examples:**
- Database passwords
- JWT signing keys
- OAuth client secrets
- API keys (third-party services)
- Session encryption keys

---

### Critical Variables

**Definition:** Secrets that compromise entire system if leaked.

**Storage:**
```bash
# Vault + HSM: secret/infrastructure/root
ROOT_DATABASE_PASSWORD=<vault-hsm>
ENCRYPTION_MASTER_KEY=<vault-hsm>
VAULT_ROOT_TOKEN=<vault-hsm>
TLS_PRIVATE_KEY=<vault-hsm>
```

**Rules:**
- ✅ MUST be in Vault with HSM backend
- ✅ MUST require 2FA for access
- ✅ MUST have audit logging
- ✅ MUST rotate on schedule
- ✅ MUST be environment-specific
- ❌ NEVER accessed by application code directly
- ❌ NEVER logged under any circumstances

**Examples:**
- Root database credentials
- Master encryption keys
- Vault root tokens
- Certificate private keys
- Infrastructure admin passwords

---

## Storage Implementation

### .env.example (Public)
```bash
# products/core-auth/backend-api/.env.example

# === PUBLIC CONFIGURATION ===
APP_NAME=core-auth-backend-api
ENVIRONMENT=development
LOG_LEVEL=info
RUST_BACKTRACE=1

# === INTERNAL (replace with real values) ===
RS_CORE_AUTH_URL=http://rs-core-auth:3000
REDIS_URL=redis://redis:6379
BFF_PORT=3100

# === SECRETS (use Vault in production) ===
# JWT_SECRET=<vault:secret/core-auth/jwt_secret>
# DATABASE_PASSWORD=<vault:secret/core-auth/db_password>
# Never put real secrets here!
```

### .env (Internal + Development Secrets)
```bash
# products/core-auth/backend-api/.env
# ⚠️ This file is gitignored

# Internal config
RS_CORE_AUTH_URL=http://rs-core-auth:3000
REDIS_URL=redis://172.32.6.51:6379

# Development-only secrets (NOT for production)
JWT_SECRET=dev_secret_change_in_prod_minimum_32_chars
DATABASE_PASSWORD=dev_password
```

### Vault (Production Secrets)
```bash
# Write secret to Vault
vault kv put secret/core-auth/prod \
  jwt_secret="$(openssl rand -base64 32)" \
  database_password="$(openssl rand -base64 32)" \
  api_key="${API_KEY}"

# Read in application
vault kv get -field=jwt_secret secret/core-auth/prod
```

### Docker Compose (Development)
```yaml
services:
  backend-api:
    environment:
      # Public vars (inline OK)
      - APP_NAME=core-auth-backend-api
      - LOG_LEVEL=info
      
      # Internal vars (from .env file)
      - RS_CORE_AUTH_URL=${RS_CORE_AUTH_URL}
      - REDIS_URL=${REDIS_URL}
      
      # Secrets (dev only, NEVER in production compose)
      - JWT_SECRET=${JWT_SECRET:-dev_secret_change_in_prod}
    env_file:
      - .env
```

### Docker Compose (Production)
```yaml
services:
  backend-api:
    environment:
      # Public vars only
      - APP_NAME=core-auth-backend-api
      - LOG_LEVEL=info
      
      # Internal vars from .env
      - RS_CORE_AUTH_URL=${RS_CORE_AUTH_URL}
      
      # Secrets injected by orchestrator
      # (K8s secrets, Docker secrets, or Vault agent)
    volumes:
      - /run/secrets:/secrets:ro  # Read secrets from volume
```

---

## Code Implementation

### Startup Validation (Rust)
```rust
// products/core-auth/backend-api/src/config.rs

use std::env;

#[derive(Debug)]
pub struct Config {
    // Public
    pub app_name: String,
    pub log_level: String,
    
    // Internal
    pub rs_core_auth_url: String,
    pub redis_url: String,
    
    // Secret
    jwt_secret: String,  // Private field
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        // Public vars (optional)
        let app_name = env::var("APP_NAME")
            .unwrap_or_else(|_| "core-auth-backend-api".to_string());
        
        let log_level = env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "info".to_string());
        
        // Internal vars (required)
        let rs_core_auth_url = env::var("RS_CORE_AUTH_URL")
            .map_err(|_| ConfigError::Missing("RS_CORE_AUTH_URL"))?;
        
        let redis_url = env::var("REDIS_URL")
            .map_err(|_| ConfigError::Missing("REDIS_URL"))?;
        
        // Secret vars (required, validate length)
        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| ConfigError::Missing("JWT_SECRET"))?;
        
        if jwt_secret.len() < 32 {
            return Err(ConfigError::Invalid("JWT_SECRET must be at least 32 characters"));
        }
        
        Ok(Config {
            app_name,
            log_level,
            rs_core_auth_url,
            redis_url,
            jwt_secret,
        })
    }
    
    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }
}

// Never implement Display/Debug for Config (prevents logging secrets)
```

### Secret Protection
```rust
// Prevent secrets in logs
impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("app_name", &self.app_name)
            .field("log_level", &self.log_level)
            .field("rs_core_auth_url", &self.rs_core_auth_url)
            .field("redis_url", &"[REDACTED]")  // Partially redacted
            .field("jwt_secret", &"[REDACTED]")  // Fully redacted
            .finish()
    }
}
```

---

## Secret Rotation

### Rotation Schedule

| Secret Type | Rotation Frequency | Method |
|-------------|-------------------|--------|
| JWT secrets | 90 days | Vault + app restart |
| Database passwords | 90 days | Vault dynamic secrets |
| API keys (3rd party) | 180 days | Manual + Vault update |
| TLS certificates | 60 days | Let's Encrypt auto |
| Encryption keys | 365 days | Key rotation with re-encryption |

### Rotation Process
```bash
#!/bin/bash
# scripts/rotate-secrets.sh

set -e

ENVIRONMENT=$1
SERVICE=$2

echo "Rotating secrets for $SERVICE in $ENVIRONMENT..."

# 1. Generate new secret
NEW_SECRET=$(openssl rand -base64 32)

# 2. Write to Vault with version
vault kv put secret/${SERVICE}/${ENVIRONMENT} \
  jwt_secret="$NEW_SECRET"

# 3. Restart service (graceful)
docker compose restart ${SERVICE}

# 4. Verify new secret loaded
sleep 5
docker compose logs ${SERVICE} | grep "Config loaded"

echo "✅ Secret rotation complete"
```

---

## Automated Enforcement

### Pre-Commit Hook
```bash
#!/bin/bash
# .git/hooks/pre-commit

set -e

echo "🔍 Checking for secrets in staged files..."

# Patterns that indicate secrets
SECRET_PATTERNS=(
  "password\s*=\s*['\"][^'\"]{8,}"
  "secret\s*=\s*['\"][^'\"]{16,}"
  "api[_-]?key\s*=\s*['\"][^'\"]{20,}"
  "token\s*=\s*['\"][^'\"]{20,}"
  "private[_-]?key"
  "-----BEGIN .* PRIVATE KEY-----"
)

VIOLATIONS=0

for pattern in "${SECRET_PATTERNS[@]}"; do
  if git diff --cached | grep -iE "$pattern"; then
    echo "❌ VIOLATION: Potential secret detected"
    echo "   Pattern: $pattern"
    VIOLATIONS=$((VIOLATIONS + 1))
  fi
done

# Check for .env files being committed
if git diff --cached --name-only | grep -E "\.env$|secrets/"; then
  echo "❌ VIOLATION: .env or secrets/ file in commit"
  VIOLATIONS=$((VIOLATIONS + 1))
fi

if [[ $VIOLATIONS -gt 0 ]]; then
  echo ""
  echo "❌ Commit blocked: Secrets detected in staged files"
  echo "   Remove secrets and use Vault or .env.example instead"
  exit 1
fi

echo "✅ No secrets detected in staged files"
```

### CI/CD Validation
```bash
#!/bin/bash
# core-services/_rules/scripts/validate-env-classification.sh

set -e

echo "🔍 [Rule #07] Validating Environment Variable Classification..."

VIOLATIONS=0

# 1. Check .env files are gitignored
echo "  → Checking .env files are not tracked..."
TRACKED_ENV=$(git ls-files | grep "\.env$" || true)

if [[ -n "$TRACKED_ENV" ]]; then
  echo "❌ VIOLATION: .env files tracked in git"
  echo "$TRACKED_ENV"
  VIOLATIONS=$((VIOLATIONS + 1))
fi

# 2. Verify .env.example exists for services
echo "  → Verifying .env.example presence..."
SERVICES=$(find products -name "Dockerfile" -exec dirname {} \;)

for service_dir in $SERVICES; do
  if [[ ! -f "$service_dir/.env.example" ]]; then
    echo "⚠️  WARNING: Missing .env.example in $service_dir"
  fi
done

# 3. Check for hardcoded secrets in compose files
echo "  → Checking for hardcoded secrets in compose files..."
SECRET_IN_COMPOSE=$(find products infrastructure -name "docker-compose*.yml" \
  -exec grep -iE "password|secret|token|key" {} \; | \
  grep -v "VAULT\|vault:\|<vault" | \
  grep -E "=.{16,}" || true)

if [[ -n "$SECRET_IN_COMPOSE" ]]; then
  echo "⚠️  WARNING: Potential hardcoded secrets in compose files"
  echo "$SECRET_IN_COMPOSE"
fi

# 4. Verify secrets use Vault notation in production
echo "  → Checking production uses Vault references..."
PROD_COMPOSE=$(find products -name "docker-compose.yml" ! -name "*dev*")

for file in $PROD_COMPOSE; do
  if grep -iE "JWT_SECRET=|DATABASE_PASSWORD=|API_KEY=" "$file" 2>/dev/null | grep -v "vault:" | grep -q "="; then
    echo "⚠️  WARNING: Production secrets not using Vault in $file"
  fi
done

if [[ $VIOLATIONS -eq 0 ]]; then
  echo "✅ [Rule #07] Environment Variable Classification validation passed"
  exit 0
else
  echo "❌ [Rule #07] Environment Variable Classification validation FAILED with $VIOLATIONS violation(s)"
  exit 1
fi
```

---

## Migration Guide

### Existing Secrets → Vault

**Step 1: Audit current secrets**
```bash
grep -r "PASSWORD\|SECRET\|KEY" products/*/.*env 2>/dev/null | \
  grep -v ".example" > secrets-audit.txt
```

**Step 2: Install Vault**
```bash
cd infrastructure
docker compose -f docker-compose.vault.yml up -d
vault operator init
vault operator unseal
```

**Step 3: Migrate secrets**
```bash
# For each service
SERVICE=core-auth-backend-api
ENV=prod

# Read current secret
CURRENT_SECRET=$(grep JWT_SECRET products/core-auth/backend-api/.env | cut -d= -f2)

# Write to Vault
vault kv put secret/${SERVICE}/${ENV} \
  jwt_secret="$CURRENT_SECRET"

# Update compose to use Vault
```

**Step 4: Update .env.example**
```bash
# Add Vault placeholder
echo "# JWT_SECRET=<vault:secret/core-auth/prod/jwt_secret>" >> .env.example
```

**Step 5: Remove from .env**
```bash
sed -i '/JWT_SECRET=/d' .env
```

---

## Security Benefits

### Defense in Depth

**1. Separation of Duties**
- Developers: Access to public + internal
- DevOps: Access to secrets
- Security team: Access to critical secrets

**2. Audit Trail**
```bash
# Vault audit log
vault audit enable file file_path=/vault/logs/audit.log

# Query who accessed secrets
vault audit list
```

**3. Automatic Rotation**
```bash
# Vault dynamic secrets (PostgreSQL example)
vault secrets enable database
vault write database/roles/readonly \
  db_name=postgres \
  creation_statements="CREATE ROLE \"{{name}}\"..." \
  default_ttl="1h" \
  max_ttl="24h"
```

**4. Leak Detection**
```bash
# Scan git history for secrets
truffleHog --regex --entropy=False file://$(pwd)
```

---

## Normative Status

- Violations **SHOULD** block deployment (enforced in production)
- All new services **MUST** follow classification system
- Existing services **SHOULD** migrate to Vault within 90 days
- `.env` files **MUST NEVER** be committed
- Production secrets **MUST** use Vault or equivalent

---

## Examples

### ✅ Correct: Classified Variables
```bash
# .env.example (committed)
# === PUBLIC ===
APP_NAME=core-auth
LOG_LEVEL=info

# === INTERNAL (replace with real values) ===
RS_CORE_AUTH_URL=http://rs-core-auth:3000
REDIS_URL=redis://redis:6379

# === SECRETS (use Vault in production) ===
# JWT_SECRET=<vault:secret/core-auth/prod/jwt_secret>
# DATABASE_PASSWORD=<vault:secret/core-auth/prod/db_password>
```

### ❌ Wrong: All Mixed Together
```bash
# .env (committed to git - WRONG!)
APP_NAME=core-auth
JWT_SECRET=super_secret_key_123456789
RS_CORE_AUTH_URL=http://rs-core-auth:3000
DATABASE_PASSWORD=prod_db_password_456
LOG_LEVEL=info
```

---

**Author:** DevOps Working Group  
**Date:** 2025-01-06  
**Version:** 1.0  
**Replaces:** None  
**Related:** Canon Rule #03 (BFF Mandatory Boundary), Canon Rule #05 (Core Services Port Non-Exposure)
