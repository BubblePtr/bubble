# bubble-context

目标职责：负责短期记忆和上下文窗口管理，包括 token 预算与压缩策略。

当前状态：脚手架阶段，采用 trait 路线预留 `ContextManager`、`TokenCounter` 和 `CompressionStrategy`。

下一步：确定 tokenizer 选型，优先按 `tiktoken-rs` 方向补首个默认实现。
