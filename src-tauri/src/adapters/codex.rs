use super::{
    apply_extended_path, apply_shell_env, command_for_cli, hide_console_window, resolve_cli_path,
    CliAdapter, CommandOptions, LineType, ParsedLine,
};
use serde_json::Value;
use crate::storage::models::CliType;
use async_trait::async_trait;
use std::path::Path;
use std::process::Stdio;
use tokio::process::Command;

pub struct CodexAdapter {
    path: Option<String>,
}

impl CodexAdapter {
    pub fn new() -> Self {
        let path = resolve_cli_path("codex");
        Self { path }
    }

    fn exec_args(prompt: &str, options: CommandOptions) -> Vec<String> {
        let mut args = vec![
            "exec".to_string(),
            "--dangerously-bypass-approvals-and-sandbox".to_string(),
        ];
        if options.skip_git_repo_check {
            args.push("--skip-git-repo-check".to_string());
        }
        args.push(prompt.to_string());
        args
    }

    fn readonly_args(prompt: &str, options: CommandOptions) -> Vec<String> {
        let mut args = vec![
            "exec".to_string(),
            "--dangerously-bypass-approvals-and-sandbox".to_string(),
            "--json".to_string(),  // Output JSONL for parsing
        ];
        if options.skip_git_repo_check {
            args.push("--skip-git-repo-check".to_string());
        }
        args.push(prompt.to_string());
        args
    }

    fn build_exec_command(
        &self,
        prompt: &str,
        working_dir: &Path,
        readonly: bool,
        options: CommandOptions,
    ) -> Command {
        let exe = self.path.as_deref().unwrap_or("codex");
        let args = if readonly {
            Self::readonly_args(prompt, options)
        } else {
            Self::exec_args(prompt, options)
        };
        let mut cmd = command_for_cli(exe, &args, working_dir);
        apply_extended_path(&mut cmd);
        apply_shell_env(&mut cmd);
        cmd.stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        cmd
    }
}

#[async_trait]
impl CliAdapter for CodexAdapter {
    fn name(&self) -> &str {
        "Codex CLI"
    }

    fn cli_type(&self) -> CliType {
        CliType::Codex
    }

    fn is_installed(&self) -> bool {
        self.path.is_some()
    }

    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    async fn version(&self) -> Option<String> {
        let exe = self.path.as_deref().unwrap_or("codex");
        let mut cmd = Command::new(exe);
        apply_extended_path(&mut cmd);
        apply_shell_env(&mut cmd);
        hide_console_window(&mut cmd);
        let output = cmd.arg("--version").output().await.ok()?;

        if output.status.success() {
            Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            None
        }
    }

    fn build_command(&self, prompt: &str, working_dir: &Path, options: CommandOptions) -> Command {
        self.build_exec_command(prompt, working_dir, false, options)
    }

    fn build_readonly_command(
        &self,
        prompt: &str,
        working_dir: &Path,
        options: CommandOptions,
    ) -> Command {
        self.build_exec_command(prompt, working_dir, true, options)
    }

    fn detect_completion(&self, output: &str, signal: &str) -> bool {
        // Codex output is plain text, direct detection
        output.contains(signal)
    }

