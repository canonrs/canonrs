#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/../.."

echo "📦 Bundling canonrs.css..."

OUTPUT="canonrs-ui/styles/canonrs.bundle.css"
> "$OUTPUT"

process_file() {
    local file="$1"
    local base_dir=$(dirname "$file")

    while IFS= read -r line; do
        if [[ $line =~ @import[[:space:]]+[\"\'](.*)[\"\']\; ]]; then
            local import_path="${BASH_REMATCH[1]}"
            local resolved="$base_dir/$import_path"

            if [[ -f "$resolved" ]]; then
                echo "/* Bundled: $import_path */" >> "$OUTPUT"
                process_file "$resolved"
            else
                echo "⚠️  Missing: $import_path"
            fi
        else
            echo "$line" >> "$OUTPUT"
        fi
    done < "$file"
}

process_file "canonrs-ui/styles/canonrs.css"

echo "✅ Bundle created: $OUTPUT ($(wc -l < "$OUTPUT") lines)"
