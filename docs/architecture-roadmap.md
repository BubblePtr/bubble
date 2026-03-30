# Bubble Architecture Roadmap

本文件记录从当前过渡实现迁移到目标 monorepo 架构的路径，不替代 [`bubble-SPEC.md`](../bubble-SPEC.md)。

## 当前状态

- 仓库已拆为 workspace，`bubble` crate 继续承载现有可运行实现。
- `crates/*` 已建立边界和最小 draft 接口，但尚未承接实际行为。
- 教学网站 `book/` 已创建骨架，内容仍以章节计划和设计说明为主。

## 迁移路线

### 阶段 1：脚手架完成

- 完成 workspace 根和 `bubble/` 对外 crate 重组
- 建立 `core/providers/tools/context/memory/evolution/harness` 边界
- 补进度文档和 `book/` 基础目录

### 阶段 2：下沉稳定内核

- 将消息模型、agent 循环、provider trait 等稳定抽象迁入 `bubble-core`
- 将现有 tool 运行模型迁入 `bubble-tools`
- 让 `bubble` crate 转为更薄的 facade 和 re-export 层

### 阶段 3：补首个模块实现

- `providers`：先接 OpenAI 默认实现
- `context`：先落 token 预算和最小窗口策略
- `memory`：先落可选默认存储方向
- `harness`：先落 tracing span 与基础 metrics

### 阶段 4：扩展能力

- 继续演进 `memory`、`context` 和 `harness`
- 专门评审 `evolution` / multi-agent 的边界后再实现
- 教学网站补交互组件和章节正文

## 已固定的架构约束

- `bubble` 是唯一对外 crate。
- `bubble-core` 不依赖任何其他内部 crate。
- `context` 采用 trait 路线，不把压缩策略先收敛为 enum。
- `memory` 默认方向记为 SQLite / `sqlite-vec`，但当前不绑定实现。
- 教学网站用于解释设计，不追踪每次代码微调。