    fn parse_output_line(&self, line: &str) -> ParsedLine {
        // Codex --json outputs JSONL, parse item.completed events for agent_message text
        if let Ok(json) = serde_json::from_str::<Value>(line) {
            let event_type = json.get("type").and_then(|t| t.as_str()).unwrap_or("");
            
            match event_type {
                "item.completed" => {
                    // Extract text from agent_message items
                    if let Some(item) = json.get("item") {
                        let item_type = item.get("type").and_then(|t| t.as_str()).unwrap_or("");
                        if item_type == "agent_message" {
                            if let Some(text) = item.get("text").and_then(|t| t.as_str()) {
                                return ParsedLine {
                                    content: text.to_string(),
                                    line_type: LineType::Json,
                                    is_assistant: true,
                                };
                            }
                        }
                    }
                    // Non-message item.completed, skip
                    ParsedLine {
                        content: String::new(),
                        line_type: LineType::Json,
                        is_assistant: false,
                    }
                }
                "thread.started" | "turn.started" | "turn.completed" | "item.delta" => {
                    // Control events, skip
                    ParsedLine {
                        content: String::new(),
                        line_type: LineType::Json,
                        is_assistant: false,
                    }
                }
                _ => {
                    // No type field or unknown type - pass through as text
                    // This handles: direct brainstorm JSON, unknown events, and Loop mode output
                    // that happens to look like JSON (safer fallback to avoid losing content)
                    ParsedLine {
                        content: line.to_string(),
                        line_type: LineType::Text,
                        is_assistant: true,
                    }
                }
            }
        } else {
            // Not JSON, treat as plain text (fallback for non --json mode)
            ParsedLine {
                content: line.to_string(),
                line_type: LineType::Text,
                is_assistant: true,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CodexAdapter;
    use super::CommandOptions;

    #[test]
    fn exec_args_include_exec_and_full_auto() {
        let args = CodexAdapter::exec_args("hello", CommandOptions::default());
        assert_eq!(
            args,
            vec!["exec", "--dangerously-bypass-approvals-and-sandbox", "hello"]
        );
    }

    #[test]
    fn readonly_args_use_read_only_sandbox() {
        let args = CodexAdapter::readonly_args("hello", CommandOptions::default());
        assert_eq!(
            args,
            vec!["exec", "--dangerously-bypass-approvals-and-sandbox", "--json", "hello"]
        );
    }

    #[test]
    fn exec_args_include_skip_git_repo_check() {
        let args = CodexAdapter::exec_args(
            "hello",
            CommandOptions {
                skip_git_repo_check: true,
            },
        );
        assert_eq!(
            args,
            vec![
                "exec",
                "--dangerously-bypass-approvals-and-sandbox",
                "--skip-git-repo-check",
                "hello"
            ]
        );
    }

    #[test]
    fn readonly_args_include_skip_git_repo_check() {
        let args = CodexAdapter::readonly_args(
            "hello",
            CommandOptions {
                skip_git_repo_check: true,
            },
        );
        assert_eq!(
            args,
            vec![
                "exec",
                "--dangerously-bypass-approvals-and-sandbox",
                "--json",
                "--skip-git-repo-check",
                "hello"
            ]
        );
    }

    #[test]
    fn parse_output_line_extracts_agent_message_text() {
        let adapter = CodexAdapter::new();
        let line = r#"{"type":"item.completed","item":{"type":"agent_message","text":"Hello world"}}"#;
        let parsed = adapter.parse_output_line(line);
        assert_eq!(parsed.content, "Hello world");
        assert_eq!(parsed.line_type, super::LineType::Json);
        assert!(parsed.is_assistant);
    }

    #[test]
    fn parse_output_line_skips_control_events() {
        let adapter = CodexAdapter::new();
        for event in ["thread.started", "turn.started", "turn.completed", "item.delta"] {
            let line = format!(r#"{{"type":"{}"}}"#, event);
            let parsed = adapter.parse_output_line(&line);
            assert!(parsed.content.is_empty(), "should skip {}", event);
        }
    }

    #[test]
    fn parse_output_line_passes_through_unknown_json() {
        let adapter = CodexAdapter::new();
        let line = r#"{"question":"Hi","options":[]}"#;
        let parsed = adapter.parse_output_line(line);
        assert_eq!(parsed.content, line);
        assert_eq!(parsed.line_type, super::LineType::Text);
    }

    #[test]
    fn parse_output_line_handles_plain_text() {
        let adapter = CodexAdapter::new();
        let line = "Hello world";
        let parsed = adapter.parse_output_line(line);
        assert_eq!(parsed.content, "Hello world");
        assert_eq!(parsed.line_type, super::LineType::Text);
    }

    #[test]
    fn parse_output_line_preserves_completion_signal() {
        let adapter = CodexAdapter::new();
        let line = "<done>COMPLETE</done>";
        let parsed = adapter.parse_output_line(line);
        assert_eq!(parsed.content, "<done>COMPLETE</done>");
        assert!(parsed.is_assistant);
    }
}
