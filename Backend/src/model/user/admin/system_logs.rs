use crate::common::BoxError;
use crate::model::project::contract::json::Json;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

pub struct SystemLogs {
    dir: String,
    level_filter: Option<String>,
    query: Option<String>,
    limit: usize,
}

impl SystemLogs {
    pub fn new(
        dir: String,
        level_filter: Option<String>,
        query: Option<String>,
        limit: usize,
    ) -> Self {
        Self {
            dir,
            level_filter,
            query,
            limit,
        }
    }

    fn log_files(&self) -> Vec<PathBuf> {
        let path = Path::new(&self.dir);
        if !path.exists() || !path.is_dir() {
            return Vec::new();
        }
        let Ok(entries) = fs::read_dir(path) else {
            return Vec::new();
        };
        let mut files: Vec<PathBuf> = entries
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("log"))
            .collect();
        files.sort_by(|a, b| b.cmp(a));
        files
    }

    fn parse_line(&self, line: &str) -> Option<LogEntry> {
        if line.trim().is_empty() {
            return None;
        }
        let parsed: serde_json::Value = serde_json::from_str(line).unwrap_or_else(|_| {
            serde_json::json!({
                "level": "INFO",
                "fields": { "message": line }
            })
        });
        let level = parsed
            .get("level")
            .and_then(|v| v.as_str())
            .unwrap_or("INFO")
            .to_uppercase();
        if let Some(ref filter) = self.level_filter {
            if !filter.is_empty() && filter.to_uppercase() != "ALL" && level != filter.to_uppercase() {
                return None;
            }
        }
        let message = parsed
            .get("fields")
            .and_then(|f| f.get("message"))
            .and_then(|m| m.as_str())
            .unwrap_or(line)
            .to_string();
        if let Some(ref q) = self.query {
            if !q.is_empty() {
                let q_lower = q.to_lowercase();
                if !message.to_lowercase().contains(&q_lower) && !line.to_lowercase().contains(&q_lower) {
                    return None;
                }
            }
        }
        let timestamp = parsed.get("timestamp").and_then(|v| v.as_str()).map(String::from);
        let target = parsed.get("target").and_then(|v| v.as_str()).map(String::from);
        Some(LogEntry {
            timestamp,
            level,
            target,
            message,
            raw: parsed,
        })
    }
}

#[derive(Serialize)]
struct LogEntry {
    timestamp: Option<String>,
    level: String,
    target: Option<String>,
    message: String,
    raw: serde_json::Value,
}

#[async_trait::async_trait]
impl Json for SystemLogs {
    async fn json(&self) -> Result<serde_json::Value, BoxError> {
        let files = self.log_files();
        let file_names: Vec<String> = files
            .iter()
            .filter_map(|p| p.file_name().and_then(|n| n.to_str()).map(String::from))
            .collect();
        let mut logs = Vec::new();
        for file in &files {
            if logs.len() >= self.limit {
                break;
            }
            if let Ok(content) = fs::read_to_string(file) {
                for line in content.lines().rev() {
                    if logs.len() >= self.limit {
                        break;
                    }
                    if let Some(entry) = self.parse_line(line) {
                        logs.push(entry);
                    }
                }
            }
        }
        Ok(serde_json::json!({
            "files": file_names,
            "logs": logs,
        }))
    }
}
