//! Draft provider layer for Bubble.

use bubble_core::{Conversation, Message};

pub trait ProviderFactory {
    type Provider;

    fn build(&self) -> Self::Provider;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderKind {
    OpenAi,
    Anthropic,
    Ollama,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProviderRequest {
    pub conversation_len: usize,
}

impl From<&Conversation> for ProviderRequest {
    fn from(conversation: &Conversation) -> Self {
        Self {
            conversation_len: conversation.messages().len(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderResponse {
    pub message: Message,
}

#[cfg(test)]
mod tests {
    use bubble_core::{Conversation, Message, Role};

    use super::ProviderRequest;

    #[test]
    fn request_can_be_derived_from_conversation() {
        let mut conversation = Conversation::new();
        conversation.push(Message::new(Role::User, "ping"));

        let request = ProviderRequest::from(&conversation);
        assert_eq!(request.conversation_len, 1);
    }
}
