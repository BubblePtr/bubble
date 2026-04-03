use std::fs;
use std::path::{Component, Path};
use std::process::Command;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

const MAX_READ_BYTES: usize = 64 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub function: ToolFunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolFunctionCall {
    pub name: String,
    pub arguments: String,
}

pub fn definitions() -> Vec<Value> {
    vec![
        function_tool(
            "bash",
            "Run a shell command in the current working directory. This tool is not sandboxed.",
            json!({
                "type": "object",
                "properties": {
                    "command": { "type": "string" }
                },
                "required": ["command"]
            }),
        ),
        function_tool(
            "read",
            "Read a text file.",
            json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" }
                },
                "required": ["path"]
            }),
        ),
        function_tool(
            "write",
            "Write a text file, overwriting existing content.",
            json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" },
                    "content": { "type": "string" }
                },
                "required": ["path", "content"]
            }),
        ),
        function_tool(
            "edit",
            "Replace one exact text block in a file.",
            json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" },
                    "old": { "type": "string" },
                    "new": { "type": "string" }
                },
                "required": ["path", "old", "new"]
            }),
        ),
    ]
}

pub fn execute(call: &ToolCall, cwd: &Path) -> Result<String> {
    match call.function.name.as_str() {
        "bash" => {
            let input: BashInput = serde_json::from_str(&call.function.arguments)
                .with_context(|| format!("invalid arguments for {}", call.function.name))?;
            let output = Command::new("sh")
                .arg("-lc")
                .arg(&input.command)
                .current_dir(cwd)
                .output()
                .with_context(|| format!("failed to run command: {}", input.command))?;
            Ok(json!({
                "stdout": String::from_utf8_lossy(&output.stdout),
                "stderr": String::from_utf8_lossy(&output.stderr),
                "exit_code": output.status.code().unwrap_or(-1),
            })
            .to_string())
        }
        "read" => {
            let input: ReadInput = serde_json::from_str(&call.function.arguments)
                .with_context(|| format!("invalid arguments for {}", call.function.name))?;
            let path = resolve_path(cwd, &input.path)?;
            let bytes =
                fs::read(&path).with_context(|| format!("failed to read {}", path.display()))?;
            let truncated = bytes.len() > MAX_READ_BYTES;
            let bytes = if truncated {
                &bytes[..MAX_READ_BYTES]
            } else {
                &bytes[..]
            };
            Ok(json!({
                "path": path.display().to_string(),
                "content": String::from_utf8_lossy(bytes),
                "truncated": truncated,
            })
            .to_string())
        }
        "write" => {
            let input: WriteInput = serde_json::from_str(&call.function.arguments)
                .with_context(|| format!("invalid arguments for {}", call.function.name))?;
            let path = resolve_path(cwd, &input.path)?;
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("failed to create {}", parent.display()))?;
            }
            fs::write(&path, input.content)
                .with_context(|| format!("failed to write {}", path.display()))?;
            Ok(json!({ "ok": true, "path": path.display().to_string() }).to_string())
        }
        "edit" => {
            let input: EditInput = serde_json::from_str(&call.function.arguments)
                .with_context(|| format!("invalid arguments for {}", call.function.name))?;
            if input.old.is_empty() {
                bail!("edit.old cannot be empty");
            }
            let path = resolve_path(cwd, &input.path)?;
            let content = fs::read_to_string(&path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            let matches = content.match_indices(&input.old).count();
            match matches {
                0 => bail!("edit target not found in {}", path.display()),
                1 => {
                    let updated = content.replacen(&input.old, &input.new, 1);
                    fs::write(&path, updated)
                        .with_context(|| format!("failed to write {}", path.display()))?;
                    Ok(json!({ "ok": true, "replacements": 1, "path": path.display().to_string() }).to_string())
                }
                count => bail!("edit target matched {count} times in {}", path.display()),
            }
        }
        other => bail!("unknown tool: {other}"),
    }
}

fn function_tool(name: &str, description: &str, parameters: Value) -> Value {
    json!({
        "type": "function",
        "function": {
            "name": name,
            "description": description,
            "parameters": parameters
        }
    })
}

