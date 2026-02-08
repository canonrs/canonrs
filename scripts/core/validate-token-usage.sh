#!/bin/bash
set -e

echo "🔍 Validating token usage and API layers..."

TOKENS_FILE="canonrs-ui/styles/.generated/allowed-tokens.txt"
PUBLIC_API_FILE="canonrs-ui/styles/.generated/public-api-tokens.txt"

if [ ! -f "$TOKENS_FILE" ] || [ ! -f "$PUBLIC_API_FILE" ]; then
  echo "❌ Token files not generated. Run cargo build first."
  exit 1
fi

# 1. Verificar tokens inválidos
USED_TOKENS=$(grep -rh "var(--" canonrs-ui/styles --include="*.css" \
  | grep -v "allowed-tokens.txt\|public-api-tokens.txt" \
  | grep -o "var(--[a-z0-9-]*)" \
  | sed 's/var(//' | sed 's/)//' \
  | sort -u)

INVALID=""
while IFS= read -r token; do
  if ! grep -q "^${token}$" "$TOKENS_FILE"; then
    INVALID="${INVALID}${token}\n"
  fi
done <<< "$USED_TOKENS"

if [ -n "$INVALID" ]; then
  echo "❌ INVALID TOKENS USED:"
  echo -e "$INVALID"
  exit 1
fi

# 2. Verificar UI/Blocks usam apenas PUBLIC API
UI_TOKENS=$(grep -rh "var(--" canonrs-ui/styles/ui canonrs-ui/styles/blocks --include="*.css" \
  | grep -o "var(--[a-z0-9-]*)" \
  | sed 's/var(//' | sed 's/)//' \
  | sort -u)

FORBIDDEN=""
while IFS= read -r token; do
  if ! grep -q "^${token}$" "$PUBLIC_API_FILE"; then
    if [[ ! "$token" =~ ^(card|button|input|alert)-.*$ ]]; then
      FORBIDDEN="${FORBIDDEN}${token}\n"
    fi
  fi
done <<< "$UI_TOKENS"

if [ -n "$FORBIDDEN" ]; then
  echo "❌ UI/BLOCKS USING INTERNAL TOKENS:"
  echo -e "$FORBIDDEN"
  exit 1
fi

# 3. Bloquear literais
if grep -RhE ":\s*[0-9]+(px|rem|%|ms|deg|[0-9]);" canonrs-ui/styles/.generated 2>/dev/null; then
  echo "❌ LITERAL VALUES FOUND"
  exit 1
fi

echo "✅ All tokens valid"
echo "✅ UI layer respects PUBLIC API"
echo "✅ No literals in generated CSS"
