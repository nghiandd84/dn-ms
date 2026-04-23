---
name: agent-tool-policy
description: Use when reviewing or updating agent tool permissions
---

# Agent Tool Policy

## Overview
Tool access control matrix for all agent types. Defines which built-in tools each agent can use.

## When to Use
- Reviewing agent tool permissions
- Updating agent configurations
- Debugging tool access issues
- Onboarding new agent types

## Tool Access Table

| Agent | Allowed Tools | Denied Tools |
|-------|---------------|--------------|
| orchestrator | All built-in except denied | web_fetch, fs_write |
| explorer | fs_read, glob, grep, code, web_fetch, web_search | fs_write, shell, subagent |
| oracle | fs_read, glob, grep, code, web_fetch, web_search, knowledge | fs_write, shell, subagent |
| fixer | fs_read, fs_write, glob, grep, code, shell | subagent, web_fetch, web_search |
| designer | fs_read, glob, grep, code | fs_write, shell, subagent, web_fetch, web_search |
| librarian | fs_read, glob, grep, web_fetch, web_search, knowledge | fs_write, shell, subagent, code |
| historian | fs_read, glob, grep, knowledge | fs_write, shell, subagent, web_fetch, web_search, code |
| observer | fs_read, glob, grep, code | fs_write, shell, subagent, web_fetch, web_search |
| council | fs_read, glob, grep, code, knowledge | fs_write, shell, subagent, web_fetch, web_search |

## Built-in Tools

```
web_fetch, web_search, fs_read, fs_write, shell, subagent, glob, grep, code, knowledge, todo_list
```

## MCP Tools

- **@mcp syntax:** MCP servers must be referenced as `@<serverName>` in both `tools` and `allowedTools` arrays to be active
- Example: `@exa`, `@grep_app`, `@filesystem`, `@fetch`
- Must use `@` prefix in tool configuration arrays

## Librarian Tool Priority

1. @exa MCP (primary)
2. @context7 MCP
3. @grep_app MCP
4. @fetch MCP
5. web_search
6. web_fetch

## Prerequisites

`prereq-check.sh` auto-installs:
- ast-grep
- uv
- rg

## Quick Reference

| Task | Action |
|------|--------|
| Add tool to agent | Add to `tools` array |
| Remove tool from agent | Add to `deniedTools` array |
| Check MCP availability | Review `mcpServers` config |
| Verify prerequisites | Run `prereq-check.sh` |

## Common Mistakes

- Listing MCP tools in `tools`/`deniedTools` arrays (they're auto-available)
- Forgetting `todo_list` is available to all agents
- Denying `fs_read` breaks most agent functionality
