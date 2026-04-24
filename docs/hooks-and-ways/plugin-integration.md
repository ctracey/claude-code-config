# Plugin Integration with the Ways System

How Claude Code's native plugin system can be used to distribute and install ways.

## Background: Claude Code Plugin System

Claude Code (version post-August 2025) includes a native plugin system with marketplace support. A plugin is a self-contained directory that can contain skills, agents, hooks, MCP servers, LSP servers, and other components. Plugins are distributed via marketplaces — catalogs that list plugins and their sources.

### Plugin lifecycle

```
marketplace.json          /plugin install foo@marketplace          ~/.claude/plugins/cache/
(catalog)          →      (copies plugin to cache)           →     marketplace/plugin/version/
```

Marketplace plugins are **copied** to a local cache at `~/.claude/plugins/cache/<marketplace>/<plugin>/<version>/` rather than used in-place. This is a security boundary — plugins cannot reference files outside their own directory.

### Installation scopes

When a plugin is installed, a scope determines where the `enabledPlugins` entry is written. The plugin files themselves always land in the same cache location regardless of scope.

| Scope | `enabledPlugins` written to | Effect |
|---|---|---|
| `user` (default) | `~/.claude/settings.json` | Available in all sessions, all projects |
| `project` | `$PROJECT/.claude/settings.json` | Available when working in this project; shared via git |
| `local` | `$PROJECT/.claude/settings.local.json` | Available in this project; gitignored |

### Session-only loading

A separate mechanism exists outside the marketplace install flow:

```bash
claude --plugin-dir PATH
```

This loads a plugin from `PATH` in-place for the duration of the session only. The plugin is **not** copied to cache. This is intended for development and testing.

---

## The Integration Goal

Ways are contextual guidance files (`way.md`) that the ways system discovers and injects into Claude's context window based on keyword and semantic triggers. Currently, ways must live in one of two locations:

- **Global**: `~/.claude/hooks/ways/{domain}/{wayname}/way.md`
- **Project-local**: `$PROJECT/.claude/ways/{domain}/{wayname}/way.md`

The goal is to allow ways to be distributed as plugins — so a team or individual can install a set of ways via `/plugin install` without replacing or forking the entire `~/.claude` config.

A plugin shipping ways would look like:

```
my-ways-plugin/
  .claude-plugin/
    plugin.json
  ways/
    domain/wayname/way.md
```

No hooks or scripts required in the plugin itself. The existing ways infrastructure handles discovery and injection.

---

## What Needs to Change

The discovery script `hooks/ways/check-prompt.sh` currently scans two locations:

```bash
scan_ways "$PROJECT_DIR/.claude/ways"   # project-local
scan_ways "${HOME}/.claude/hooks/ways"  # global
```

Plugin ways land in `~/.claude/plugins/cache/` — neither of these paths. The hook never finds them.

To support plugin-distributed ways, the hook needs to scan the plugin cache in addition to its current locations.

---

## Scenario Analysis

### Scenario 1: User scope, single version

**Install**: `/plugin install my-ways@marketplace` (default scope)

**Plugin lands at**: `~/.claude/plugins/cache/<marketplace>/my-ways/<version>/ways/`

**Solution**: Add a scan of the plugin cache after the existing scans:

```bash
# Scan installed plugins (user-scope cache)
while IFS= read -r -d '' ways_dir; do
  scan_ways "$ways_dir"
done < <(find "${HOME}/.claude/plugins/cache" -maxdepth 4 -type d -name "ways" -print0 2>/dev/null)
```

The `-maxdepth 4` matches `cache/<marketplace>/<plugin>/<version>/ways` without descending into the way files.

**Status**: Straightforward. One addition to `check-prompt.sh`. The existing idempotent marker system handles deduplication — ways fire once per session regardless of how many matching cache directories are found.

---

### Scenario 2: User scope, multiple versions in cache

**Context**: After a plugin update, the previous version directory is marked as orphaned and retained for 7 days to allow in-flight sessions to continue. During this window, both the old and new version `ways/` directories exist in cache.

**Problem**: The cache scan finds both. Ways from the old version would fire alongside the new version's ways during the orphan grace period.

**Solution**: The scan should resolve to the latest version only. Before scanning, identify the highest semver directory under each `<marketplace>/<plugin>/` path and scan only that one. Orphaned directories should be skipped.

**Open question**: Does Claude Code mark orphaned directories in a discoverable way (e.g., a sentinel file), or does the hook need to parse version strings and sort them? If a sentinel exists, the scan can simply skip marked directories. If not, version sorting is required.

**Status**: Needs investigation before implementing.

---

### Scenario 3: Project scope — does not leak to other scopes

**Install**: `/plugin install my-ways@marketplace --scope project` (in `project-a`)

