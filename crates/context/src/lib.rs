//! Draft context engineering layer for Bubble.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenBudget {
    pub max_tokens: usize,
}

pub trait TokenCounter: Send + Sync {
    fn count_tokens(&self, input: &str) -> usize;
}

pub trait CompressionStrategy: Send + Sync {
    fn compress(&self, messages: &[String], budget: TokenBudget) -> Vec<String>;
}

pub trait ContextManager: Send + Sync {
    fn build_context(&self, messages: &[String], budget: TokenBudget) -> Vec<String>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TiktokenRsPlanned;

#[cfg(test)]
mod tests {
    use super::TokenBudget;

    #[test]
    fn budget_is_constructible() {
        let budget = TokenBudget { max_tokens: 4096 };
        assert_eq!(budget.max_tokens, 4096);
    }
}
