#!/bin/bash
# Check file line counts for staged files
# Blocks commit if any .rs, .ts, .tsx, or .css file exceeds 500 lines

MAX_LINES=500

# Exempt files (pre-existing violations - do not block on these)
EXEMPT_FILES="src-tauri/src/services/server_state.rs src-tauri/src/commands.rs"

# Get list of staged files from arguments
STAGED_FILES="$@"

if [ -z "$STAGED_FILES" ]; then
    exit 0
fi

VIOLATIONS=""

for file in $STAGED_FILES; do
    # Only check target file types
    if [[ "$file" =~ \.(rs|ts|tsx|css)$ ]]; then
        # Check if file is exempt
        IS_EXEMPT=0
        for exempt in $EXEMPT_FILES; do
            if [[ "$file" == "$exempt" ]]; then
                IS_EXEMPT=1
                break
            fi
        done
        if [ "$IS_EXEMPT" -eq 1 ]; then
            continue
        fi

        if [ -f "$file" ]; then
            LINE_COUNT=$(wc -l < "$file")
            if [ "$LINE_COUNT" -gt "$MAX_LINES" ]; then
                VIOLATIONS="${VIOLATIONS}${file}:${LINE_COUNT}\n"
            fi
        fi
    fi
done

if [ -n "$VIOLATIONS" ]; then
    echo "❌ File line count violation (max: ${MAX_LINES} lines):"
    echo -e "$VIOLATIONS"
    exit 1
fi

exit 0