**Plugin lands at**: `~/.claude/plugins/cache/<marketplace>/my-ways/<version>/ways/` (same cache as user scope)

**Problem**: The scenario 1 cache scan is global — it finds all plugins regardless of which scope enabled them. A plugin installed at project scope for `project-a` would have its ways fire in sessions for `project-b`, or in sessions with no project context at all.

**Solution required**: Scope-aware scanning. The hook would need to:

1. Read `enabledPlugins` from the active project's `.claude/settings.json` and `~/.claude/settings.json`
2. Resolve each enabled plugin entry to its cache path
3. Only scan `ways/` directories from enabled plugins for the current context

This requires the hook to understand the `enabledPlugins` format and maintain a mapping from plugin identity to cache path — currently opaque to the ways system.

**Status**: Deferred. Requires a more significant change to the discovery architecture.

---

### Scenario 4: Project scope — does not leak to other projects

**Install**: `/plugin install my-ways@marketplace --scope project` (in `project-a`)

**Problem**: Same root cause as scenario 3. The cache scan has no concept of which project is active. Ways from `project-a`'s plugin fire when working in `project-b`.

**Solution**: Same scope-aware scanning described in scenario 3. Resolving scenario 3 resolves this one too — they share the same fix.

**Status**: Deferred alongside scenario 3.

---

### Scenario 5: Local scope

**Install**: `/plugin install my-ways@marketplace --scope local`

**Plugin lands at**: same cache location. `enabledPlugins` written to `$PROJECT/.claude/settings.local.json` (gitignored).

**Problem**: Same leakage issue as scenarios 3 and 4. The cache scan does not distinguish local-scoped from user-scoped plugins.

**Solution**: Covered by the scope-aware scanning in scenario 3. Local scope behaves identically to project scope for the purposes of this integration — the only difference is the settings file used, which the scope-aware logic would already read.

**Status**: Deferred alongside scenario 3.

---

### Scenario 6: Plugin explicitly disabled

**Context**: A plugin is installed (present in cache) but disabled via `/plugin disable my-ways@marketplace`.

**Problem**: The cache scan has no awareness of enabled/disabled state. Disabling a plugin via the plugin system would have no effect on way injection — ways would continue firing.

**Solution**: Same scope-aware scanning as scenario 3. An `enabledPlugins` allowlist approach naturally handles this — if a plugin is not in the enabled list, it is not scanned regardless of whether its files are in cache.

**Status**: Deferred alongside scenario 3. Until scope-aware scanning is implemented, disabled plugins will continue to have their ways injected.

---

### Scenario 7: Conflicting way paths between plugins

**Context**: Two installed plugins both ship a way at the same `domain/wayname` path (e.g., both include `softwaredev/delivery/github/way.md`).

**Problem**: At runtime, `find` traversal order is non-deterministic. Whichever plugin's way is found first fires and sets the marker; the second is a no-op. The outcome depends on filesystem ordering, not intent.

**Resolution point**: Install time, not runtime. When a plugin is installed, the install process should validate that none of its `ways/` paths conflict with:

- Ways in any other currently installed plugin
- Ways in the global ways system (`~/.claude/hooks/ways/`)

If a conflict is detected, the install should fail with a clear error identifying the conflicting path and the plugin that owns it. This mirrors how package managers handle file conflicts.

**Convention as mitigation**: Plugin authors should namespace their way paths under a domain that reflects their plugin (e.g., `myplugin/featurename/way.md` rather than reusing existing domain names). This reduces the surface for conflicts but does not eliminate them — install-time validation is still required as a hard guarantee.

**Status**: Install-time validation is outside the scope of changes to this repo (the ways hook system). This is a requirement on the Claude Code plugin installer. Document as a constraint on plugin authoring until the installer enforces it.

---

### Scenario 8: Plugin uninstalled, orphan still in cache

**Context**: A user uninstalls a plugin. The cache directory is marked as orphaned and retained for 7 days before deletion.

**Problem**: During the grace period, the `ways/` directory still exists in cache. The scan finds it and ways continue to fire despite the plugin being uninstalled.

**Solution**: Same orphan-skipping logic as scenario 2. If orphaned directories are marked with a sentinel file, the scan skips them. Ways stop firing immediately on uninstall rather than after the 7-day cleanup.

**Status**: Linked to scenario 2 — resolving the orphan detection question resolves both.

---

### Scenario 9: Session-only (`--plugin-dir PATH`)

**Load**: `claude --plugin-dir ~/my-plugin-dev/`

**Plugin location**: Used in-place at `PATH`. No cache copy made.

**Problem**: The cache scan does not cover this path. Ways from session-only plugins are never discovered.

