# Runbook

Runbook scans the current project and machine so AI coding agents can choose the right tools before they start changing files.

## CLI

```bash
cargo install agent-runbook
runbook scan
runbook scan --global
runbook scan --local
```

## Skill

The Codex skill lives at `skills/runbook-skill`. Install it through the Skills CLI:

```bash
npx skills add Ariestar/agent-runbook --skill runbook-skill
```

To install globally for supported agents:

```bash
npx skills add Ariestar/agent-runbook --skill runbook-skill --global
```

Discoverable skill distribution is handled by https://skills.sh/. The `runbook` CLI itself is distributed through Cargo.
