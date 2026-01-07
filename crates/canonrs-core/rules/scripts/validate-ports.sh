#!/bin/bash
set -e

echo "🔍 [Rule #04] Validating Port Allocation Strategy..."

VIOLATIONS=0

# 1. Extract all port mappings
echo "  → Extracting port mappings..."
ALL_PORTS=$(find products infrastructure -name "docker-compose*.yml" -exec grep -h "ports:" {} \; | \
  grep -oP '"\K[0-9]+(?=:)' | sort -n || true)

# 2. Check for duplicates
echo "  → Checking for duplicate ports..."
DUPLICATES=$(echo "$ALL_PORTS" | uniq -d)

if [[ -n "$DUPLICATES" ]]; then
  echo "❌ VIOLATION: Duplicate host ports detected"
  echo "$DUPLICATES"
  VIOLATIONS=$((VIOLATIONS + 1))
fi

# 3. Validate frontend uses 8000-8199
echo "  → Validating frontend port range (8000-8199)..."
FRONTEND_PORTS=$(find products -name "docker-compose*.yml" -exec grep -A5 "frontend.*:" {} \; | \
  grep "ports:" | grep -oP '"\K[0-9]+' || true)

for port in $FRONTEND_PORTS; do
  if [[ $port -lt 8000 || $port -gt 8199 ]]; then
    echo "❌ VIOLATION: Frontend port $port outside 8000-8199 range"
    VIOLATIONS=$((VIOLATIONS + 1))
  fi
done

# 4. Validate BFF uses 3100-3199
echo "  → Validating BFF port range (3100-3199)..."
BFF_PORTS=$(find products -name "docker-compose*.yml" -exec grep -A5 "backend-api.*:" {} \; | \
  grep "ports:" | grep -oP '"\K[0-9]+' || true)

for port in $BFF_PORTS; do
  if [[ $port -lt 3100 || $port -gt 3199 ]]; then
    echo "❌ VIOLATION: BFF port $port outside 3100-3199 range"
    VIOLATIONS=$((VIOLATIONS + 1))
  fi
done

# 5. Check PORT_MAP.md exists and is recent
echo "  → Checking PORT_MAP.md status..."
if [[ ! -f "docs/PORT_MAP.md" ]]; then
  echo "⚠️  WARNING: PORT_MAP.md not found"
else
  LAST_UPDATE=$(grep "Last Updated:" docs/PORT_MAP.md | grep -oP '\d{4}-\d{2}-\d{2}' || echo "unknown")
  echo "  ℹ️  PORT_MAP.md last updated: $LAST_UPDATE"
fi

if [[ $VIOLATIONS -eq 0 ]]; then
  echo "✅ [Rule #04] Port Allocation validation passed"
  exit 0
else
  echo "❌ [Rule #04] Port Allocation validation FAILED with $VIOLATIONS violation(s)"
  exit 1
fi
