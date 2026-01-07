#!/bin/bash
set -e

echo "🔍 [Rule #03] Validating BFF Mandatory Boundary..."

VIOLATIONS=0

# 1. Check frontend NEVER joins sha-net
echo "  → Checking frontend network isolation..."
FRONTEND_SHA_NET=$(find products -name "docker-compose*.yml" -exec grep -l "frontend" {} \; | \
  xargs grep -A10 "frontend.*:" 2>/dev/null | grep "sha-net" || true)

if [[ -n "$FRONTEND_SHA_NET" ]]; then
  echo "❌ VIOLATION: Frontend service cannot join sha-net"
  echo "$FRONTEND_SHA_NET"
  VIOLATIONS=$((VIOLATIONS + 1))
fi

# 2. Check frontend code for RS_CORE_* env vars
echo "  → Checking frontend for core service credentials..."
FRONTEND_CORE_REFS=$(find products/*/frontend*/src -type f \( -name "*.rs" -o -name "*.ts" -o -name "*.tsx" -o -name "*.js" -o -name "*.jsx" \) \
  -exec grep -l "RS_CORE_" {} \; 2>/dev/null || true)

if [[ -n "$FRONTEND_CORE_REFS" ]]; then
  echo "❌ VIOLATION: Frontend contains core service references"
  echo "$FRONTEND_CORE_REFS"
  VIOLATIONS=$((VIOLATIONS + 1))
fi

# 3. Check BFF is gateway (has both networks)
echo "  → Verifying BFF has gateway access..."
BFF_SERVICES=$(find products -name "docker-compose.yml" -exec grep -l "backend-api" {} \;)

for compose_file in $BFF_SERVICES; do
  HAS_SHA_NET=$(grep -A20 "backend-api:" "$compose_file" | grep "sha-net" || true)
  HAS_PRODUCT_NET=$(grep -A20 "backend-api:" "$compose_file" | grep -E "product-internal|.*-internal" || true)
  
  if [[ -z "$HAS_SHA_NET" ]]; then
    echo "⚠️  WARNING: BFF missing sha-net access in $compose_file"
  fi
  
  if [[ -z "$HAS_PRODUCT_NET" ]]; then
    echo "⚠️  WARNING: BFF missing product network in $compose_file"
  fi
done

if [[ $VIOLATIONS -eq 0 ]]; then
  echo "✅ [Rule #03] BFF Boundary validation passed"
  exit 0
else
  echo "❌ [Rule #03] BFF Boundary validation FAILED with $VIOLATIONS violation(s)"
  exit 1
fi
