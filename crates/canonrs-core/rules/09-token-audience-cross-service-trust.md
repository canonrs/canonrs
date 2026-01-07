# Canon Rule #09: Token Audience & Cross-Service Trust

**Status:** Normative  
**Applies to:** All services consuming authentication tokens, multi-product architectures  
**Related:** Canon Rule #08 (Authentication Core Hardening), Canon Rule #03 (BFF Mandatory Boundary)

---

## The Principle

**Tokens MUST specify and verify their intended audience to prevent token misuse across service boundaries.**

Every token consumer MUST validate that it is the intended recipient before accepting the token.

---

## The Problem

### Anti-pattern: Universal Token Acceptance
````rust
// ❌ WRONG: BFF accepts any valid token
async fn authenticate(token: &str) -> Result<User> {
    // Verifies signature but not audience
    let claims = jwt::verify(token, &public_key)?;
    
    // ❌ No check if this BFF is the intended audience
    // ❌ Token from Product A works in Product B
    // ❌ Token from admin API works in user API
    
    Ok(User::from_claims(claims))
}

// Token issued for "billing-bff" works in "core-auth-bff"
// Token issued for "admin-api" works in "user-api"
````

**Issues:**
- ❌ **Privilege Escalation:** Admin tokens work in user endpoints
- ❌ **Cross-Product Leakage:** Token from Product A works in Product B
- ❌ **Unclear Trust:** No explicit service-to-service trust model
- ❌ **Audit Trail:** Can't determine intended vs actual token usage
- ❌ **Blast Radius:** Stolen token has system-wide access

---

## The Solution

### Correct: Audience-Scoped Tokens
````rust
// Token issued by rs-core-auth
{
  "sub": "user_123",
  "exp": 1735689600,
  "iss": "rs-core-auth",
  "aud": ["core-auth-bff"],  // ✅ Explicit audience
  "scope": ["read:profile", "write:profile"]
}

// BFF verifies it's the intended audience
async fn authenticate(token: &str) -> Result<User> {
    let claims = jwt::verify(token, &public_key)?;
    
    // ✅ MUST verify audience
    if !claims.aud.contains("core-auth-bff") {
        return Err(AuthError::InvalidAudience);
    }
    
    // ✅ MUST verify scope
    if !claims.scope.contains(&"read:profile") {
        return Err(AuthError::InsufficientScope);
    }
    
    Ok(User::from_claims(claims))
}
````

---

## Mandatory Requirements

### 1. Token Audience Field

**Every token MUST include `aud` claim:**
````rust
// rs-core-auth token generation
pub async fn issue_token(&self, user_id: Uuid, audience: &str) -> Result<String> {
    let claims = Claims {
        sub: user_id.to_string(),
        exp: now() + Duration::minutes(15),
        iat: now(),
        iss: "rs-core-auth".to_string(),
        aud: vec![audience.to_string()],  // ✅ Explicit audience
        scope: self.get_user_scopes(user_id).await?,
    };
    
    self.jwt_signer.sign(&claims)
}

// BFF requests token with its identity
let token = auth_core.issue_token(user_id, "core-auth-bff").await?;
````

**Requirements:**
- ✅ MUST include `aud` field in all tokens
- ✅ MUST be array of strings (supports multiple audiences)
- ✅ MUST use service DNS name or identifier
- ✅ MUST be validated by recipient

---

### 2. Audience Validation

**Every token consumer MUST verify audience:**
````rust
// products/core-auth/backend-api/src/middleware/auth.rs

pub struct AuthMiddleware {
    public_key: PublicKey,
    expected_audience: String,  // "core-auth-bff"
}

impl AuthMiddleware {
    pub async fn verify_token(&self, token: &str) -> Result<Claims> {
        // 1. Verify signature
        let claims = jwt::decode::<Claims>(
            token,
            &self.public_key,
            &Validation::new(Algorithm::EdDSA)
        )?;
        
        // 2. MANDATORY: Verify audience
        if !claims.aud.contains(&self.expected_audience) {
            error!(
                "Token audience mismatch: expected {}, got {:?}",
                self.expected_audience,
                claims.aud
            );
            return Err(AuthError::InvalidAudience);
        }
        
        // 3. Verify expiration
        if claims.exp < now() {
            return Err(AuthError::TokenExpired);
        }
        
        Ok(claims)
    }
}
````

---

### 3. Service Identity Registry

**Maintain canonical list of service identities:**
````yaml
# docs/SERVICE_REGISTRY.md

## Registered Service Identities

### BFF Services
- `core-auth-bff` - Core authentication BFF
- `billing-bff` - Billing product BFF
- `admin-bff` - Admin panel BFF

