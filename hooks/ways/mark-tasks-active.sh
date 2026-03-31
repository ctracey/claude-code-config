#!/bin/bash
# PreToolUse hook for TaskCreate
# Sets marker so context-threshold nag stops repeating
source "$(dirname "$0")/sessions-root.sh"
INPUT=$(cat)
SESSION_ID=$(echo "$INPUT" | jq -r '.session_id // empty' 2>/dev/null)
[[ -n "$SESSION_ID" ]] && mkdir -p "${SESSIONS_ROOT}/${SESSION_ID}" && touch "${SESSIONS_ROOT}/${SESSION_ID}/tasks-active"
