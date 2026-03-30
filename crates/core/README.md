# bubble-core

目标职责：承载 Bubble 的核心抽象，包括消息模型、provider trait 和 agent 主协调接口。

当前状态：脚手架阶段，仅提供最小 draft 类型定义，不承载现有 REPL 或 agent loop 实现。

下一步：在保持 runtime-agnostic 的前提下，逐步把 `bubble` crate 中的稳定核心能力下沉到这里。
