//! Draft self-evolution layer for Bubble.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReflectionNote {
    pub summary: String,
}

pub trait ReflectionEngine: Send + Sync {
    fn reflect(&self, input: &str) -> anyhow::Result<ReflectionNote>;
}

#[cfg(test)]
mod tests {
    use super::ReflectionNote;

    #[test]
    fn note_can_be_created() {
        let note = ReflectionNote {
            summary: String::from("design pending"),
        };

        assert_eq!(note.summary, "design pending");
    }
}
