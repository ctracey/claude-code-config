---
files: way\.md$|SKILL\.md$|\.claude/hooks/[^/]+\.sh$
scope: agent, subagent
---
# Before Creating a New Way, Skill, or Hook

Check whether something already exists that covers this need before writing new content.

## Steps

1. **Ways** — scan `~/.claude/hooks/ways/` and `$PROJECT/.claude/ways/` for existing ways with overlapping vocabulary or file triggers
2. **Skills** — scan `~/.claude/skills/` and `.claude/skills/` for existing skills with similar descriptions
3. **Hooks** — check `~/.claude/hooks/` and `settings.json` for existing hook scripts that already intercept the same event

## Decision

| Finding | Action |
|---------|--------|
| Existing coverage, partial gap | Extend the existing file |
| Overlapping but different concern | Create new, ensure vocabulary doesn't collide |
| Nothing relevant | Proceed with new |

Present the finding to the user before writing anything.
