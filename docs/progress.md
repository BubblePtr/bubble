# Bubble Progress

最后更新：2026-04-03

## Workspace / Crate 拆分

| 项目 | 状态 | 说明 |
|---|---|---|
| 根 workspace | 已完成 | 根 `Cargo.toml` 已切换为 workspace |
| `bubble` 对外 crate | 已完成 | 现有单 crate 代码已迁入 `bubble/` |
| `crates/core` | 已完成 | 已创建最小 draft 核心抽象 |
| `crates/providers` | 已完成 | 已创建 feature 占位与请求模型 |
| `crates/tools` | 已完成 | 已创建最小工具注册接口 |
| `crates/context` | 已完成 | 已创建 trait 化上下文占位接口 |
| `crates/memory` | 已完成 | 已创建长期记忆 trait 占位 |
| `crates/evolution` | 已完成 | 已创建 crate 骨架，设计待定 |
| `crates/harness` | 已完成 | 已创建可观测性骨架 |
| 真实能力迁移到内部 crates | 未开始 | 仍由 `bubble` crate 承载现有实现 |

## bubble crate

| 项目 | 状态 | 说明 |
|---|---|---|
| tools 路径安全加固 | 已完成 | `resolve_path` 禁止绝对路径与 `..` 穿越，新增测试覆盖 |
| bash tool 描述更新 | 已完成 | 明确标注 "not sandboxed" |

## 教学网站 `book/`

| 项目 | 状态 | 说明 |
|---|---|---|
| Next.js + Nextra 基础脚手架 | 已完成 | 已创建 `package.json`、`next.config.mjs`、文档目录 |
| 首页与章节骨架 | 已完成 | 已创建 `index.mdx`、`ch01-repl.mdx` |
| 首页正文与开源项目推荐卡片 | 已完成 | 引入 `ProjectCard` 组件，推荐 Pi Agent / Hermes Agent |
| ch01 章节正文 | 已完成 | 完成 REPL 循环、tool calling、路径安全等内容 |
| 交互组件 | 已完成 | `AgentLoopAnimation`、`ReplCycleAnimation`、`ProjectCard` |
| 站点首页重新设计 | 已完成 | 简化首页布局，统一全局样式 |
| 步进式执行动画 | 未开始 | 本阶段不实现 |
| 站点本地静态构建 | 已完成 | 已在本地执行 `npm run build` 验证通过 |
| 站点 CI / 构建接入 | 未开始 | 当前未接入 CI，先保留本地验证路径 |

## SPEC 文档

| 项目 | 状态 | 说明 |
|---|---|---|
| 技术栈更新 | 已完成 | 从 Astro + Starlight 迁移至 Next.js + Nextra |
| 目录结构对齐 | 已完成 | 与实际代码结构保持一致 |

## 本地验证命令

### Rust workspace

```bash
cargo fmt --all --check
cargo check --workspace --all-features
cargo test --workspace
```

### Book

```bash
cd book
npm install
npm run build
```

说明：教学网站当前未接入 CI，本阶段以脚手架可继续扩展为目标。
