<div align="center">

# Runbook

**A local operating contract for AI coding agents.**

[![Crates.io](https://img.shields.io/crates/v/agent-runbook?style=flat-square)](https://crates.io/crates/agent-runbook)
[![Downloads](https://img.shields.io/crates/d/agent-runbook?style=flat-square)](https://crates.io/crates/agent-runbook)
[![Docs.rs](https://img.shields.io/docsrs/agent-runbook?style=flat-square)](https://docs.rs/agent-runbook)
[![skills.sh](https://skills.sh/b/Ariestar/agent-runbook)](https://skills.sh/Ariestar/agent-runbook)
[![GitHub stars](https://img.shields.io/github/stars/Ariestar/agent-runbook?style=flat-square)](https://github.com/Ariestar/agent-runbook/stargazers)
[![Last commit](https://img.shields.io/github/last-commit/Ariestar/agent-runbook?style=flat-square)](https://github.com/Ariestar/agent-runbook/commits)
[![Repo size](https://img.shields.io/github/repo-size/Ariestar/agent-runbook?style=flat-square)](https://github.com/Ariestar/agent-runbook)
[![License](https://img.shields.io/crates/l/agent-runbook?style=flat-square)](https://github.com/Ariestar/agent-runbook)

[English](README.md) · [简体中文](README.zh-CN.md)

[Install](#install) · [Quick Start](#quick-start) · [Agent Skill](#agent-skill) · [Commands](#commands)

</div>

Runbook scans the current project and machine before an AI coding agent starts changing files. It turns local facts into explicit tool choices, guardrails, and repository preferences so the agent does not guess the wrong package manager, shell, test runner, deployment tool, or risk boundary.

```bash
runbook scan
runbook category test lint formatter --lang rust
runbook prefer set shell --lang all --tool brush --reason "Use brush for Unix-style commands on Windows."
```

## Why Runbook?

AI coding agents often enter a repository with incomplete context. They may run `npm install` in a `pnpm` project, use PowerShell syntax where a Unix-like shell was requested, ignore a repo-specific test runner, or print environment values that should stay redacted.

Runbook gives the agent a small preflight step:

- detect project-local requirements from files such as `Cargo.toml`, `Cargo.lock`, `package.json`, and `.git`
- detect available machine tools and versions
- expose task-specific tool candidates by category and language
- record explicit repository preferences in `.runbook/preferences.yaml`
- surface guardrails for package managers, secrets, remote writes, and destructive tools

> [!NOTE]
> `runbook scan` is fact-only. Repository preferences are written only through `runbook prefer` and then consumed by `runbook category`.

## Install

Install the CLI from crates.io:

```bash
cargo install agent-runbook
```

Or install directly from the repository:

```bash
cargo install --git https://github.com/Ariestar/agent-runbook.git
```

Verify the binary:

```bash
runbook --version
```

## Quick Start

Start in a project directory and run a scan:

```bash
runbook scan
```

Example output shape:

```text
Agent Runbook Scan
Mode: all
Project: /path/to/project

Global Tools
- cargo: cargo (cargo 1.95.0 ...)
- rg: rg (ripgrep 15.1.0 ...)

Local Requirements
- cargo: Cargo.toml
- git: .git

Recommended Operating Guardrails
- Use cargo for Rust build, test, and run commands when Cargo.toml exists. [Cargo.toml]

Warnings
- None
```

Ask for candidates when the right tool family is not obvious:

```bash
runbook category test lint formatter --lang rust
```

Store a durable repository preference only after the user or team confirms it:

```bash
runbook prefer set test --lang rust --tool cargo --reason "Use Cargo as the default Rust test runner."
```

Runbook writes preferences to:

```yaml
schema: 1
preferences:
  - category: test
    lang: rust
    tool: cargo
    reason: Use Cargo as the default Rust test runner.
```

## Agent Skill

Runbook is designed as a CLI kernel plus an agent skill. The CLI provides deterministic, testable output; the skill teaches agents when to call it and how to interpret the result.

Install the Codex skill with the Skills CLI:

```bash
npx skills add Ariestar/agent-runbook --skill runbook-skill
```

Install it globally for supported agents:

```bash
npx skills add Ariestar/agent-runbook --skill runbook-skill --global
```

The skill tells the agent to:

- run `runbook --version`, `runbook scan`, `runbook prefer`, and a task-relevant `runbook category ... --lang ...` query before non-trivial repository work
- treat `runbook scan` as a fact inventory, not a complete tool-choice workflow
- compare category candidates before asking the user to choose a tool
- avoid silently writing preferences
- treat high-risk cloud, database, secrets, deployment, and destructive operations as confirmation-gated

## Commands

| Command | Purpose |
| --- | --- |
| `runbook scan` | Scan machine tools and current-project requirements |
| `runbook scan --global` | Scan only machine-level tools |
| `runbook scan --local` | Scan only current-project requirements |
| `runbook scan --minimal` | Print compact tool-name output |
| `runbook category` | List functional tool categories |
| `runbook category <category>... --lang <lang>` | Inspect candidate tools for a task and language |
| `runbook prefer` | List repository-local tool preferences |
| `runbook prefer set <category> --lang <lang> --tool <tool> --reason <text>` | Record a confirmed repository preference |
| `runbook prefer unset <category> --lang <lang>` | Remove a stale repository preference |
| `runbook --version` | Print the installed CLI version |

## Tool Registry

Runbook ships with a YAML-backed registry of tool metadata. Each tool spec can describe:

- command name and aliases
- categories such as `build`, `test`, `lint`, `shell`, `deploy`, `database`, or `security`
- language fit
- version detection
- local project detection
- use and avoid guidance
- risk level and side effects
- operating guardrails

The registry is compiled into the CLI and also powers the static tool index in `apps/web`.

```bash
runbook category
```

## Web Tool Index

The repository includes a small static browser for the tool registry:

```bash
cd apps/web
python -m http.server 8000
```

Open `http://localhost:8000` to search tools by name, category, language, and risk level.

## Development

Build and test from the repository root:

```bash
cargo fmt --check
cargo clippy
cargo test
```

Run the local CLI during development:

```bash
cargo run -- scan
cargo run -- category shell --lang all
cargo run -- prefer
```
