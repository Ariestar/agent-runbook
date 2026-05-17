---
name: runbook-skill
description: Use before coding, debugging, build, test, deployment, infrastructure, database, repository maintenance, or agent handoff tasks to run `runbook scan`, understand the current project and machine tool environment, choose the right CLI tools, avoid package-manager/build-tool confusion, and respect risk guardrails before mutating files or remote systems. Also use when the user asks what tools are available, which tool should be used, or how an agent should prepare before starting work.
---

# Runbook Skill

Run `runbook --version` and `runbook scan` before non-trivial repository work, then turn the result into a task-local operating contract. When tool choice is not obvious, use `runbook category` to expose the relevant tool candidates instead of guessing from memory.

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

Use compact output only when you need a quick inventory of tool names:

```bash
runbook scan --minimal
```

4. Run this when the task may need a tool beyond the obvious project-local build/test/search commands:

```bash
runbook category
```

If category arguments are unclear, ask the CLI for usage:

```bash
runbook category --help
```

5. Choose one or more functional categories from that output, infer the project language from the scan and repository files, then inspect candidates:

```bash
runbook category <category>... --lang <lang>
```

Use `--lang all` only when the task is language-independent. Cross-language tools appear in language-specific queries automatically.
Tools may belong to multiple categories, so query the category that matches the task. For example, `runbook category test --lang rust` includes Rust tools that support testing even if they also build or manage packages.

6. Interpret the scan output:
   - `Local Requirements`: project-implied tools and workflows.
   - `Global Tools`: commands available on this machine.
   - `Recommended Operating Guardrails`: constraints to follow during the task.
   - `Warnings`: missing tools or risky inconsistencies.
7. Interpret category output as a candidate set, not an instruction. Pick the tool that fits the repo, task, risk, and installed state.
8. Prefer local requirements over globally available alternatives.
9. Do not mix package managers, build systems, test runners, deployment tools, or infrastructure tools unless the user explicitly asks.
10. Treat high-risk categories as confirmation-gated before mutation: cloud, infra, database, secrets, security scanners that may expose secrets, deployment, remote write, and destructive file operations.
11. Continue with the user's task using the derived contract.

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
- category queries that would improve tool choice

Mention the contract to the user only when it changes the plan, explains a tool choice, blocks progress, or prevents a risky action. Otherwise, let it guide behavior silently.

## Category Query Rules

- Use `runbook category` before asking which tool family exists.
- Use `runbook category <category>... --lang <lang>` before using an unfamiliar or non-project-local tool. Query related categories together when the task crosses boundaries, such as `test lint formatter`.
- Start with the closest category instead of guessing a tool's primary category. Multi-category tools are returned by every matching category.
- Do not query every category. Query only the functional category related to the task, such as `search`, `lint`, `test`, `security`, `database`, `deploy`, `container`, `cloud`, `docs`, or `benchmark`.
- If the best candidate is missing, mention it only when it materially affects the task; otherwise choose an installed suitable alternative.
- For remote-write or destructive categories, use category output to identify risk first, then ask for confirmation before mutation.

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
