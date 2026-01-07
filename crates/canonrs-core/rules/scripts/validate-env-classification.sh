#!/bin/bash
set -euo pipefail

echo "🔍 Canon Rule #07: Environment Variable Classification"

ERRORS=0

if ! find . -name ".env.example" | grep -q .; then
  echo "⚠️  No .env.example files found"
  ERRORS=$((ERRORS + 1))
fi

if find . -name ".env" -not -path "*/node_modules/*" -exec grep -l "SECRET\|PASSWORD\|KEY" {} \; 2>/dev/null | grep -q .; then
  echo "❌ Secrets found in .env files"
  ERRORS=$((ERRORS + 1))
fi

if [ "$ERRORS" -eq 0 ]; then
  echo "✅ Rule #07 validation passed"
  exit 0
else
  echo "❌ Rule #07 failed with $ERRORS violation(s)"
  exit 1
fi
