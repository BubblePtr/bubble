# bubble-providers

目标职责：提供各 LLM 厂商接入层，实现 `bubble-core` 中定义的 provider trait。

当前状态：脚手架阶段，仅保留 crate 边界、feature flag 和占位请求/响应模型。

下一步：先实现共用 HTTP 基础设施，再按 feature 增量接入 OpenAI、Anthropic、Ollama。
