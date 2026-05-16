# Post-Push Knowledge Generation Workflow

## Overview

After every push to the `master` branch, Kiro automatically prompts the user to generate or update knowledge documentation in `.kiro/docs`.

## Trigger

A successful `git push` targeting the `master` (or `main`) branch.

## Workflow

1. Kiro identifies which files/features were changed in the pushed commits
2. Kiro asks the user which docs to generate/update:
   - **API docs** → `.kiro/docs/api/<service>.md`
   - **Dev guides** → `.kiro/docs/dev/<topic>.md`
   - **Architecture docs** → `.kiro/docs/architecture/`
   - **Skip** — no docs needed
3. If the user agrees, Kiro analyzes the changed code and generates/updates the relevant markdown files
4. After generating docs, Kiro offers to commit the doc changes separately

## What Gets Documented

| Change Type | Doc Location |
|---|---|
| New/updated API endpoints | `.kiro/docs/api/<service>.md` |
| New patterns or flows | `.kiro/docs/dev/<topic>.md` |
| Schema/migration changes | `.kiro/docs/architecture/schemas/<feature>.md` |
| Service boundary changes | `.kiro/docs/architecture/service-boundaries/<feature>.md` |

## Configuration

This rule is defined in `.kiro/skills/develop/git-commit/SKILL.md` under the "Post-Push to Master: Knowledge Generation" section.

## Purpose

Keeps `.kiro/docs` knowledge bases up-to-date with the latest code changes, ensuring Kiro always has accurate project context for future assistance.
