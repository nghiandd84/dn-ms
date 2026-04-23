---
name: context7-docs
description: Use when user asks about libraries, crates, API references, setup/configuration, or needs code involving third-party packages. Trigger on crate names (tokio, axum, sea-orm, serde, sqlx, tower, etc.) or phrases like "how to use", "API docs", "latest syntax".
---

# Context7 Documentation Lookup

Fetch current library documentation via Context7 instead of relying on training data.

## Steps

### Step 1: Resolve Library ID

Call `resolve-library-id` with:
- `libraryName`: library name from the user's question
- `query`: user's full question (improves relevance)

### Step 2: Select Best Match

- Prefer exact name match
- Higher benchmark score = better docs quality
- If user mentioned a version (e.g. "Angular 19"), prefer version-specific ID

### Step 3: Fetch Docs

Call `query-docs` with:
- `libraryId`: resolved Context7 ID (e.g. `/angular/angular`)
- `query`: user's specific question

### Step 4: Answer

Use fetched docs to answer — include code examples, cite version when relevant.

## Rules

- Always resolve ID first — never query with raw library name
- Pass full user question as `query` for better relevance
- Prefer official packages over community forks when multiple matches exist
