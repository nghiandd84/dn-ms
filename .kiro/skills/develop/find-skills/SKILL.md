---
name: find-skills
description: Use when the user asks "how do I do X", "find a skill for X", "is there a skill that can...", or wants to discover available agent skills in this workspace
---

# Find Skills

This skill helps you discover available skills in the workspace and recommend the right one for a task.

## Skill Locations

| Path | Content |
| ---- | ------- |
| `.kiro/skills/rust/` | Rust idioms, patterns, best practices (~140 skills) |
| `.kiro/skills/develop/` | Development workflow skills (coding, git, cartography, etc.) |

## How to Help Users Find Skills

### Step 1: Search Local Skills

```bash
ls .kiro/skills/rust/ | grep -i <keyword>
ls .kiro/skills/develop/
```

Or read skill frontmatter to match by description:

```bash
grep -rl "<keyword>" .kiro/skills/ --include="*.md" -l
```

### Step 2: Categorize by Domain

Rust skills are prefixed by category:

| Prefix | Domain |
| ------ | ------ |
| `api-` | API design, traits, builders |
| `async-` | Tokio, channels, concurrency |
| `err-` | Error handling |
| `mem-` | Memory optimization |
| `perf-` | Performance |
| `type-` | Type system patterns |
| `own-` | Ownership, borrowing, lifetimes |
| `name-` | Naming conventions |
| `doc-` | Documentation |
| `lint-` | Clippy, lints, formatting |
| `test-` | Testing patterns |
| `proj-` | Project structure |
| `opt-` | Compiler optimizations |
| `anti-` | Anti-patterns to avoid |

### Step 3: Present Options to the User

Example response:

```
Found relevant skills for error handling:
- err-thiserror-lib — Use thiserror for library error types
- err-anyhow-app — Use anyhow for application error handling
- err-context-chain — Add context to errors with .context()

To read one: cat .kiro/skills/rust/err-thiserror-lib.md
```

## When No Skills Are Found

If no relevant skill exists:

1. Acknowledge that no existing skill was found
2. Offer to help with the task directly using general capabilities
3. Suggest creating a new skill in `.kiro/skills/` if the pattern is reusable
