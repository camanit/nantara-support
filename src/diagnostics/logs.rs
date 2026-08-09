use serde::Serialize;
use std::fs;
use crate::utils::cmd::{run_powershell, CmdOutput};

#[derive(Debug, Serialize)]
pub struct BsodDumpInfo {
    pub file_name: String,
    pub created_at: String,
    pub size_bytes: u64,
}

#[derive(Debug, Serialize)]
pub struct SystemLogsResult {
    pub event_logs: CmdOutput,
    pub minidump_files: Vec<BsodDumpInfo>,
}

pub fn get_recent_system_logs() -> SystemLogsResult {
    let event_logs = run_powershell(
        "Get-EventLog -LogName System -Newest 10 -EntryType Error,Warning | Select-Object TimeGenerated, Source, EventID, Message | Format-Table -AutoSize"
    );

    let mut minidumps = Vec::new();
    let minidump_path = "C:\\Windows\\Minidump";

    if let Ok(entries) = fs::read_dir(minidump_path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                let name = entry.file_name().to_string_lossy().to_string();
                let created = metadata
                    .created()
                    .or_else(|_| metadata.modified())
                    .map(|t| format!("{:?}", t))
                    .unwrap_or_else(|_| "Unknown".into());

                minidumps.push(BsodDumpInfo {
                    file_name: name,
                    created_at: created,
                    size_bytes: metadata.len(),
                });
            }
        }
    }

    SystemLogsResult {
        event_logs,
        minidump_files: minidumps,
    }
}
