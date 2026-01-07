#!/bin/bash
set -euo pipefail

echo "🔍 Canon Rule #10: mTLS Core-to-Core"

ERRORS=0

# 1. Verificar declaração de certs em docker-compose
if ! grep -r "certs/server\|certs/client" docker-compose*.yml 2>/dev/null | grep -q .; then
  echo "⚠️  No certificate volumes declared in docker-compose"
  ERRORS=$((ERRORS + 1))
fi

# 2. Verificar server mTLS config (Axum/Actix)
if ! grep -r "with_client_auth\|require_client_cert\|ClientCertVerifier" core-services products 2>/dev/null | grep -q .; then
  echo "❌ No mTLS server configuration found"
  ERRORS=$((ERRORS + 1))
fi

# 3. Verificar client cert usage (reqwest)
if ! grep -r "Identity::from_pem\|add_root_certificate\|\.identity(" core-services products 2>/dev/null | grep -q .; then
  echo "❌ No client certificate usage found"
  ERRORS=$((ERRORS + 1))
fi

# 4. Proibir danger_accept_invalid_certs
if grep -r "danger_accept_invalid_certs\|VERIFY.*false" core-services products 2>/dev/null | grep -q .; then
  echo "❌ Found danger_accept_invalid_certs (PROHIBITED)"
  ERRORS=$((ERRORS + 1))
fi

# 5. Verificar documentação de certs
if ! grep -i "certificate\|mtls\|vault.*pki" docs/*.md 2>/dev/null | grep -q .; then
  echo "⚠️  Missing certificate documentation"
  ERRORS=$((ERRORS + 1))
fi

# 6. SERVICE_REGISTRY.md deve existir
if [ ! -f "docs/SERVICE_REGISTRY.md" ]; then
  echo "❌ Missing docs/SERVICE_REGISTRY.md"
  ERRORS=$((ERRORS + 1))
fi

if [ "$ERRORS" -eq 0 ]; then
  echo "✅ Rule #10 validation passed"
  exit 0
else
  echo "❌ Rule #10 failed with $ERRORS violation(s)"
  exit 1
fi
