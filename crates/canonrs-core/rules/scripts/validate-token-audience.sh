#!/bin/bash
set -euo pipefail

echo "🔍 Canon Rule #09: Token Audience & Cross-Service Trust"

ERRORS=0

# 1. SERVICE_REGISTRY.md MUST exist
if [ ! -f docs/SERVICE_REGISTRY.md ]; then
  echo "❌ Missing docs/SERVICE_REGISTRY.md"
  ERRORS=$((ERRORS + 1))
fi

# 2. Auth Core MUST emit tokens with aud
if ! find core-services/auth products/ -type f \( -name "*.rs" -o -name "*.ts" -o -name "*.go" \) -exec grep -l "struct Claims" {} \; | xargs grep -q "aud:" 2>/dev/null; then
  echo "❌ Claims struct missing 'aud' field"
  ERRORS=$((ERRORS + 1))
fi

# 3. BFFs MUST validate audience
if ! grep -r "aud.*contains\|InvalidAudience\|expected_audience" core-services products 2>/dev/null | grep -q .; then
  echo "❌ No audience validation detected in BFFs"
  ERRORS=$((ERRORS + 1))
fi

# 4. SERVICE_ID MUST be present in BFFs
if ! grep -r "SERVICE_ID" core-services products 2>/dev/null | grep -q .; then
  echo "❌ SERVICE_ID not used in token validation"
  ERRORS=$((ERRORS + 1))
fi

if [ "$ERRORS" -eq 0 ]; then
  echo "✅ Rule #09 validation passed"
  exit 0
else
  echo "❌ Rule #09 failed with $ERRORS violation(s)"
  exit 1
fi
