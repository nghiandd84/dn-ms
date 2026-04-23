---
name: git-commit
description: Use when the user asks to commit changes, create a git commit, or mentions "/commit". Supports auto-detecting type/scope, generating conventional commit messages, interactive commits, and intelligent file staging.
---

# Git Commit with Conventional Commits

## Overview

Create standardized, semantic git commits using the Conventional Commits specification. Analyze the actual diff to determine appropriate type, scope, and message.

## Conventional Commit Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

## Commit Types

| Type       | Purpose                        |
| ---------- | ------------------------------ |
| `feat`     | New feature                    |
| `fix`      | Bug fix                        |
| `docs`     | Documentation only             |
| `style`    | Formatting/style (no logic)    |
| `refactor` | Code refactor (no feature/fix) |
| `perf`     | Performance improvement        |
| `test`     | Add/update tests               |
| `build`    | Build system/dependencies      |
| `ci`       | CI/config changes              |
| `chore`    | Maintenance/misc               |
| `revert`   | Revert commit                  |

## Breaking Changes

```
# Exclamation mark after type/scope
feat!: remove deprecated endpoint

# BREAKING CHANGE footer
feat: allow config to extend other configs

BREAKING CHANGE: `extends` key behavior changed
```

## Workflow

### 1. Analyze Diff

```bash
# If files are staged, use staged diff
git diff --staged

# If nothing staged, use working tree diff
git diff

# Also check status
git status --porcelain
```

### 2. Stage Files (if needed)

If nothing is staged or you want to group changes differently:

```bash
# Stage specific files
git add path/to/file1 path/to/file2

# Stage by pattern
git add *.test.*
git add src/components/*

# Interactive staging
git add -p
```

**Never commit secrets** (.env, credentials.json, private keys).

### 3. Generate Commit Message

Analyze the diff to determine:

- **Type**: What kind of change is this?
- **Scope**: What area/module is affected?
- **Description**: One-line summary of what changed (present tense, imperative mood, <72 chars)

### 4. Execute Commit

```bash
# Single line
git commit -m "<type>[scope]: <description>"

# Multi-line with body/footer
git commit -m "$(cat <<'EOF'
<type>[scope]: <description>

<optional body>

<optional footer>
EOF
)"
```

## Best Practices

- One logical change per commit
- Present tense: "add" not "added"
- Imperative mood: "fix bug" not "fixes bug"
- Reference issues: `Closes #123`, `Refs #456`
- Keep description under 72 characters

## Git Safety Protocol

- NEVER update git config
- NEVER run destructive commands (--force, hard reset) without explicit request
- NEVER skip hooks (--no-verify) unless user asks
- NEVER force push to main/master
- If commit fails due to hooks, fix and create NEW commit (don't amend)

## IMPORTANT: Do NOT Auto-Push

**After committing, do NOT automatically push.** Wait for user confirmation before pushing.

- After `git commit`, ask "Ready to push?" or wait for user to say "push"
- Only push when explicitly requested
- This allows reviewing changes before they go to remote

## Pre-Commit Verification

### Test Gate

Before committing, verify code quality:

1. **Run checks**:
   ```bash
   cargo check --workspace
   cargo clippy --workspace -- -D warnings
   ```

2. **Run tests if available**:
   ```bash
   cargo test --workspace
   ```

3. **Handle failures**:
   - If checks/tests fail, show failure output and STOP
   - Do NOT commit broken code
   - User can override with explicit "commit anyway"

### Secret Detection

**Never commit without checking for secrets:**

Scan staged files for:
- `.env` files (any variant)
- `*.pem`, `*.key` files
- `credentials.*`, `secrets.*` files
- Files containing patterns like `API_KEY=`, `SECRET=`, `PASSWORD=`

If secrets detected:
1. Alert user immediately
2. Show which files/lines contain secrets
3. Ask for confirmation before proceeding

## Post-Commit Options

After successful commit, present these options:

```
Committed. What next?
1. Push to remote
2. Push and create Pull Request
3. Keep local (I'll handle it later)
4. Amend/undo this commit
```

Wait for user selection before taking action.

## Red Flags / Never

- **NEVER** commit without checking for secrets (.env, *.pem, credentials.*)
- **NEVER** auto-push after commit
- **NEVER** update git config
- **NEVER** run destructive commands (--force, hard reset) without explicit request
- **NEVER** skip hooks (--no-verify) unless user asks
- **NEVER** force push to main/master
- **NEVER** commit broken tests (unless user explicitly says "commit anyway")
