use anyhow::{Context, Result, bail};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::Config;
use crate::tools::ToolCall;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMessage {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

impl ChatMessage {
    pub fn system(content: String) -> Self {
        Self {
            role: String::from("system"),
            content: Some(content),
            tool_call_id: None,
            tool_calls: None,
        }
    }

    pub fn user(content: String) -> Self {
        Self {
            role: String::from("user"),
            content: Some(content),
            tool_call_id: None,
            tool_calls: None,
        }
    }

    pub fn assistant(content: Option<String>, tool_calls: Vec<ToolCall>) -> Self {
        Self {
            role: String::from("assistant"),
            content,
            tool_call_id: None,
            tool_calls: if tool_calls.is_empty() {
                None
            } else {
                Some(tool_calls)
            },
        }
    }

    pub fn tool(tool_call_id: String, content: String) -> Self {
        Self {
            role: String::from("tool"),
            content: Some(content),
            tool_call_id: Some(tool_call_id),
            tool_calls: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AssistantMessage {
    pub content: Option<String>,
    #[serde(default)]
    pub tool_calls: Vec<ToolCall>,
}

#[derive(Clone)]
pub struct OpenAiClient {
    client: Client,
    config: Config,
}

impl OpenAiClient {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub async fn chat(
        &self,
        messages: &[ChatMessage],
        tools: &[Value],
    ) -> Result<AssistantMessage> {
        let url = format!(
            "{}/chat/completions",
            self.config.base_url.trim_end_matches('/')
        );
        let response = self
            .client
            .post(url)
            .bearer_auth(&self.config.api_key)
            .json(&serde_json::json!({
                "model": self.config.model,
                "messages": messages,
                "tools": tools,
                "tool_choice": "auto"
            }))
            .send()
            .await
            .context("chat completion request failed")?;

        let status = response.status();
        let body = response
            .text()
            .await
            .context("failed to read response body")?;
        if !status.is_success() {
            bail!("chat completion failed with status {status}: {body}");
        }

        let response: ChatCompletionResponse =
            serde_json::from_str(&body).context("failed to parse chat completion response")?;
        let choice = response
            .choices
            .into_iter()
            .next()
            .context("chat completion returned no choices")?;

        Ok(AssistantMessage {
            content: choice.message.content,
            tool_calls: choice.message.tool_calls,
        })
    }
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatChoiceMessage,
}

#[derive(Debug, Deserialize)]
struct ChatChoiceMessage {
    content: Option<String>,
    #[serde(default)]
    tool_calls: Vec<ToolCall>,
}

#[cfg(test)]
mod tests {
    use super::ChatCompletionResponse;

    #[test]
    fn parses_tool_calls_response() {
        let response: ChatCompletionResponse = serde_json::from_str(
            r#"{
                "choices": [{
                    "message": {
                        "content": null,
                        "tool_calls": [{
                            "id": "call_1",
                            "type": "function",
                            "function": {
                                "name": "read",
                                "arguments": "{\"path\":\"Cargo.toml\"}"
                            }
                        }]
                    }
                }]
            }"#,
        )
        .unwrap();

        assert_eq!(
            response.choices[0].message.tool_calls[0].function.name,
            "read"
        );
    }
}
