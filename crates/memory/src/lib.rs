//! Draft long-term memory layer for Bubble.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryRecord {
    pub key: String,
    pub value: String,
}

pub trait MemoryStore: Send + Sync {
    fn put(&mut self, record: MemoryRecord) -> anyhow::Result<()>;
    fn get(&self, key: &str) -> anyhow::Result<Option<MemoryRecord>>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SqliteVecPlanned;

#[cfg(test)]
mod tests {
    use super::MemoryRecord;

    #[test]
    fn record_keeps_key_and_value() {
        let record = MemoryRecord {
            key: String::from("project"),
            value: String::from("bubble"),
        };

        assert_eq!(record.key, "project");
        assert_eq!(record.value, "bubble");
    }
}
