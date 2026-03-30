//! Draft observability layer for Bubble.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpanRecord {
    pub name: String,
}

pub trait Harness: Send + Sync {
    fn record_span(&self, span: SpanRecord) -> anyhow::Result<()>;
}

#[cfg(test)]
mod tests {
    use super::SpanRecord;

    #[test]
    fn span_record_stores_name() {
        let span = SpanRecord {
            name: String::from("agent.step"),
        };

        assert_eq!(span.name, "agent.step");
    }
}
