# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概览

**Bubble** 是一个用 Rust 实现的通用 Agent 框架，目标是生产级可分发（发布到 crates.io）、渐进式模块化。框架附带教学网站（`book/`），解释设计原理，不追踪代码变化。

权威架构文档：[bubble-SPEC.md](bubble-SPEC.md)（中文）

## 常用命令

```bash
# Rust
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo check --workspace --all-features
cargo test --workspace
cargo test -p bubble-core          # 单个 crate 测试

# 教学网站
cd book && npm install
npm run dev      # 启动开发服务器
npm run build    # 生产构建
```

pre-push hook 由 `cargo-husky` 管理，自动执行 fmt + clippy + test。

## 仓库结构

```
bubble/
├── crates/          # 内部 workspace crates（不发布）
│   ├── core/        # Message / LlmProvider trait / Agent struct（零内部依赖）
│   ├── providers/   # OpenAI / Anthropic / Ollama 实现（feature-gated）
│   ├── tools/       # ToolRegistry、ToolCall、执行管道
│   ├── context/     # Token 预算、上下文压缩
│   ├── memory/      # 长期记忆（SQLite 默认）
│   ├── evolution/   # 反思与性能自适应
│   └── harness/     # tracing、metrics、OpenTelemetry
├── bubble/          # 唯一对外门面 crate（feature flags 控制模块引入）
├── book/            # 教学网站（Next.js + Nextra）
└── bubble-SPEC.md   # 架构规范
```

## 依赖方向（严格单向，禁止循环）

```
core → providers → tools → context → memory → evolution → harness
```

`core` 是零依赖的纯 trait 层；`bubble` 是唯一对外门面，通过 feature flags 聚合。

## Feature Flags（bubble crate）

```toml
bubble = { version = "0.1", features = ["memory", "tools", "harness"] }
```

可用 feature：`providers`、`tools`、`context`、`memory`、`evolution`、`harness`。

## 核心类型

- `Role`：System / User / Assistant / Tool
- `Message`：role + content
- `LlmProvider` trait：`complete(&conversation) -> ProviderFuture`
- `Agent<P>`：provider 泛型 agent 结构体
- `ToolRegistry`：BTreeMap 背后的工具注册表
- `ToolCall` / `ToolResult`：工具调用入参与返回

## 关键约束

- **禁止 unsafe**：workspace 级别 `unsafe_code = "forbid"`
- 新增内部 crate 须加入 `Cargo.toml` `[workspace.members]` 并在 `bubble` 中添加对应 feature flag
- Provider 实现须实现 `bubble-core` 的 `LlmProvider` trait
