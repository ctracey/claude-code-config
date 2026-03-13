#!/bin/bash
# Token position reader — returns the current token count from the active transcript.
# Source this file; it provides functions for token-gated way re-disclosure (ADR-104).
#
# Re-disclosure uses percentage of context window, not fixed token counts.
# This scales automatically: 25% of 1M = 250K, 25% of 200K = 50K.
#
# Usage:
#   source "${HOME}/.claude/hooks/ways/token-position.sh"
#   get_token_position "$SESSION_ID"
#   # now $TOKEN_POSITION is set
#
#   stamp_way_tokens "$WAY_MARKER_NAME" "$SESSION_ID"
#   # writes current token position to marker
#
#   token_distance_exceeded "$WAY_MARKER_NAME" "$SESSION_ID"
#   # returns 0 if re-disclosure threshold exceeded, 1 otherwise

# Re-disclosure fires when a way has drifted 25% of the context window.
# On 1M Opus: ~250K tokens between disclosures (~3-4 per session max)
# On 200K Sonnet: ~50K tokens between disclosures (~2-3 per session max)
REDISCLOSE_PCT=25

# Detect model and set context window size
_detect_context_window() {
  local session_id="$1"
  local project_dir="${CLAUDE_PROJECT_DIR:-$PWD}"
  local project_slug=$(echo "$project_dir" | sed 's|[/.]|-|g')
  local conv_dir="${HOME}/.claude/projects/${project_slug}"

  local transcript=$(find "$conv_dir" -maxdepth 1 -name "*.jsonl" ! -name "*.tmp" -printf '%T@ %p\n' 2>/dev/null \
    | sort -rn | head -1 | cut -d' ' -f2-)

  if [[ -z "$transcript" || ! -f "$transcript" ]]; then
    CONTEXT_WINDOW=200000  # conservative default
    _TRANSCRIPT=""
    return
  fi

  local model=$(jq -r 'select(.type=="assistant" and .message.model) | .message.model' "$transcript" 2>/dev/null | tail -1)

  case "$model" in
    *opus-4-6*|*opus-4*)  CONTEXT_WINDOW=1000000 ;;
    *sonnet*)             CONTEXT_WINDOW=200000 ;;
    *haiku*)              CONTEXT_WINDOW=200000 ;;
    *)                    CONTEXT_WINDOW=200000 ;;
  esac

  # Calculate re-disclosure threshold from percentage
  REDISCLOSE_TOKENS=$(( CONTEXT_WINDOW * REDISCLOSE_PCT / 100 ))

  # Cache transcript path for token reading
  _TRANSCRIPT="$transcript"
}

# Read current token position from transcript API usage data
get_token_position() {
  local session_id="$1"

  if [[ -z "$_TRANSCRIPT" ]]; then
    _detect_context_window "$session_id"
  fi

  if [[ -z "$_TRANSCRIPT" || ! -f "$_TRANSCRIPT" ]]; then
    TOKEN_POSITION=0
    return
  fi

  TOKEN_POSITION=$(jq -r '
    select(.type=="assistant" and .message.usage.cache_read_input_tokens > 0)
    | .message.usage
    | (.cache_read_input_tokens // 0) + (.cache_creation_input_tokens // 0) + (.input_tokens // 0)
  ' "$_TRANSCRIPT" 2>/dev/null | sort -rn | head -1)

  TOKEN_POSITION="${TOKEN_POSITION:-0}"
}

# Write current token position to way marker
stamp_way_tokens() {
  local way_marker_name="$1"
  local session_id="$2"
  echo "$TOKEN_POSITION" > "/tmp/.claude-way-tokens-${way_marker_name}-${session_id}"
}

# Check if token distance exceeds re-disclosure threshold
# Returns 0 (true) if exceeded, 1 (false) if not
token_distance_exceeded() {
  local way_marker_name="$1"
  local session_id="$2"

  # Ensure window is detected
  if [[ -z "$CONTEXT_WINDOW" ]]; then
    _detect_context_window "$session_id"
  fi

  # Read token position at last disclosure
  local last_tokens=$(cat "/tmp/.claude-way-tokens-${way_marker_name}-${session_id}" 2>/dev/null || echo 0)

  # Get current position
  get_token_position "$session_id"

  local distance=$(( TOKEN_POSITION - last_tokens ))

  if [[ $distance -ge $REDISCLOSE_TOKENS ]]; then
    TOKEN_DISTANCE=$distance
    return 0
  fi

  TOKEN_DISTANCE=$distance
  return 1
}
