//! Draft tool system for Bubble.

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCall {
    pub name: String,
    pub payload: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolResult {
    pub output: String,
}

pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn execute(&self, call: &ToolCall) -> anyhow::Result<ToolResult>;
}

#[derive(Default)]
pub struct ToolRegistry {
    tools: BTreeMap<&'static str, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        self.tools.insert(tool.name(), tool);
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{Tool, ToolCall, ToolRegistry, ToolResult};

    struct EchoTool;

    impl Tool for EchoTool {
        fn name(&self) -> &'static str {
            "echo"
        }

        fn execute(&self, call: &ToolCall) -> anyhow::Result<ToolResult> {
            Ok(ToolResult {
                output: call.payload.clone(),
            })
        }
    }

    #[test]
    fn registry_tracks_registered_tools() {
        let mut registry = ToolRegistry::new();
        registry.register(Box::new(EchoTool));

        assert_eq!(registry.len(), 1);
    }
}
