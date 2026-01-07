#!/bin/bash
set -euo pipefail

echo "🔍 Canon Rule #08: Authentication Core Hardening"

ERRORS=0

AUTH_DIR=$(find core-services -type d -name "*auth*" -o -name "rs-core-auth" | head -1)

if [ -z "$AUTH_DIR" ]; then
  echo "❌ Auth Core service not found"
  exit 1
fi

# 1. Verificar endpoint /auth/verify
if ! grep -r "/auth/verify\|/verify" "$AUTH_DIR/src" 2>/dev/null | grep -q .; then
  echo "❌ Missing /auth/verify endpoint"
  ERRORS=$((ERRORS + 1))
fi

# 2. Proibir lógica de produto em Auth Core
if grep -r "Order\|Invoice\|Product\|Cart" "$AUTH_DIR/src" 2>/dev/null | grep -v "test\|spec\|comment" | grep -q .; then
  echo "❌ Product logic found in Auth Core"
  ERRORS=$((ERRORS + 1))
fi

# 3. Verificar não exposição via Traefik
if grep -r "traefik.*$AUTH_DIR\|Host.*auth" docker-compose*.yml traefik/ 2>/dev/null | grep -q .; then
  echo "❌ Auth Core exposed via Traefik"
  ERRORS=$((ERRORS + 1))
fi

# 4. Verificar secure password hashing
if ! grep -r "bcrypt\|argon2" "$AUTH_DIR/src" 2>/dev/null | grep -q .; then
  echo "❌ No secure password hashing found"
  ERRORS=$((ERRORS + 1))
fi

# 5. Proibir algoritmos inseguros
if grep -r "md5\|sha1" "$AUTH_DIR/src" 2>/dev/null | grep -v "test\|spec\|git" | grep -q .; then
  echo "❌ Insecure hashing algorithm detected"
  ERRORS=$((ERRORS + 1))
fi

if [ "$ERRORS" -eq 0 ]; then
  echo "✅ Rule #08 validation passed"
  exit 0
else
  echo "❌ Rule #08 failed with $ERRORS violation(s)"
  exit 1
fi
