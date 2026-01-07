#!/bin/bash
set -e

echo "🔍 [Rule #05] Validating Core Services Port Non-Exposure..."

VIOLATIONS=0

# 1. Find all rs-core-* services with 'ports:' key
echo "  → Checking rs-core-* services..."
CORE_WITH_PORTS=$(find infrastructure products -name "docker-compose*.yml" -print0 | \
  xargs -0 grep -l "rs-core-" | \
  xargs grep -A10 "rs-core-.*:" | \
  grep -B5 "ports:" || true)

if [[ -n "$CORE_WITH_PORTS" ]]; then
  echo "❌ VIOLATION: Core services cannot expose host ports"
  echo "$CORE_WITH_PORTS"
  VIOLATIONS=$((VIOLATIONS + 1))
fi

# 2. Check infrastructure services (Redis, PostgreSQL, Kafka)
echo "  → Checking infrastructure services..."
INFRA_SERVICES=("redis" "postgres" "postgresql" "kafka")

for service in "${INFRA_SERVICES[@]}"; do
  INFRA_PORTS=$(find infrastructure -name "docker-compose*.yml" -print0 | \
    xargs -0 grep -A10 "${service}:" 2>/dev/null | \
    grep "ports:" || true)
  
  if [[ -n "$INFRA_PORTS" ]]; then
    echo "❌ VIOLATION: Infrastructure service '$service' cannot expose host ports"
    echo "$INFRA_PORTS"
    VIOLATIONS=$((VIOLATIONS + 1))
  fi
done

# 3. Verify core services use 'expose:' instead
echo "  → Verifying core services use 'expose:' only..."
CORE_SERVICES=$(find infrastructure products -name "docker-compose*.yml" -print0 | \
  xargs -0 grep -l "rs-core-" || true)

for file in $CORE_SERVICES; do
  CORE_NAMES=$(grep "rs-core-" "$file" | grep -oP 'rs-core-[a-z-]+' | sort -u)
  
  for core in $CORE_NAMES; do
    HAS_EXPOSE=$(grep -A10 "${core}:" "$file" | grep "expose:" || true)
    
    if [[ -z "$HAS_EXPOSE" ]]; then
      echo "⚠️  WARNING: Core service '$core' in $file should use 'expose:'"
    fi
  done
done

if [[ $VIOLATIONS -eq 0 ]]; then
  echo "✅ [Rule #05] Core Services Non-Exposure validation passed"
  exit 0
else
  echo "❌ [Rule #05] Core Services Non-Exposure validation FAILED with $VIOLATIONS violation(s)"
  exit 1
fi
