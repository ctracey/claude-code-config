---
name: version-bump-workflow
description: Scaffold a GitHub Actions workflow that auto-bumps the version on PR merge. Commits with `feat:` trigger a minor bump; everything else triggers a patch bump. Major version stays manual. Use when the user wants automated versioning on merge to main.
allowed-tools: Bash, Read, Write, Glob
---

# Version Bump Workflow

Scaffold `.github/workflows/version-bump.yml` for the current project.

## Detect Project Type

```bash
ls package.json Cargo.toml pyproject.toml 2>/dev/null
```

Adjust the version bump command based on what's found:
- `package.json` → `npm version <bump> --no-git-tag-version`
- `Cargo.toml` → use `sed` to update the `version` field directly
- `pyproject.toml` → use `sed` to update the `version` field directly

## Detect Default Branch

```bash
git remote show origin | grep "HEAD branch"
```

Use the result (`main`, `master`, etc.) as the trigger branch.

## Scaffold the Workflow

Create `.github/workflows/version-bump.yml`:

```yaml
name: Version Bump

on:
  push:
    branches:
      - <default-branch>

jobs:
  bump-version:
    if: github.actor != 'github-actions[bot]'
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
          token: ${{ secrets.GITHUB_TOKEN }}

      - <setup-step>  # e.g. actions/setup-node@v4 for Node projects

      - name: Determine bump type
        id: bump
        run: |
          LAST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")
          if [ -n "$LAST_TAG" ]; then
            COMMITS=$(git log "${LAST_TAG}..HEAD" --pretty=format:"%s")
          else
            COMMITS=$(git log --pretty=format:"%s")
          fi

          if echo "$COMMITS" | grep -qE "^feat(\(.*\))?:"; then
            echo "bump=minor" >> "$GITHUB_OUTPUT"
          else
            echo "bump=patch" >> "$GITHUB_OUTPUT"
          fi

      - name: Bump version
        run: <version-bump-command>

      - name: Commit and push
        run: |
          NEW_VERSION=<read-new-version>
          git config user.name "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"
          git add <version-file>
          git commit -m "chore: bump version to ${NEW_VERSION}"
          git push
```

Fill in the placeholders based on detected project type before writing the file.

### Node (package.json)

- Setup step: `uses: actions/setup-node@v4` with `node-version-file: .nvmrc` if `.nvmrc` exists, otherwise `node-version: lts/*`
- Version bump: `npm version ${{ steps.bump.outputs.bump }} --no-git-tag-version`
- Read new version: `$(node -p "require('./package.json').version")`
- Version file: `package.json`

### Rust (Cargo.toml)

- Setup step: none needed (ubuntu-latest includes cargo)
- Version bump: parse current version and increment with shell arithmetic, then `sed -i`
- Read new version: parse from `Cargo.toml`
- Version file: `Cargo.toml`

### Python (pyproject.toml)

- Setup step: none needed for version bump only
- Version bump: use `sed` or `python -c` to increment the version field
- Read new version: parse from `pyproject.toml`
- Version file: `pyproject.toml`

## After Creating the File

Tell the user:
- What branch it triggers on
- That `feat:` commits → minor, everything else → patch
- That major bumps stay manual (`git tag -a vX.Y.Z -m "..."`)
- That the workflow won't loop because of the `github.actor` guard
