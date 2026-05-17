# Roadmap

Runbook should stay focused on helping AI coding agents choose and operate command-line tools safely. It is not a general encyclopedia of frameworks, platforms, or libraries.

## Near term

### Platform filtering

Status: implemented.

- CLI supports `runbook category <category> --platform <platform>`.
- Web index supports a Platform filter.
- Tool cards may declare `platform` separately from `lang`.

### Registry curation guidelines

Define and document what belongs in `data/tools`:

- Include tools with an agent-useful action surface: search, read, edit, build, test, lint, format, debug, inspect, deploy, benchmark, scan, package, or coordinate work.
- Do not include arbitrary framework or library names unless they expose a meaningful CLI/action surface that agents should choose or guard.
- Keep `lang` as programming language or file ecosystem.
- Keep `platform` as execution/deployment target or environment such as `android`, `ios`, `web`, `desktop`, `cloud`, `local`, or `network`.

### Registry lint command

Add a maintainer-facing command such as:

```bash
runbook registry lint
```

It should validate YAML cards with clearer errors than build-time panics:

- filename/name consistency
- duplicate or empty `category`, `lang`, and `platform` values
- controlled values for `lang`, `platform`, risk levels, and risk effects
- invalid `all` combinations such as `lang: [all, rust]`
- missing docs, guardrails, or detection metadata

### Scan performance

Current global scanning is slow because it checks every registry tool. `scan --minimal` skips version commands, but it still resolves each candidate binary/alias through the system command resolver.

Observed on Windows:

- `scan --local --minimal` is fast because it only checks project files.
- `scan --global --minimal` remains slow because it runs `where.exe` for every tool and alias.
- Batched `where.exe tool1 tool2 ...` did not improve runtime in local testing; it appears to still resolve each argument internally and emits many miss messages.

Candidate fixes:

- Avoid shelling out once per tool where possible. On Windows, either use a faster resolver than repeated `where.exe` or maintain a PATH/shim index that matches command-resolution semantics closely enough for agent use.
- Consider a two-tier resolver: fast availability scan by indexed PATH/shims, with slower system resolution only for ambiguous or high-value misses.
- Make normal `scan` version probing optional or bounded by per-command timeouts; version data is useful for debugging but should not dominate preflight.
- Cache global scan results for a short TTL because installed tools rarely change during one agent session.

## Research track

### Local discovery via existing tools

Do not rebuild a full project/stack detector inside Runbook without research first. Investigate existing tools that can identify project type, languages, package managers, runtimes, and workflow hints, then decide whether Runbook should call them, read their output, or simply recommend them.

Tools and areas to evaluate:

- `github-linguist` / `enry` for language and repository composition.
- `tokei`, `scc`, or `cloc` for language/file statistics.
- `mise`, `asdf`, `proto`, `devbox`, and `nix` for runtime/tool version declarations.
- ecosystem-native files and commands such as Cargo, npm/pnpm/yarn, uv/poetry, Gradle, Flutter, CocoaPods, Expo/EAS, and Fastlane.
- CI and workflow helpers such as `actionlint`, `pre-commit`, `lefthook`, `husky`, and `lint-staged`.

Evaluation criteria:

- machine-readable output
- Windows support
- speed on medium-sized repositories
- no surprising network access
- no mutation during detection
- install footprint
- how well output maps to Runbook tools, categories, languages, platforms, and guardrails

## Later

### Subcommand/action-surface model

Some important agent actions are subcommands rather than standalone binaries, for example `git grep`, `docker compose`, `npm audit`, `pnpm audit`, `cargo test`, and `xcrun simctl`.

Avoid forcing these into fake standalone tools. Design a small action-surface model only after enough registry examples justify it.

### Machine-readable output

Add stable JSON output for integration with other agents and scripts:

```bash
runbook scan --json
runbook category test --lang rust --json
runbook prefer --json
```

### Better preference ergonomics

Make preferences easier to inspect and maintain without silently writing them. Keep explicit user confirmation as the rule for durable repository preferences.

## Not planned for now

### Free-form recommend command

A command such as `runbook recommend "extract text from pdf"` is intentionally deferred. It would require natural-language intent mapping, ranking heuristics, and ongoing maintenance. The current direction is to keep tool choice explicit through categories, languages, platforms, local evidence, and preferences.

### Broad framework encyclopedia

Runbook should not try to list every frontend, backend, mobile, AI, or database framework. Frameworks belong only when they provide a CLI/action surface that an agent should execute, compare, or handle with guardrails.
