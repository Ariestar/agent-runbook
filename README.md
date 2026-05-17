# Runbook

Runbook scans the current project and machine so AI coding agents can choose the right tools before they start changing files.

## CLI

```bash
cargo install agent-runbook
runbook scan
runbook scan --global
runbook scan --local
runbook scan --minimal
runbook category
runbook category --help
runbook category lint formatter --lang rust
runbook prefer
runbook prefer set test --lang rust --tool cargo --reason "Use Cargo as the default Rust test runner."
runbook prefer unset test --lang rust
runbook --version
```

`runbook scan` is fact-only. Repository tool preferences are stored explicitly in `.runbook/preferences.yaml` through `runbook prefer`, then surfaced by `runbook category` as preferred candidates. Category results are sorted by repository preference, installed availability, language fit, risk, and name.

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