**Potential solution**: If Claude Code sets a discoverable environment variable (e.g., `CLAUDE_PLUGIN_DIR`) in hook context for `--plugin-dir` sessions, the hook could read it and scan that path. Whether this variable is available is unconfirmed.

**Status**: Out of scope. This is a development/testing flow. Ways authors test by placing files directly in `~/.claude/hooks/ways/` during development, then package for distribution.

---

## Proposed Solution A: Query `claude plugin list` Directly

> **Not yet validated.** Preferred candidate — avoids a parallel index by using Claude Code's own state as the authority.

### Concept

`claude plugin list --json` outputs installed plugins with their version, source marketplace, and enable status. The ways hook calls this command, filters to plugins from a known ways-supporting marketplace, and scans only those plugins' `ways/` directories.

No parallel index. No CLI wrapper tool. Claude Code's own records are the authority.

### Hook integration

```bash
WAYS_MARKETPLACE="my-ways-marketplace"
PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$PWD}"

plugin_ways=$(claude plugin list --json 2>/dev/null | jq -r \
  --arg mp "$WAYS_MARKETPLACE" \
  --arg proj "$PROJECT_DIR" '
  .[]
  | select(.marketplace == $mp)
  | select(.enabled == true)
  | select(
      .scope == "user" or
      (.scope == "project" and .project == $proj) or
      (.scope == "local" and .project == $proj)
    )
  | .cache_path + "/ways"
')

while IFS= read -r ways_dir; do
  [[ -n "$ways_dir" ]] && scan_ways "$ways_dir"
done <<< "$plugin_ways"
```

### Scenarios covered

| Scenario | Handled by |
|---|---|
| 1 — user scope, single version | `select(.scope == "user")` |
| 2 — multiple versions | Claude Code only lists the active version; orphans are not listed |
| 3 — project scope, no cross-scope leak | `select(.scope == "project" and .project == $proj)` |
| 4 — project scope, no cross-project leak | Same filter — `$proj` is the active project path |
| 5 — local scope | `select(.scope == "local" and .project == $proj)` |
| 6 — disabled plugin | `select(.enabled == true)` |
| 8 — uninstalled, orphan in cache | Not listed by `plugin list` — handled automatically |

Scenario 7 (conflicting paths) remains an install-time concern. Scenario 9 (`--plugin-dir`) remains out of scope.

### What needs validation

The exact JSON schema from `claude plugin list --json` is not documented. Before implementing, verify:

| Field | Needed for | Status |
|---|---|---|
| `.marketplace` | Filter to ways-specific plugins | Unconfirmed field name |
| `.enabled` | Skip disabled plugins | Unconfirmed field name |
| `.scope` | Scope-aware filtering | Unconfirmed — may not be present |
| `.project` | Project path for scope matching | Unconfirmed — may not be present |
| `.cache_path` | Locate the `ways/` directory | Unconfirmed field name |

If `scope` and `project` are not in the output, this approach falls back to user-scope-only (scenario 1), with the other scenarios remaining deferred.

**Also verify**: whether `claude plugin list` can be invoked from within a hook process without process-nesting issues or permission errors.

---

## Proposed Solution B: Purpose-Built Local Marketplace with Plugin Index

> **Not yet validated.** More complex than Solution A — carry forward only if `claude plugin list` lacks scope/project metadata.

### Concept

Rather than scanning the raw plugin cache (which has no scope metadata), this approach introduces a purpose-built local marketplace managed by a CLI tool. The CLI is the single source of truth for what is installed, at what scope, and where the files are. The ways hook reads from the CLI's index rather than walking the filesystem.

### Components

**1. CLI tool** — manages plugin installation into the local marketplace

```bash
ways-plugin add my-plugin --source ~/my-plugin-dev --scope project
ways-plugin remove my-plugin
ways-plugin list
```

The CLI wraps the standard Claude Code plugin install flow and additionally maintains two files:

**2. `marketplace.json`** — standard Claude Code marketplace catalog, kept current by the CLI

**3. `plugin-index.json`** — purpose-built metadata map, maintained alongside `marketplace.json`

```json
{
  "plugins": [
    {
      "name": "my-ways",
      "marketplace": "local",
      "version": "1.2.0",
      "scope": "user",
      "project": null,
      "cache_path": "~/.claude/plugins/cache/local/my-ways/1.2.0",
      "ways_path": "~/.claude/plugins/cache/local/my-ways/1.2.0/ways",
      "enabled": true,
      "installed_at": "2026-04-22T10:00:00Z"
    },
    {
      "name": "team-ways",
      "marketplace": "local",
      "version": "0.3.1",
      "scope": "project",
      "project": "/Users/tracer/work/project-a",
      "cache_path": "~/.claude/plugins/cache/local/team-ways/0.3.1",
      "ways_path": "~/.claude/plugins/cache/local/team-ways/0.3.1/ways",
      "enabled": true,
      "installed_at": "2026-04-22T11:00:00Z"
    }
  ]
}
```

