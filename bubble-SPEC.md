# Bubble — Project SPEC

> 本文件由架构讨论生成，供 Agent 实现参考。
> 方向性问题找墨染讨论，具体实现参照本 SPEC。

---

## 项目定位

一个用 Rust 实现的通用 Agent 框架，目标是：

- **生产级可分发**：最终发布到 crates.io，供开发者作为依赖引入
- **渐进式模块化**：每个能力模块独立可用，按需组合
- **现代 Agentic 标准**：覆盖市面上主流 Agent 框架的核心模块
- **附带教学网站**：框架基本成型后，提供一个配套的原理讲解网站

框架是主，网站是副。网站解释"为什么这样设计"，不追踪代码的每次变化。

---

## 仓库结构（Monorepo）

```
bubble/
├── crates/
│   ├── core/          # 核心 trait + 基础类型（workspace 内部）
│   ├── providers/     # LLM Provider 实现（feature-flagged）
│   ├── tools/         # Tool 系统
│   ├── context/       # Context Engineering + 上下文窗口管理
│   ├── memory/        # 长期记忆系统
│   ├── evolution/     # 自我进化机制
│   └── harness/       # 可观测性 + tracing
├── bubble/            # 对外公开的门面 crate（唯一发布到 crates.io 的包）
├── book/              # 教学网站（Next.js + Nextra）
│   ├── content/docs/       # 章节内容（.mdx 文件）
│   ├── src/components/     # 交互式步进动画组件
│   └── next.config.mjs
├── examples/
│   ├── ch01-basic-repl/
│   ├── ch02-tools/
│   └── ...
└── Cargo.toml               # workspace 根
```

**命名说明：** `crates/` 下各子目录是 workspace 内部 crate，名称简短，不对外发布。对外只有一个 `bubble` crate，通过 feature flag 控制各模块的引入。

---

## Crate 架构

### 依赖链（严格单向，禁止上层依赖下层）

内部 workspace crate 使用短名，`bubble` 是对外的门面 crate：

```
harness
    ↓
evolution
    ↓
memory
    ↓
context
    ↓
tools
    ↓
providers   ←── 实现 core 中定义的 LlmProvider trait
    ↓
core        ←── 零内部依赖，可独立使用
    ↑
bubble      ←── 门面 crate，re-export 所有公开 API，feature flag 控制模块
```

**核心约束**：`core` 不得 import 任何其他内部 crate。任何违反此方向的依赖都是架构问题，需重新设计。

### 对外使用方式

```toml
# 用户的 Cargo.toml
[dependencies]
bubble = { version = "0.1", features = ["memory", "tools", "harness"] }
```

```rust
// 用户代码
use bubble::Agent;
use bubble::memory::MemoryStore;
use bubble::tools::ToolRegistry;
```

---

### core（内部 crate）

框架的基础层，定义所有核心抽象。

**职责：**
- 定义 `LlmProvider` trait（所有 Provider 必须实现此 trait）
- 定义核心类型：`Message`、`Role`、`Conversation`
- 定义 `Tool` trait（接口层，不含实现）
- 承载基础 REPL 循环与 agent 主协调逻辑
- `Agent` struct，协调 provider + tools + loop

**设计原则：**
- 无 async runtime 绑定（不强绑 tokio）
- 类型设计要考虑序列化（serde 可选 feature）

**当前实现说明：** 这部分真实可运行能力目前仍主要在 `bubble` crate，后续再逐步下沉到 `core`。

---

### providers（内部 crate）

实现具体 LLM 厂商的接入，**全部通过 feature flag 控制**，不强制引入用户不需要的依赖。

```toml
# crates/providers/Cargo.toml

[features]
default = []
openai     = ["dep:async-openai"]
anthropic  = ["dep:anthropic-sdk"]
ollama     = ["dep:ollama-rs"]
```

**职责：**
- `OpenAiProvider` — 实现 `LlmProvider`
- `AnthropicProvider` — 实现 `LlmProvider`
- `OllamaProvider` — 实现 `LlmProvider`
- 各 provider 共用的 HTTP 基础设施（retry、timeout、error 映射）

**注意：** Provider trait 的定义在 `core`，这里只有实现。

---

### tools（内部 crate）

Tool 的注册、dispatch、执行管道。

**职责：**
- `ToolRegistry`：运行时注册/查询 tool
- `ToolCall`、`ToolResult` 类型
- Tool 执行管道（调用前校验、调用后结果处理）
- 内置 tool（filesystem、shell、http）作为可选 feature

