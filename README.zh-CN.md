<div align="center">

# Runbook

**面向 AI 编程 Agent 的本地操作契约。**

[![Crates.io](https://img.shields.io/crates/v/agent-runbook?style=flat-square)](https://crates.io/crates/agent-runbook)
[![Downloads](https://img.shields.io/crates/d/agent-runbook?style=flat-square)](https://crates.io/crates/agent-runbook)
[![Docs.rs](https://img.shields.io/docsrs/agent-runbook?style=flat-square)](https://docs.rs/agent-runbook)
[![skills.sh](https://skills.sh/b/Ariestar/agent-runbook)](https://skills.sh/Ariestar/agent-runbook)
[![GitHub stars](https://img.shields.io/github/stars/Ariestar/agent-runbook?style=flat-square)](https://github.com/Ariestar/agent-runbook/stargazers)
[![Last commit](https://img.shields.io/github/last-commit/Ariestar/agent-runbook?style=flat-square)](https://github.com/Ariestar/agent-runbook/commits)
[![Repo size](https://img.shields.io/github/repo-size/Ariestar/agent-runbook?style=flat-square)](https://github.com/Ariestar/agent-runbook)
[![License](https://img.shields.io/crates/l/agent-runbook?style=flat-square)](https://github.com/Ariestar/agent-runbook)

[English](README.md) · [简体中文](README.zh-CN.md)

[安装](#安装) · [快速开始](#快速开始) · [Agent Skill](#agent-skill) · [命令](#命令)

</div>

Runbook 会在 AI 编程 Agent 修改文件之前，扫描当前项目和本机环境。它把本地事实转换成明确的工具选择、操作护栏和仓库级偏好，避免 Agent 猜错包管理器、Shell、测试命令、部署工具或风险边界。

```bash
runbook scan
runbook category test lint formatter --lang rust
runbook prefer set test --lang rust --tool cargo-nextest --reason "Use nextest for faster Rust test runs."
```

## 为什么需要 Runbook？

AI 编程 Agent 进入一个仓库时，通常拿到的上下文是不完整的。它可能在 `pnpm` 项目里运行 `npm install`，在要求 Unix 风格 Shell 的 Windows 项目里写 PowerShell 语法，忽略仓库指定的测试工具，或者把本该脱敏的环境变量直接打印出来。

Runbook 提供一个很小的预检步骤：

- 从 `Cargo.toml`、`Cargo.lock`、`package.json`、`.git` 等文件检测项目本地要求
- 检测本机可用工具及其版本
- 按任务类别和语言暴露候选工具
- 在 `.runbook/preferences.yaml` 中记录明确的仓库级偏好
- 为包管理器、密钥、远程写入和破坏性工具提供操作护栏

> [!NOTE]
> `runbook scan` 只输出事实。仓库偏好只能通过 `runbook prefer` 写入，并由 `runbook category` 消费。

## 安装

从 crates.io 安装 CLI：

```bash
cargo install agent-runbook
```

也可以直接从仓库安装：

```bash
cargo install --git https://github.com/Ariestar/agent-runbook.git
```

确认安装成功：

```bash
runbook --version
```

## 快速开始

进入项目目录后运行扫描：

```bash
runbook scan
```

输出结构示例：

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

当 Agent 不确定应该使用哪类工具时，查询候选工具：

```bash
runbook category test lint formatter --lang rust
```

只有在用户或团队确认这是稳定仓库偏好后，才写入偏好：

```bash
runbook prefer set test --lang rust --tool cargo --reason "Use Cargo as the default Rust test runner."
```

Runbook 会写入：

```yaml
schema: 1
preferences:
  - category: test
    lang: rust
    tool: cargo
    reason: Use Cargo as the default Rust test runner.
```

## Agent Skill

Runbook 的产品形态是 CLI 内核加 Agent Skill。CLI 提供确定、可测试、可版本化的输出；Skill 告诉 Agent 什么时候调用它，以及如何解释结果。

通过 Skills CLI 安装 Codex skill：

```bash
npx skills add Ariestar/agent-runbook --skill runbook-skill
```

为支持的 Agent 全局安装：

```bash
npx skills add Ariestar/agent-runbook --skill runbook-skill --global
```

这个 skill 会要求 Agent：

- 在非平凡仓库工作前运行 `runbook --version`、`runbook scan`、`runbook prefer`，以及一个和任务相关的 `runbook category ... --lang ...` 查询
- 把 `runbook scan` 当作事实盘点，而不是完整的工具选择流程
- 在询问用户选择工具前，先自行比较 category 候选工具
- 不要静默写入偏好
- 对云、数据库、密钥、部署和破坏性操作保持确认门槛

## 命令

| 命令 | 作用 |
| --- | --- |
| `runbook scan` | 扫描本机工具和当前项目要求 |
| `runbook scan --global` | 只扫描本机工具 |
| `runbook scan --local` | 只扫描当前项目要求 |
| `runbook scan --minimal` | 输出紧凑的工具名列表 |
| `runbook category` | 列出功能类别 |
| `runbook category <category>... --lang <lang>` | 按任务和语言查看候选工具 |
| `runbook prefer` | 查看仓库本地工具偏好 |
| `runbook prefer set <category> --lang <lang> --tool <tool> --reason <text>` | 记录已确认的仓库偏好 |
| `runbook prefer unset <category> --lang <lang>` | 移除过期仓库偏好 |
| `runbook --version` | 输出已安装 CLI 版本 |

## 工具注册表

Runbook 内置一个基于 YAML 的工具注册表。每个工具规格可以描述：

- 命令名和别名
- `build`、`test`、`lint`、`shell`、`deploy`、`database`、`security` 等类别
- 适用语言
- 版本检测方式
- 项目本地检测方式
- 适用和避免使用的场景
- 风险等级和副作用
- 操作护栏

这个注册表会编译进 CLI，同时也驱动 `apps/web` 中的静态工具索引。

```bash
runbook category
```

## Web 工具索引

仓库中包含一个轻量的静态工具索引页面：

```bash
cd apps/web
python -m http.server 8000
```

打开 `http://localhost:8000`，即可按名称、类别、语言和风险等级搜索工具。

## 开发

在仓库根目录构建和测试：

```bash
cargo fmt --check
cargo clippy
cargo test
```

开发时运行本地 CLI：

```bash
cargo run -- scan
cargo run -- category shell --lang all
cargo run -- prefer
```
