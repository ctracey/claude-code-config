#!/bin/bash
# Download the pre-built way-embed binary for the current platform
#
# Detects OS/arch, downloads from GitHub Releases, verifies it runs.
# Falls back to build-from-source instructions if no pre-built binary exists.
#
# Usage:
#   download-binary.sh [--release TAG] [output-dir]
#
# The binary is placed at: output-dir/way-embed (default: ~/.claude/bin/)

set -euo pipefail

# Platform detection
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)
PLATFORM="${OS}-${ARCH}"

GH_REPO="aaronsb/claude-code-config"
RELEASE_TAG="${WAY_EMBED_RELEASE:-latest}"
BIN_NAME="way-embed-${PLATFORM}"
OUTPUT_DIR="${HOME}/.claude/bin"

# Parse args
while [[ $# -gt 0 ]]; do
  case "$1" in
    --release)
      RELEASE_TAG="$2"
      shift 2 ;;
    --help|-h)
      echo "Usage: $0 [--release TAG] [output-dir]"
      echo ""
      echo "  --release TAG  GitHub Release tag (default: latest way-embed-* release)"
      echo "  output-dir     Override output directory (default: ~/.claude/bin/)"
      echo ""
      echo "Platform: ${PLATFORM}"
      echo "Available: linux-x86_64, linux-aarch64, darwin-x86_64, darwin-arm64"
      exit 0 ;;
    *)
      OUTPUT_DIR="$1"
      shift ;;
  esac
done

OUTPUT_FILE="${OUTPUT_DIR}/way-embed"
PLATFORM_FILE="${OUTPUT_DIR}/${BIN_NAME}"

# Check if already present and working
if [[ -x "$OUTPUT_FILE" ]] && "$OUTPUT_FILE" --version >/dev/null 2>&1; then
  echo "way-embed already installed and working: $OUTPUT_FILE" >&2
  "$OUTPUT_FILE" --version >&2
  echo "$OUTPUT_FILE"
  exit 0
fi

# Need gh CLI
if ! command -v gh >/dev/null 2>&1; then
  echo "error: gh CLI not found — install it or build from source:" >&2
  echo "  cd ~/.claude/tools/way-embed && make" >&2
  exit 1
fi

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Find the latest way-embed release
if [[ "$RELEASE_TAG" == "latest" ]]; then
  RELEASE_TAG=$(gh release list --repo "$GH_REPO" --limit 20 2>/dev/null \
    | awk '/way-embed-v/{print $1; exit}')
  if [[ -z "$RELEASE_TAG" ]]; then
    echo "No way-embed release found. Build from source:" >&2
    echo "  cd ~/.claude/tools/way-embed && make setup" >&2
    exit 1
  fi
fi

echo "Platform: ${PLATFORM}" >&2
echo "Release:  ${RELEASE_TAG}" >&2

# Check if our platform binary exists in the release
if ! gh release view "$RELEASE_TAG" --repo "$GH_REPO" --json assets --jq '.assets[].name' 2>/dev/null | grep -q "^${BIN_NAME}$"; then
  echo "No pre-built binary for ${PLATFORM} in release ${RELEASE_TAG}." >&2
  echo "Available binaries:" >&2
  gh release view "$RELEASE_TAG" --repo "$GH_REPO" --json assets --jq '.assets[].name' 2>/dev/null | grep "way-embed-" | sed 's/^/  /' >&2
  echo "" >&2
  echo "Build from source instead:" >&2
  echo "  cd ~/.claude/tools/way-embed && make setup" >&2
  exit 1
fi

# Download
echo "Downloading ${BIN_NAME}..." >&2
gh release download "$RELEASE_TAG" \
  --repo "$GH_REPO" \
  --pattern "$BIN_NAME" \
  --dir "$OUTPUT_DIR" \
  --clobber

# Make executable and create symlink
chmod +x "$PLATFORM_FILE"
cp "$PLATFORM_FILE" "$OUTPUT_FILE"
chmod +x "$OUTPUT_FILE"

# Verify it runs
if "$OUTPUT_FILE" --version >/dev/null 2>&1; then
  echo "Installed: $OUTPUT_FILE ($("$OUTPUT_FILE" --version))" >&2
  ls -lh "$OUTPUT_FILE" >&2
else
  echo "WARNING: binary downloaded but won't execute on this platform" >&2
  echo "Build from source instead:" >&2
  echo "  cd ~/.claude/tools/way-embed && make" >&2
  rm -f "$OUTPUT_FILE" "$PLATFORM_FILE"
  exit 1
fi

# Output path for scripts to capture
echo "$OUTPUT_FILE"
