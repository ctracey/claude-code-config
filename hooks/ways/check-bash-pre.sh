#!/bin/bash
# PreToolUse: Check bash commands against ways — thin dispatcher
#
# The ways binary handles: command pattern matching, semantic scoring,
# check curve scoring, session state, and content output.

source "$(dirname "$0")/require-ways.sh"

INPUT=$(cat)
CMD=$(echo "$INPUT" | jq -r '.tool_input.command // empty')
DESC=$(echo "$INPUT" | jq -r '.tool_input.description // empty' | tr '[:upper:]' '[:lower:]')
SESSION_ID=$(echo "$INPUT" | jq -r '.session_id // empty')
PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(echo "$INPUT" | jq -r '.cwd // empty')}"

export CLAUDE_PROJECT_DIR="${PROJECT_DIR}"
"${HOME}/.claude/bin/ways" scan command \
  --command "$CMD" \
  --description "$DESC" \
  --session "$SESSION_ID" \
  --project "$PROJECT_DIR"