**4. Ways hook integration** — `check-prompt.sh` reads `plugin-index.json` and filters by scope and active project:

```bash
PLUGIN_INDEX="${HOME}/.claude/plugins/local/plugin-index.json"
PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$PWD}"

if [[ -f "$PLUGIN_INDEX" ]]; then
  while IFS= read -r ways_path; do
    [[ -n "$ways_path" ]] && scan_ways "$ways_path"
  done < <(jq -r --arg project "$PROJECT_DIR" '
    .plugins[]
    | select(.enabled == true)
    | select(.scope == "user" or (.scope == "project" and .project == $project) or (.scope == "local" and .project == $project))
    | .ways_path
  ' "$PLUGIN_INDEX" 2>/dev/null)
fi
```

This resolves scope leakage (scenarios 3–6), orphan handling (scenarios 2 and 8), and disabled plugins in one place — the index is the authority, not the filesystem.

### MCP Integration

The CLI tool is exposed as an MCP server so Claude can manage the local marketplace without the user dropping to the shell:

```json
{
  "mcpServers": {
    "ways-plugin-manager": {
      "command": "ways-plugin",
      "args": ["--mcp"]
    }
  }
}
```

Exposed tools: `install_plugin`, `remove_plugin`, `enable_plugin`, `disable_plugin`, `list_plugins`. This allows Claude to install, enable, and disable way plugins as part of a conversation without manual CLI interaction.

### What This Approach Does Not Solve

- **Scenario 7 (conflicting way paths)** — install-time conflict detection still needs to be built into the CLI tool
- **Scenario 9 (`--plugin-dir`)** — session-only plugins bypass this system entirely; still out of scope
- **Non-local marketplaces** — plugins installed from remote marketplaces via native Claude Code CLI are not tracked in this index unless re-installed through the CLI tool

### Validation Required

Before committing to this approach:

1. Confirm `jq` is available in hook execution context (it is used elsewhere in this repo — likely safe)
2. Confirm the `plugin-index.json` path survives across sessions and is writable by the CLI tool
3. Test that `CLAUDE_PROJECT_DIR` is reliably set in hook context for project-scoped filtering
4. Confirm the MCP server approach works with `ways-plugin --mcp` (CLI must implement MCP protocol)

---

## Implementation Plan (Scenario 1 Only)

The minimal viable implementation covers user-scope installs, which is the primary distribution path.

**Change**: Add ~6 lines to `hooks/ways/check-prompt.sh` after the existing `scan_ways` calls:

```bash
# Scan ways from installed plugins (user-scope cache)
while IFS= read -r -d '' ways_dir; do
  scan_ways "$ways_dir"
done < <(find "${HOME}/.claude/plugins/cache" -maxdepth 4 -type d -name "ways" -print0 2>/dev/null)
```

**Also applies to**: `hooks/ways/check-file-pre.sh` and `hooks/ways/check-bash-pre.sh` if those hooks also scan for ways (check before implementing).

**Plugin authoring**: A plugin shipping ways needs no hook infrastructure of its own. The `ways/` directory mirrors the global ways structure: `ways/{domain}/{wayname}/way.md`. Frontmatter and matching work identically to global ways. The same `/ways-tests` tooling applies for validation.

**Scope override**: A plugin-distributed way follows the same override rules as project-local ways. If a project-local way has the same domain/name path as a plugin way, the project-local version wins (project-local is scanned first). If a global way has the same path, the plugin way would fire alongside it — both markers are independent since they reference different underlying paths.

> **Note**: The override precedence for plugin ways vs. global ways is unresolved. If a plugin way and a global way share the same `domain/wayname` path, both will fire (separate markers). The semantics of this need to be defined — either by convention (plugin ways should use namespaced domain paths) or by adding explicit precedence logic.

---

## Local Marketplace Setup

For local development and team distribution without a public host:

```bash
# Create marketplace structure
mkdir -p ~/claude-plugins/.claude-plugin
mkdir -p ~/claude-plugins/plugins/my-ways/.claude-plugin
mkdir -p ~/claude-plugins/plugins/my-ways/ways/domain/wayname

# Add marketplace catalog
# ~/claude-plugins/.claude-plugin/marketplace.json

# Register locally
/plugin marketplace add ~/claude-plugins

# Install
/plugin install my-ways@my-marketplace-name
```

Plugin ways appear in `~/.claude/plugins/cache/my-marketplace-name/my-ways/<version>/ways/` and are picked up by the scan on next session start.