---

### context（内部 crate）

Context Engineering，即短期记忆 + 上下文窗口管理。复杂度与长期记忆系统同级，独立成模块。

**职责：**
- `ContextManager`：决定每次 LLM 调用时上下文里放什么
- Token 预算管理（计数、超限策略）
- 上下文压缩策略（摘要压缩、滑动窗口截断）
- Message 窗口化

**关键设计问题（待定）：**
- 压缩策略是 trait（可插拔）还是 enum（有限几种）？
- token 计数依赖 tokenizer 库，哪个？（tiktoken-rs？）

---

### memory（内部 crate）

长期记忆，跨会话持久化。

**职责：**
- `MemoryStore` trait（可插拔后端）
- 情景记忆（Episodic）：对话历史结构化存储
- 语义记忆（Semantic）：向量检索
- 默认后端：SQLite（`sqlite-vec` 扩展，支持向量）
- 检索结果注入 context 的接口（与 context 协作）

**关键设计问题（待定）：**
- 向量后端：sqlite-vec（轻量，零部署）vs qdrant（强大但需外部服务）？
- 建议默认 sqlite-vec，qdrant 作为可选 feature

---

### evolution（内部 crate）

自我进化机制（设计待深化，以下为初步方向）。

**职责：**
- Reflection loop：agent 对自身行为的反思与评估
- 性能评估：task 完成质量的自评指标
- 策略适应：基于历史表现调整行为偏好

**注意：** 此模块内部设计尚未确定，实现前需再讨论架构。

---

### harness（内部 crate）

可观测性，生产环境必备。

**职责：**
- 集成 `tracing` crate，为每个 agent 步骤创建 span
- 关键 metrics：token 用量、延迟、tool 调用频率、记忆命中率
- OpenTelemetry export（可选 feature）
- 本地 pretty-print 模式（开发调试用）

---

## 教学网站（book/）

### 技术栈

- **框架**：Next.js App Router + Nextra Docs Theme
- **交互组件**：React 组件（步进式执行流程动画）
- **架构图**：Mermaid.js
- **代码示例**：优先直接引用仓库源码，避免手动复制

### 代码 include 模式

当前站点基于 `book/content/docs/*.mdx` 和 `book/app/docs/[[...mdxPath]]/page.tsx` 渲染。
章节示例在内容稳定后再决定具体 include 方式。

### 章节结构

| 章节 | 主题 | 核心 crate |
|------|------|-----------|
| 1 | 基础 REPL + Tool Calling | core, tools |
| 2 | Context Engineering | context |
| 3 | 长期记忆系统 | memory |
| 4 | 自我进化机制 | evolution |
| 5 | Harness 与可观测性 | harness |

### 交互组件规范

每章包含一个「步进式执行流程」组件，规格如下：

- 用户点击「下一步」手动推进
- 每步高亮架构图中的对应节点 + 简短文字说明
- **不支持用户输入 prompt、不做实时执行**，纯静态动画
- 实现为 React 组件，本地状态管理（useState）
- 典型场景：REPL 循环各阶段、tool call invoke/response 流、记忆 write/retrieve 路径

---

## 约束与原则

1. `core` 零内部依赖 — 可单独作为依赖引入，不强迫用户带入整个框架
2. **Provider 用 feature flag，不拆多 crate** — 参考 sqlx 的 driver 模式
3. **网站文档解释设计决策，不追踪代码状态** — 代码变，文档不必跟着动
4. **每个 crate 独立可测** — 不允许必须组合多个 crate 才能跑测试
5. **crates.io 分发等产品成熟后再做** — 当前阶段内部迭代优先
6. **多语言支持（Python/TypeScript 代码 tab）推迟决策** — 等框架稳定后再加

---

## 开放问题（实现前需决策）

- [ ] `context` 的压缩策略：trait 可插拔 vs enum 有限选项
- [ ] Token 计数库选型（tiktoken-rs？）
- [ ] `memory` 向量后端默认选 sqlite-vec 还是留 trait 让用户自选
- [ ] `evolution` 内部架构（需专门讨论）
- [ ] 多 Agent 协调（Multi-agent）是否需要独立 crate，或归入 `core`
- [ ] `core` 是否绑定 tokio，或保持 async-agnostic

---

*最后更新：2026-03-29，基于架构讨论生成*
