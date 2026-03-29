use std::future::Future;
use std::io::{self, Write};
use std::path::Path;

use anyhow::{Context, Result};
use bubble::config::Config;
use bubble::llm::{AssistantMessage, ChatMessage, OpenAiClient};
use bubble::{skills, tools};

const BASE_PROMPT: &str = r#"You are a minimal coding agent.
Use tools when needed.
Be concise and direct.
Before changing files, inspect relevant context.
Use edit for targeted changes, write for full file replacement, read for file reads, and bash for shell commands."#;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env_and_args()?;
    let all_skills = skills::load_skills(&config.cwd.join("skills"))?;
    let skill_prompts = skills::selected_skill_prompts(&all_skills, &config.enabled_skills)?;

    let mut messages = vec![ChatMessage::system(build_system_prompt(&skill_prompts))];
    let tool_defs = tools::definitions();
    let client = OpenAiClient::new(config.clone());

    repl(&config.cwd, &mut messages, |history| {
        let client = client.clone();
        let tool_defs = tool_defs.clone();
        async move { client.chat(&history, &tool_defs).await }
    })
    .await
}

async fn repl<F, Fut>(cwd: &Path, messages: &mut Vec<ChatMessage>, mut chat: F) -> Result<()>
where
    F: FnMut(Vec<ChatMessage>) -> Fut,
    Fut: Future<Output = Result<AssistantMessage>>,
{
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().context("failed to flush stdout")?;

        let mut line = String::new();
        let read = stdin.read_line(&mut line).context("failed to read stdin")?;
        if read == 0 {
            break;
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if matches!(line, ":q" | ":quit" | ":exit") {
            break;
        }

        let answer = run_agent_turn(cwd, messages, &mut chat, line).await?;
        println!("{answer}");
    }

    Ok(())
}

pub async fn run_agent_turn<F, Fut>(
    cwd: &Path,
    messages: &mut Vec<ChatMessage>,
    chat: &mut F,
    user_input: &str,
) -> Result<String>
where
    F: FnMut(Vec<ChatMessage>) -> Fut,
    Fut: Future<Output = Result<AssistantMessage>>,
{
    messages.push(ChatMessage::user(user_input.to_string()));

    loop {
        let assistant = chat(messages.clone()).await?;
        let content = assistant.content.clone();
        let tool_calls = assistant.tool_calls;

        messages.push(ChatMessage::assistant(content.clone(), tool_calls.clone()));

        if tool_calls.is_empty() {
            return Ok(content.unwrap_or_default());
        }

        for call in tool_calls {
            let result = match tools::execute(&call, cwd) {
                Ok(output) => output,
                Err(err) => format!("tool error: {err:#}"),
            };
            messages.push(ChatMessage::tool(call.id, result));
        }
    }
}

fn build_system_prompt(skill_prompts: &[String]) -> String {
    if skill_prompts.is_empty() {
        return BASE_PROMPT.to_string();
    }
    format!("{BASE_PROMPT}\n\n{}", skill_prompts.join("\n\n"))
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use anyhow::Result;
    use serde_json::json;

    use super::run_agent_turn;
    use bubble::llm::{AssistantMessage, ChatMessage};
    use bubble::tools::{ToolCall, ToolFunctionCall};

    #[tokio::test]
    async fn agent_loop_handles_tool_then_answer() -> Result<()> {
        let cwd = temp_dir("turn-once");
        fs::create_dir_all(&cwd)?;
        fs::write(cwd.join("Cargo.toml"), "name = \"bubble\"")?;

        let mut responses = VecDeque::from(vec![
            AssistantMessage {
                content: None,
                tool_calls: vec![call("read", json!({ "path": "Cargo.toml" }))],
            },
            AssistantMessage {
                content: Some(String::from("done")),
                tool_calls: vec![],
            },
        ]);

        let mut messages = vec![ChatMessage {
            role: String::from("system"),
            content: Some(String::from("base")),
            tool_call_id: None,
            tool_calls: None,
        }];

        let answer = run_agent_turn(
            &cwd,
            &mut messages,
            &mut |_| {
                let response = responses.pop_front().unwrap();
                async move { Ok(response) }
            },
            "read it",
        )
        .await?;

        assert_eq!(answer, "done");
        assert_eq!(messages.iter().filter(|m| m.role == "tool").count(), 1);
        fs::remove_dir_all(cwd)?;
        Ok(())
    }

    #[tokio::test]
    async fn agent_loop_handles_multiple_tool_rounds() -> Result<()> {
        let cwd = temp_dir("turn-multi");
        fs::create_dir_all(&cwd)?;
        let mut responses = VecDeque::from(vec![
            AssistantMessage {
                content: None,
                tool_calls: vec![call("bash", json!({ "command": "printf first" }))],
            },
            AssistantMessage {
                content: None,
                tool_calls: vec![call(
                    "write",
                    json!({ "path": "out.txt", "content": "second" }),
                )],
            },
            AssistantMessage {
                content: Some(String::from("finished")),
                tool_calls: vec![],
            },
        ]);

        let mut messages = vec![ChatMessage {
            role: String::from("system"),
            content: Some(String::from("base")),
            tool_call_id: None,
            tool_calls: None,
        }];

        let answer = run_agent_turn(
            &cwd,
            &mut messages,
            &mut |_| {
                let response = responses.pop_front().unwrap();
                async move { Ok(response) }
            },
            "do work",
        )
        .await?;

        assert_eq!(answer, "finished");
        assert_eq!(fs::read_to_string(cwd.join("out.txt"))?, "second");
        fs::remove_dir_all(cwd)?;
        Ok(())
    }

    fn call(name: &str, arguments: serde_json::Value) -> ToolCall {
        ToolCall {
            id: format!("call_{name}"),
            kind: String::from("function"),
            function: ToolFunctionCall {
                name: name.to_string(),
                arguments: arguments.to_string(),
            },
        }
    }

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("bubble-{name}-{nanos}"))
    }
}
