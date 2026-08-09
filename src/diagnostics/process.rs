use serde::Serialize;
use sysinfo::System;
use crate::utils::cmd::{run_cmd, CmdOutput};

#[derive(Debug, Serialize)]
pub struct ProcessItem {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_mb: u64,
}

pub fn get_top_processes(limit: usize) -> Vec<ProcessItem> {
    let mut sys = System::new_all();
    sys.refresh_all();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_all();

    let mut list: Vec<ProcessItem> = sys
        .processes()
        .iter()
        .map(|(pid, proc_info)| ProcessItem {
            pid: pid.as_u32(),
            name: proc_info.name().to_string(),
            cpu_usage: (proc_info.cpu_usage() * 10.0).round() / 10.0,
            memory_mb: proc_info.memory() / 1024 / 1024,
        })
        .collect();

    // Sort by memory usage descending
    list.sort_by(|a, b| b.memory_mb.cmp(&a.memory_mb));
    list.truncate(limit);
    list
}

pub fn kill_process(pid: u32) -> CmdOutput {
    run_cmd(&format!("taskkill /F /PID {}", pid))
}