### Core Services (Internal Only)
- `rs-core-auth` - Authentication core (never in aud)
- `rs-core-payments` - Payments core (never in aud)
- `rs-core-notifications` - Notifications core (never in aud)

### Workers
- `email-worker` - Email processing worker
- `report-worker` - Report generation worker

## Audience Rules

1. **BFFs receive user tokens:**
   - aud: ["<product>-bff"]
   - User identity in `sub`
   
2. **Service-to-service tokens:**
   - aud: ["<target-service>"]
   - Service identity in `sub`
   
3. **Core services NEVER appear in aud:**
   - Core services don't validate user tokens
   - Only BFFs validate user tokens
````

---

### 4. Multi-Audience Tokens (Limited Use)

**When a token needs multiple audiences:**
````rust
// Use case: Token valid for multiple BFFs in same product
let claims = Claims {
    sub: user_id,
    aud: vec![
        "core-auth-bff".to_string(),
        "core-auth-admin-bff".to_string(),
    ],
    // ...
};

// Each BFF validates its own identity
if !claims.aud.contains("core-auth-bff") {
    return Err(AuthError::InvalidAudience);
}
````

**Requirements:**
- ✅ MUST document why multiple audiences needed
- ✅ MUST be same security domain
- ✅ SHOULD be same product
- ❌ NEVER span unrelated products

---

### 5. Service-to-Service Authentication

**Core services calling other core services:**
````rust
// rs-core-payments calling rs-core-auth for verification

pub struct ServiceAuthClient {
    service_token: String,  // Long-lived service credential
    service_id: String,     // "rs-core-payments"
}

impl ServiceAuthClient {
    pub async fn call_auth_core(&self, endpoint: &str) -> Result<Response> {
        // Service-to-service token
        let s2s_token = self.generate_service_token("rs-core-auth").await?;
        
        let response = reqwest::Client::new()
            .post(format!("http://rs-core-auth:3000{}", endpoint))
            .header("Authorization", format!("Bearer {}", s2s_token))
            .header("X-Service-ID", &self.service_id)
            .send()
            .await?;
        
        Ok(response)
    }
    
    async fn generate_service_token(&self, target: &str) -> Result<String> {
        let claims = Claims {
            sub: self.service_id.clone(),  // Source service
            aud: vec![target.to_string()], // Target service
            exp: now() + Duration::minutes(5),
            iss: "rs-core-auth",
            scope: vec!["service:call".to_string()],
        };
        
        // Sign with service's private key
        jwt::sign(&claims, &self.private_key)
    }
}
````

---

### 6. Scope-Based Authorization

**Fine-grained permissions within audience:**
````rust
// Token with scopes
{
  "sub": "user_123",
  "aud": ["core-auth-bff"],
  "scope": [
    "read:profile",
    "write:profile",
    "read:sessions",
    "delete:sessions"
  ]
}

