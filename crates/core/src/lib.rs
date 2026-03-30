//! Draft core abstractions for Bubble's future internal architecture.
//! These types are intentionally minimal in the scaffold phase.

use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Conversation {
    messages: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn messages(&self) -> &[Message] {
        &self.messages
    }
}

pub type ProviderFuture<'a> = Pin<Box<dyn Future<Output = anyhow::Result<Message>> + Send + 'a>>;

pub trait LlmProvider: Send + Sync {
    fn complete<'a>(&'a self, conversation: &'a Conversation) -> ProviderFuture<'a>;
}

pub struct Agent<P> {
    provider: P,
}

impl<P> Agent<P> {
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    pub fn provider(&self) -> &P {
        &self.provider
    }
}

#[cfg(test)]
mod tests {
    use super::{Conversation, Message, Role};

    #[test]
    fn conversation_stores_messages() {
        let mut conversation = Conversation::new();
        conversation.push(Message::new(Role::User, "hello"));

        assert_eq!(conversation.messages().len(), 1);
        assert_eq!(conversation.messages()[0].content, "hello");
    }
}
