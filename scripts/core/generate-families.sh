#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/../.."

echo "🔐 Generating CanonRS Families"

cd canonrs-tools/family-engine
cargo run --quiet

cd ../..

test -f canonrs-ui/styles/.generated/family-f-data.css || {
  echo "❌ Family CSS not generated"
  exit 1
}

echo "✅ All families generated"
