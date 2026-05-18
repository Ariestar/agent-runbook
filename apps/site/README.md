# Runbook

The operating contract layer for AI coding agents.

Runbook helps agents choose shell commands deliberately instead of guessing. It combines local project discovery, durable repository preferences, and a structured CLI registry with risk, effects, and guardrails.

## What is this site?

This is the Runbook web registry: a curated, machine-readable catalog of CLI/action tools that AI coding agents can safely reason about. Each tool entry includes:

- **use_when** — when an agent should pick this tool
- **avoid_when** — when another tool is better
- **guardrails** — safety rules for agents
- **risk** — low / medium / high / critical
- **effects** — read_files, write_files, execute_code, network_access, etc.
- **category / lang / platform** — metadata for task-local tool selection

## Use with Runbook CLI

```bash
runbook scan
runbook prefer
runbook category search --lang rust
runbook category test lint --lang typescript
```

The registry becomes useful when paired with local evidence from `runbook scan`: what is installed, what the project requires, and which tools are risky or unavailable.

## Browse

Visit the website or use the JSON API:

```text
registry at `../../awesome-agent-cli/data/tools/`
```

## Contribute

Add a YAML file to the `awesome-agent-cli` repository: `data/tools/your-tool.yaml`. See `/contribute` for the template and field reference.

## Development

```bash
pnpm install
pnpm dev       # dev server at localhost:4321
pnpm build     # static build to dist/
pnpm preview   # preview the build
```

## Tech Stack

- [Astro](https://astro.build) — static site framework
- [React](https://react.dev) — interactive search island
- [Tailwind CSS](https://tailwindcss.com) — styling
- Tool data: YAML files from the root `awesome-agent-cli` submodule at `../../awesome-agent-cli/data/tools/`

## License

ISC
