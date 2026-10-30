#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MAX_LINES=350
FAILED=0

echo "🔍 Auditing file line limits in $REPO_ROOT/src (Max: $MAX_LINES lines)..."
while IFS= read -r file; do
    rel_file="${file#"$REPO_ROOT/"}"
    lines=$(wc -l < "$file")
    if [ "$lines" -ge "$MAX_LINES" ]; then
        echo "❌ [VIOLATION] $rel_file has $lines lines (limit: $MAX_LINES)"
        FAILED=1
    else
        printf "  ✓ %-35s %3d lines\n" "$rel_file" "$lines"
    fi
done < <(find "$REPO_ROOT/src" -type f -name "*.rs" | sort)

if [ "$FAILED" -eq 1 ]; then
    echo "❌ Line limit audit failed! Decompose bloated files to adhere to SRP."
    exit 1
fi

echo "✅ All source files adhere to the < $MAX_LINES line limit."
