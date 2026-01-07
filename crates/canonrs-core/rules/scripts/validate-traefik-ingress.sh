#!/bin/bash
set -e

echo "🔍 [Rule #06] Validating Traefik Single Public Ingress..."

VIOLATIONS=0

# 1. Check only Traefik uses port 80
echo "  → Checking port 80 exclusivity..."
PORT_80_SERVICES=$(find infrastructure products -name "docker-compose*.yml" -exec grep -l "\"80:" {} \; 2>/dev/null || true)
PORT_80_COUNT=$(echo "$PORT_80_SERVICES" | grep -v "^$" | wc -l)

if [[ $PORT_80_COUNT -gt 1 ]]; then
  echo "❌ VIOLATION: Multiple services expose port 80"
  echo "$PORT_80_SERVICES"
  VIOLATIONS=$((VIOLATIONS + 1))
elif [[ $PORT_80_COUNT -eq 1 ]]; then
  SERVICE_WITH_80=$(echo "$PORT_80_SERVICES" | head -1)
  if [[ ! "$SERVICE_WITH_80" =~ traefik ]]; then
    echo "❌ VIOLATION: Port 80 exposed by non-Traefik service: $SERVICE_WITH_80"
    VIOLATIONS=$((VIOLATIONS + 1))
  fi
fi

# 2. Check only Traefik uses port 443
echo "  → Checking port 443 exclusivity..."
PORT_443_SERVICES=$(find infrastructure products -name "docker-compose*.yml" -exec grep -l "\"443:" {} \; 2>/dev/null || true)
PORT_443_COUNT=$(echo "$PORT_443_SERVICES" | grep -v "^$" | wc -l)

if [[ $PORT_443_COUNT -gt 1 ]]; then
  echo "❌ VIOLATION: Multiple services expose port 443"
  echo "$PORT_443_SERVICES"
  VIOLATIONS=$((VIOLATIONS + 1))
elif [[ $PORT_443_COUNT -eq 1 ]]; then
  SERVICE_WITH_443=$(echo "$PORT_443_SERVICES" | head -1)
  if [[ ! "$SERVICE_WITH_443" =~ traefik ]]; then
    echo "❌ VIOLATION: Port 443 exposed by non-Traefik service: $SERVICE_WITH_443"
    VIOLATIONS=$((VIOLATIONS + 1))
  fi
fi

# 3. Verify Traefik exists in compose files
echo "  → Verifying Traefik configuration exists..."
TRAEFIK_COMPOSE=$(find infrastructure -name "*traefik*.yml" 2>/dev/null | head -1)
if [[ -z "$TRAEFIK_COMPOSE" ]]; then
  TRAEFIK_COMPOSE=$(find infrastructure -name "docker-compose*.yml" -exec grep -l "traefik:" {} \; 2>/dev/null | head -1)
fi

if [[ -z "$TRAEFIK_COMPOSE" ]]; then
  echo "⚠️  WARNING: No Traefik configuration found in infrastructure/"
else
  echo "  ℹ️  Traefik configured in: $TRAEFIK_COMPOSE"
fi

# 4. Check if Traefik is running (optional, may not be in CI)
if command -v docker &> /dev/null; then
  if docker ps 2>/dev/null | grep -q traefik; then
    echo "  ✓ Traefik container is running"
  else
    echo "  ℹ️  Traefik container not running (OK in CI/dev)"
  fi
fi

# 5. Check public services use Traefik labels
echo "  → Checking public services for Traefik labels..."
PUBLIC_SERVICES=$(find products -name "docker-compose.yml" -exec grep -l "frontend" {} \; 2>/dev/null || true)

if [[ -n "$PUBLIC_SERVICES" ]]; then
  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    
    SERVICE_NAME=$(basename $(dirname "$file"))
    
    # Count traefik labels (returns number, not string)
    TRAEFIK_COUNT=$(grep "traefik.enable" "$file" 2>/dev/null | wc -l)
    HAS_PUBLIC_PORTS=$(grep -E "\"80:|\"443:|\"8000:|\"8100:" "$file" 2>/dev/null || true)
    
    if [[ -n "$HAS_PUBLIC_PORTS" ]]; then
      # Check if it's in docker-compose.dev.yml (allowed)
      if [[ "$file" =~ \.dev\.yml$ ]]; then
        echo "  ℹ️  Dev ports OK in $file"
      else
        echo "⚠️  WARNING: Public service in $file may expose ports directly"
        echo "    Found: $(echo "$HAS_PUBLIC_PORTS" | head -1)"
      fi
    fi
    
    # Compare integer, not string with spaces
    if [[ $TRAEFIK_COUNT -eq 0 ]] && [[ ! "$file" =~ \.dev\.yml$ ]]; then
      echo "  ℹ️  INFO: $file has no Traefik configuration"
    fi
  done <<< "$PUBLIC_SERVICES"
fi

# 6. Verify web services don't expose 80/443 in production compose
echo "  → Checking production composes for direct web exposure..."
PROD_COMPOSES=$(find products -name "docker-compose.yml" ! -name "*dev*" ! -name "*test*" 2>/dev/null || true)

if [[ -n "$PROD_COMPOSES" ]]; then
  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    
    WEB_PORTS=$(grep -E "\"80:|\"443:" "$file" 2>/dev/null || true)
    
    if [[ -n "$WEB_PORTS" ]]; then
      echo "❌ VIOLATION: Production compose exposes web ports directly: $file"
      echo "$WEB_PORTS"
      VIOLATIONS=$((VIOLATIONS + 1))
    fi
  done <<< "$PROD_COMPOSES"
fi

if [[ $VIOLATIONS -eq 0 ]]; then
  echo "✅ [Rule #06] Traefik Single Public Ingress validation passed"
  exit 0
else
  echo "❌ [Rule #06] Traefik Single Public Ingress validation FAILED with $VIOLATIONS violation(s)"
  exit 1
fi
