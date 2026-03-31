#!/bin/bash
# State-based trigger evaluator — thin dispatcher to ways binary
#
# Evaluates: context-threshold, file-exists, session-start triggers.
# Also handles core guidance re-injection safety net.

INPUT=$(cat)
SESSION_ID=$(echo "$INPUT" | jq -r '.session_id // empty')
TRANSCRIPT=$(echo "$INPUT" | jq -r '.transcript_path // empty')
PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(echo "$INPUT" | jq -r '.cwd // empty')}"

export CLAUDE_PROJECT_DIR="${PROJECT_DIR}"

ARGS=(--session "$SESSION_ID" --project "$PROJECT_DIR")
[[ -n "$TRANSCRIPT" ]] && ARGS+=(--transcript "$TRANSCRIPT")

"${HOME}/.claude/bin/ways" scan state "${ARGS[@]}"
