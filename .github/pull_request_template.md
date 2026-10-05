## Summary
<!-- Describe the changes made and the motivation behind them -->

---

## Type of Change
- [ ] Bugfix
- [ ] New Feature
- [ ] Refactor / Optimization
- [ ] Documentation / CI

---

## Pre-submission Quality Checklist
Before requesting review, ensure your branch passes all standard checks:

- [ ] Rebased onto latest `main`: `git pull --rebase origin main`
- [ ] Formatted: `cargo fmt --all -- --check`
- [ ] Linter passing with zero warnings: `cargo clippy --all-targets -- -D warnings`
- [ ] Tests passing: `cargo test`
- [ ] Registry linter passing: `python awesome-agent-cli/scripts/lint_registry.py`