fn resolve_path(cwd: &Path, path: &str) -> Result<std::path::PathBuf> {
    let path = Path::new(path);
    if path.as_os_str().is_empty() {
        bail!("path cannot be empty");
    }
    if path.is_absolute() {
        bail!("absolute paths are not allowed: {}", path.display());
    }
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir | Component::Prefix(_)))
    {
        bail!("parent directory traversal is not allowed: {}", path.display());
    }
    Ok(cwd.join(path))
}

#[derive(Debug, Deserialize)]
struct BashInput {
    command: String,
}

#[derive(Debug, Deserialize)]
struct ReadInput {
    path: String,
}

#[derive(Debug, Deserialize)]
struct WriteInput {
    path: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct EditInput {
    path: String,
    old: String,
    new: String,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use serde_json::json;

    use super::{ToolCall, ToolFunctionCall, execute};

    #[test]
    fn bash_returns_stdout_stderr_and_exit_code() {
        let cwd = temp_dir("bash");
        fs::create_dir_all(&cwd).unwrap();
        let result = execute(
            &call(
                "bash",
                json!({ "command": "printf hi; printf err >&2; exit 7" }),
            ),
            &cwd,
        )
        .unwrap();
        assert!(result.contains("\"stdout\":\"hi\""));
        assert!(result.contains("\"stderr\":\"err\""));
        assert!(result.contains("\"exit_code\":7"));
        fs::remove_dir_all(cwd).unwrap();
    }

    #[test]
    fn read_handles_truncation() {
        let cwd = temp_dir("read");
        fs::create_dir_all(&cwd).unwrap();
        let path = cwd.join("big.txt");
        fs::write(&path, "a".repeat(70 * 1024)).unwrap();
        let result = execute(&call("read", json!({ "path": "big.txt" })), &cwd).unwrap();
        assert!(result.contains("\"truncated\":true"));
        fs::remove_dir_all(cwd).unwrap();
    }

    #[test]
    fn write_creates_parent_directories() {
        let cwd = temp_dir("write");
        fs::create_dir_all(&cwd).unwrap();
        execute(
            &call(
                "write",
                json!({ "path": "nested/file.txt", "content": "hello" }),
            ),
            &cwd,
        )
        .unwrap();
        assert_eq!(
            fs::read_to_string(cwd.join("nested/file.txt")).unwrap(),
            "hello"
        );
        fs::remove_dir_all(cwd).unwrap();
    }

    #[test]
    fn read_rejects_parent_traversal() {
        let cwd = temp_dir("read-reject-parent");
        fs::create_dir_all(&cwd).unwrap();
        let err = execute(&call("read", json!({ "path": "../secret.txt" })), &cwd).unwrap_err();
        assert!(
            err.to_string()
                .contains("parent directory traversal is not allowed")
        );
        fs::remove_dir_all(cwd).unwrap();
    }

    #[test]
    fn write_rejects_absolute_paths() {
        let cwd = temp_dir("write-reject-absolute");
        fs::create_dir_all(&cwd).unwrap();
        let absolute = cwd.join("absolute.txt");
        let err = execute(
            &call(
                "write",
                json!({
                    "path": absolute.display().to_string(),
                    "content": "hello"
                }),
            ),
            &cwd,
        )
        .unwrap_err();
        assert!(err.to_string().contains("absolute paths are not allowed"));
        fs::remove_dir_all(cwd).unwrap();
    }

    #[test]
    fn edit_requires_exactly_one_match() {
        let cwd = temp_dir("edit");
        fs::create_dir_all(&cwd).unwrap();
        let path = cwd.join("file.txt");
        fs::write(&path, "alpha beta").unwrap();
        execute(
            &call(
                "edit",
                json!({ "path": "file.txt", "old": "beta", "new": "gamma" }),
            ),
            &cwd,
        )
        .unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "alpha gamma");

        let err = execute(
            &call(
                "edit",
                json!({ "path": "file.txt", "old": "missing", "new": "x" }),
            ),
            &cwd,
        )
        .unwrap_err();
        assert!(err.to_string().contains("not found"));

        fs::write(&path, "dup dup").unwrap();
        let err = execute(
            &call(
                "edit",
                json!({ "path": "file.txt", "old": "dup", "new": "x" }),
            ),
            &cwd,
        )
        .unwrap_err();
        assert!(err.to_string().contains("matched 2 times"));
        fs::remove_dir_all(cwd).unwrap();
    }

    fn call(name: &str, arguments: serde_json::Value) -> ToolCall {
        ToolCall {
            id: String::from("call_1"),
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
