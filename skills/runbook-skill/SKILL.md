---
name: runbook-skill
description: Use before coding, debugging, build, test, deployment, infrastructure, database, repository maintenance, or agent handoff tasks to run `runbook scan`, understand the current project and machine tool environment, choose the right CLI tools, avoid package-manager/build-tool confusion, and respect risk guardrails before mutating files or remote systems. Also use when the user asks what tools are available, which tool should be used, or how an agent should prepare before starting work.
---

# Runbook Skill

Run `runbook --version` and `runbook scan` before non-trivial repository work, then turn the result into a task-local operating contract. The CLI owns environment discovery; this skill owns agent behavior.

## Workflow

1. Start in the repository or task directory.
2. Check the installed CLI version:

```bash
runbook --version
```

3. Run:

```bash
runbook scan
```

4. Interpret the output:
   - `Local Requirements`: project-implied tools and workflows.
   - `Global Tools`: commands available on this machine.
   - `Recommended Operating Guardrails`: constraints to follow during the task.
   - `Warnings`: missing tools or risky inconsistencies.
5. Prefer local requirements over globally available alternatives.
6. Do not mix package managers, build systems, test runners, deployment tools, or infrastructure tools unless the user explicitly asks.
7. Treat high-risk categories as confirmation-gated before mutation: cloud, infra, database, secrets, security scanners that may expose secrets, deployment, remote write, and destructive file operations.
8. Continue with the user's task using the derived contract.

## Operating Contract

After scanning, internally derive:

- project type and runtime
- package manager
- build command
- test command
- search/read/edit tools
- deployment or infrastructure tools
- risky tools that require confirmation
- missing tools that affect the request

Mention the contract to the user only when it changes the plan, explains a tool choice, blocks progress, or prevents a risky action. Otherwise, let it guide behavior silently.

## Tool Choice Rules

- Use `rg` for repository text search when available.
- Use the project's detected package manager; do not create a second lockfile.
- Use project-local build/test scripts before inventing raw commands.
- Use parser-aware or project-native tools for structured files when available.
- Use simple replacement tools such as `sd` only for mechanical text changes.
- Use `gh`, cloud CLIs, database CLIs, secrets tools, or deployment CLIs in read-only mode first; ask before remote writes or destructive actions.
- On Windows, respect repository instructions for shell choice. If `brush` is available and requested, use it for Unix-style commands, but use PowerShell for Windows shim commands that fail under `brush`.

## Failure Handling

If `runbook` is unavailable:

- Do not invent scan results.
- Say the preflight CLI is missing if the missing tool affects the task.
- Continue with normal repository inspection only when the task can still be completed safely.
- Install the CLI separately with Cargo when needed: `cargo install --git https://github.com/Ariestar/agent-runbook.git`.
