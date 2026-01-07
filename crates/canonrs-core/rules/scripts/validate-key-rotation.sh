#!/bin/bash
set -euo pipefail

echo "🔍 Canon Rule #11: Key Rotation & kid Enforcement"

ERRORS=0

# 1. Verificar kid em token generation
if ! grep -r "kid\|key_id" core-services/auth products/core-auth 2>/dev/null | grep -q .; then
  echo "❌ No 'kid' usage found in token generation"
  ERRORS=$((ERRORS + 1))
fi

# 2. Verificar uso de Vault Transit
if ! grep -r "vault.*transit\|VaultTransit" core-services/auth 2>/dev/null | grep -q .; then
  echo "❌ No Vault Transit usage found"
  ERRORS=$((ERRORS + 1))
fi

# 3. Verificar JWKS endpoint
if ! grep -r "jwks\|well-known" core-services/auth 2>/dev/null | grep -q .; then
  echo "⚠️  No JWKS endpoint found"
  ERRORS=$((ERRORS + 1))
fi

# 4. Verificar validação de kid em BFFs
if ! grep -r "kid.*header\|decode_header.*kid" products core-services 2>/dev/null | grep -q .; then
  echo "❌ BFFs not validating kid from token header"
  ERRORS=$((ERRORS + 1))
fi

# 5. Proibir hardcoded private keys
if grep -r "BEGIN.*PRIVATE.*KEY" core-services products 2>/dev/null | grep -v ".md" | grep -q .; then
  echo "❌ Found hardcoded private keys (PROHIBITED)"
  ERRORS=$((ERRORS + 1))
fi

if [ "$ERRORS" -eq 0 ]; then
  echo "✅ Rule #11 validation passed"
  exit 0
else
  echo "❌ Rule #11 failed with $ERRORS violation(s)"
  exit 1
fi
