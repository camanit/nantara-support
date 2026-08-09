use serde::Serialize;
use std::process::Command;

#[derive(Debug, Serialize, Clone)]
pub struct CmdOutput {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

/// Menjalankan perintah di PowerShell secara aman
pub fn run_powershell(command: &str) -> CmdOutput {
    tracing::info!("Running PowerShell command: {}", command);
    match Command::new("powershell")
        .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", command])
        .output()
    {
        Ok(output) => CmdOutput {
            success: output.status.success(),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).trim().to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        },
        Err(err) => CmdOutput {
            success: false,
            exit_code: None,
            stdout: String::new(),
            stderr: err.to_string(),
        },
    }
}

/// Menjalankan perintah di Windows CMD secara aman
pub fn run_cmd(command: &str) -> CmdOutput {
    tracing::info!("Running CMD command: {}", command);
    match Command::new("cmd")
        .args(["/C", command])
        .output()
    {
        Ok(output) => CmdOutput {
            success: output.status.success(),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).trim().to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        },
        Err(err) => CmdOutput {
            success: false,
            exit_code: None,
            stdout: String::new(),
            stderr: err.to_string(),
        },
    }
}
