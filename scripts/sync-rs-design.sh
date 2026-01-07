#!/bin/bash
set -euo pipefail

VERSION=${1:-"snapshot"}
SRC="/opt/docker/monorepo/packages-rust/rs-design"
DST="/opt/docker/monorepo/opensource/canonrs/crates/rs-design"

echo "🔄 Syncing rs-design $VERSION..."
rm -rf "$DST"
rsync -a --exclude target --exclude .git "$SRC/" "$DST/"
echo "✅ Done"
