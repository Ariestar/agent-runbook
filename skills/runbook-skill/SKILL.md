---
name: runbook-skill
description: Use before coding, debugging, build, test, deployment, infrastructure, database, repository maintenance, or agent handoff tasks to run `runbook scan`, understand the current project and machine tool environment, choose the right CLI tools, avoid package-manager/build-tool confusion, and respect risk guardrails before mutating files or remote systems. Also use when the user asks what tools are available, which tool should be used, or how an agent should prepare before starting work.
---

# Runbook Skill

Run `runbook --version`, `runbook scan`, `runbook prefer`, and at least one task-relevant `runbook category ... --lang ...` query before non-trivial repository work. `runbook scan` is only the fact inventory; it is not a complete tool-choice workflow by itself. Turn the scan, preferences, and category candidates into a task-local operating contract before mutating files or running build/test/lint/deploy/database commands.

When tool choice is not obvious, use `runbook category` to expose the relevant tool candidates instead of guessing from memory. When a repository has repeated or user-confirmed tool choices, read or update them through `runbook prefer`; never write preferences silently.

## Completion Bar

For non-trivial repository work, Runbook preflight is complete only after all of these are true:

- the installed CLI version is known
- project and machine facts have been scanned
- repository preferences have been read
- task-relevant category candidates have been inspected
- the final tool choices are explained by local evidence, preferences, or category comparison

Do not say or imply that Runbook has been "run" or that the operating contract is complete after only `runbook scan`, unless the user explicitly asked for a scan-only inventory.

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

4. Read repository preferences immediately after scanning:

```bash
runbook prefer
```

`runbook scan` does not display preferences. If you skip `runbook prefer`, you may miss the repository's durable tool choices.

5. List categories when you need to choose which functional category applies:

```bash
runbook category
```

If category arguments are unclear, ask the CLI for usage:

```bash
runbook category --help
```

6. Choose one or more functional categories from that output, infer the project language from the scan and repository files, then inspect candidates before acting:

```bash
runbook category <category>... --lang <lang>
```

Use `--lang all` only when the task is language-independent. Cross-language tools appear in language-specific queries automatically.
Tools may belong to multiple categories, so query the category that matches the task. For example, `runbook category test --lang rust` includes Rust tools that support testing even if they also build or manage packages.

7. If the user confirms a durable repository preference, record it explicitly:

```bash
runbook prefer set <category> --lang <lang> --tool <tool> --reason "<reason>"
```

Remove a stale preference only after it is no longer valid:

```bash
runbook prefer unset <category> --lang <lang>
```

8. Interpret the scan output:
   - `Local Requirements`: project-implied tools and workflows.
   - `Global Tools`: commands available on this machine.
   - `Recommended Operating Guardrails`: constraints to follow during the task.
   - `Warnings`: missing tools or risky inconsistencies.
9. Interpret preference output as durable repository guidance, not as a replacement for checking installed state and task fit.
10. Interpret category output as a candidate set, not an instruction. Preferred tools are strong repository-local signals, but still check the task, risk, and installed state.
11. Prefer local requirements over globally available alternatives.
12. Do not mix package managers, build systems, test runners, deployment tools, or infrastructure tools unless the user explicitly asks.
13. Treat high-risk categories as confirmation-gated before mutation: cloud, infra, database, secrets, security scanners that may expose secrets, deployment, remote write, and destructive file operations.
14. Continue with the user's task using the derived contract.

## Operating Contract

After the complete preflight, internally derive:

- project type and runtime
- package manager
- build command
- test command
- search/read/edit tools
- deployment or infrastructure tools
- risky tools that require confirmation
- missing tools that affect the request
- category queries that would improve tool choice
- repository preferences that affect the chosen category and language

Mention the contract to the user only when it changes the plan, explains a tool choice, blocks progress, or prevents a risky action. Otherwise, let it guide behavior silently.

If the user asks which tools you will use, answer from the full evidence set: scan results, preferences, and relevant category candidates. Do not answer from scan results alone.

## Category Query Rules

- For non-trivial repository work, run at least one task-relevant `runbook category <category>... --lang <lang>` query before the first mutation or verification command.
- Use `runbook category` before asking which tool family exists.
- Use `runbook category <category>... --lang <lang>` before using an unfamiliar or non-project-local tool. Query related categories together when the task crosses boundaries, such as `test lint formatter`.
- Start with the closest category instead of guessing a tool's primary category. Multi-category tools are returned by every matching category.
- Do not query every category. Query only the functional category related to the task, such as `search`, `lint`, `test`, `security`, `database`, `deploy`, `container`, `cloud`, `docs`, or `benchmark`.
- If the best candidate is missing, mention it only when it materially affects the task; otherwise choose an installed suitable alternative.
- For remote-write or destructive categories, use category output to identify risk first, then ask for confirmation before mutation.

Useful starting queries:

- Rust code work: `runbook category build test lint formatter package-manager --lang rust`
- Python code work: `runbook category package-manager test lint formatter type-checker --lang python`
- JavaScript or TypeScript work: `runbook category package-manager runtime build test lint formatter type-checker --lang typescript`
- Shell or repository maintenance: `runbook category shell search file-viewer text vcs --lang all`
- Documentation work: `runbook category docs text formatter --lang markdown`
- Deployment or infrastructure work: `runbook category deploy cloud container infra database secrets security --lang all`

## Preference Rules

- `runbook scan` is fact-only; do not expect it to read or display preferences.
- Run `runbook prefer` after `runbook scan` during non-trivial repository work, even if you expect no preferences.
- `runbook category <category>... --lang <lang>` marks preferred tools from `.runbook/preferences.yaml`, then sorts candidates by preference, installed availability, language fit, risk, and name.
- Use `runbook prefer` to read existing repository preferences before recommending a durable tool choice.
- Do not call `runbook prefer set` unless the user explicitly confirms the preference or directly asks you to record it.
- The reason must be human-readable and repository-specific, such as why this repo uses `cargo-nextest` over `cargo test`.
- Treat preferences as guidance for future agent behavior, not as permission to ignore local scripts, missing binaries, or risk guardrails.

## Decision and Question Rules

- Compare candidates yourself first. Do not ask the user to choose a tool before checking scan, preferences, and category output.
- Ask the user only when there is real unresolved ambiguity: multiple plausible candidates, no local evidence, no preference, and a choice that would create durable project behavior or meaningful risk.
- Do not ask when local evidence or preferences resolve the choice, such as `uv.lock` indicating `uv`, `Cargo.toml` indicating Cargo, or `.runbook/preferences.yaml` preferring a shell.
- Before writing a new preference, ask for explicit confirmation unless the user directly instructed you to record it.
- If a tool choice is made without asking, be able to explain the evidence: local requirement, available tool, preference, category ordering, or guardrail.

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