// Endpoint-level scope validation
#[axum::debug_handler]
async fn update_profile(
    Extension(claims): Extension<Claims>,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<Profile>> {
    // ✅ Verify required scope
    if !claims.scope.contains(&"write:profile") {
        return Err(AuthError::InsufficientScope);
    }
    
    // Process request
    let profile = update_user_profile(&claims.sub, req).await?;
    Ok(Json(profile))
}
````

---

## Cross-Product Token Exchange

### Scenario: User in Product A needs Product B access

**❌ WRONG: Reuse Product A token in Product B**
````rust
// User has token for billing-bff
// Frontend tries to use same token for core-auth-bff
// ❌ This MUST be rejected by audience validation
````

**✅ CORRECT: Token exchange flow**
````rust
// 1. Frontend requests new token for Product B
POST /auth/exchange
{
  "current_token": "...",
  "target_audience": "core-auth-bff"
}

// 2. BFF validates current token and requests new one
pub async fn exchange_token(
    current_token: &str,
    target_audience: &str,
) -> Result<String> {
    // Validate current token
    let claims = self.verify_token(current_token).await?;
    
    // Request new token from auth core
    let new_token = self.auth_core
        .issue_token(claims.sub, target_audience)
        .await?;
    
    // Audit token exchange
    self.audit.log(TokenExchange {
        user_id: claims.sub,
        from_audience: claims.aud,
        to_audience: target_audience,
    }).await?;
    
    Ok(new_token)
}
````

---

## Implementation Patterns

### 1. BFF Token Validation Middleware
````rust
// products/core-auth/backend-api/src/middleware/auth.rs

use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

pub async fn validate_token_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, AuthError> {
    // Extract token from Authorization header
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(AuthError::MissingToken)?;
    
    // Get auth middleware from extensions
    let auth = req
        .extensions()
        .get::<Arc<AuthMiddleware>>()
        .ok_or(AuthError::Internal)?;
    
    // Verify token (includes audience check)
    let claims = auth.verify_token(token).await?;
    
    // Inject claims into request
    req.extensions_mut().insert(claims);
    
    Ok(next.run(req).await)
}
````

### 2. Service Registration on Startup
````rust
// products/core-auth/backend-api/src/main.rs

#[tokio::main]
async fn main() -> Result<()> {
    // Load service identity from environment
    let service_id = env::var("SERVICE_ID")
        .expect("SERVICE_ID must be set (e.g., core-auth-bff)");
    
    // Validate service ID is registered
    if !REGISTERED_SERVICES.contains(&service_id.as_str()) {
        panic!("Service ID '{}' not in SERVICE_REGISTRY", service_id);
    }
    
    // Initialize auth middleware with service identity
    let auth = AuthMiddleware {
        public_key: load_public_key().await?,
        expected_audience: service_id.clone(),
    };
    
    info!("Service '{}' initialized with audience validation", service_id);
    
    // Start server...
}

const REGISTERED_SERVICES: &[&str] = &[
    "core-auth-bff",
    "billing-bff",
    "admin-bff",
];
````

---

## Security Benefits

### 1. Blast Radius Limitation
````
Without audience validation:
├── Token stolen from Product A
└── Works in Products A, B, C, D
    └── Full system compromise

With audience validation:
├── Token stolen from Product A
└── Only works in Product A
    └── Contained breach
````

### 2. Privilege Separation
````
Admin token (aud: ["admin-bff"]):
├── ✅ Can access admin endpoints
└── ❌ Cannot access user endpoints (different audience)

User token (aud: ["user-bff"]):
├── ✅ Can access user endpoints
└── ❌ Cannot access admin endpoints (different audience)
````

### 3. Audit Trail
````json
// Auth event log
{
  "event": "token_issued",
  "user_id": "user_123",
  "audience": "core-auth-bff",
  "timestamp": "2025-01-06T12:00:00Z"
}

{
  "event": "token_rejected",
  "token_audience": "billing-bff",
  "expected_audience": "core-auth-bff",
  "timestamp": "2025-01-06T12:05:00Z"
}
````

---

## Automated Enforcement

### Validation Script
````bash
#!/bin/bash
# core-services/_rules/scripts/validate-token-audience.sh

set -e

echo "🔍 [Rule #09] Validating Token Audience & Cross-Service Trust..."

VIOLATIONS=0

# 1. Check BFFs validate audience
echo "  → Checking BFF token validation..."
BFF_SERVICES=$(find products -name "Cargo.toml" -exec grep -l "backend-api\|bff" {} \; | xargs dirname)

for service_dir in $BFF_SERVICES; do
  # Check for audience validation in code
  if ! grep -r "aud\|audience" "$service_dir/src" | grep -q "verify\|validate\|check"; then
    echo "⚠️  WARNING: $service_dir may not validate token audience"
  fi
  
  # Check SERVICE_ID environment variable
  if ! grep -r "SERVICE_ID" "$service_dir" >/dev/null 2>&1; then
    echo "⚠️  WARNING: $service_dir missing SERVICE_ID configuration"
  fi
done

# 2. Verify SERVICE_REGISTRY.md exists
echo "  → Checking service registry..."
if [[ ! -f "docs/SERVICE_REGISTRY.md" ]]; then
  echo "❌ VIOLATION: Missing docs/SERVICE_REGISTRY.md"
  VIOLATIONS=$((VIOLATIONS + 1))
fi

# 3. Check for hardcoded audiences
echo "  → Checking for hardcoded audience values..."
HARDCODED=$(find products -name "*.rs" -exec grep -l "aud.*=.*\[" {} \; | \
  xargs grep -E "\"[a-z-]+-(bff|api)\"" | \
  grep -v "expected_audience\|SERVICE_ID" || true)

if [[ -n "$HARDCODED" ]]; then
  echo "⚠️  WARNING: Possible hardcoded audience values"
  echo "$HARDCODED"
fi

# 4. Verify tokens include aud in claims struct
echo "  → Checking Claims struct includes audience..."
CLAIMS_STRUCTS=$(find products core-services -name "*.rs" -exec grep -l "struct Claims" {} \;)

for file in $CLAIMS_STRUCTS; do
  if ! grep -A10 "struct Claims" "$file" | grep -q "aud:"; then
    echo "⚠️  WARNING: Claims struct in $file missing aud field"
  fi
done

if [[ $VIOLATIONS -eq 0 ]]; then
  echo "✅ [Rule #09] Token Audience validation passed"
  exit 0
else
  echo "❌ [Rule #09] Token Audience validation FAILED with $VIOLATIONS violation(s)"
  exit 1
fi
