use serde::Serialize;
use sysinfo::{Disks, System};
use crate::utils::cmd::{run_cmd, CmdOutput};

#[derive(Debug, Serialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_gb: f64,
    pub available_gb: f64,
}

#[derive(Debug, Serialize)]
pub struct SystemMetrics {
    pub os_name: String,
    pub os_version: String,
    pub uptime_seconds: u64,
    pub cpu_count: usize,
    pub cpu_global_usage: f32,
    pub total_memory_mb: u64,
    pub used_memory_mb: u64,
    pub memory_used_percent: f32,
    pub disks: Vec<DiskInfo>,
}

pub fn get_system_metrics() -> SystemMetrics {
    let mut sys = System::new_all();
    sys.refresh_all();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_usage();

    let total_mem = sys.total_memory() / 1024 / 1024;
    let used_mem = sys.used_memory() / 1024 / 1024;
    let mem_percent = if total_mem > 0 {
        (used_mem as f32 / total_mem as f32) * 100.0
    } else {
        0.0
    };

    let disks_list = Disks::new_with_refreshed_list();
    let mut disks_info = Vec::new();

    for disk in &disks_list {
        disks_info.push(DiskInfo {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            total_gb: (disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0 * 100.0).round() / 100.0,
            available_gb: (disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0 * 100.0).round() / 100.0,
        });
    }

    SystemMetrics {
        os_name: System::name().unwrap_or_else(|| "Windows".into()),
        os_version: System::os_version().unwrap_or_else(|| "Unknown".into()),
        uptime_seconds: System::uptime(),
        cpu_count: sys.cpus().len(),
        cpu_global_usage: (sys.global_cpu_info().cpu_usage() * 10.0).round() / 10.0,
        total_memory_mb: total_mem,
        used_memory_mb: used_mem,
        memory_used_percent: (mem_percent * 10.0).round() / 10.0,
        disks: disks_info,
    }
}

pub fn run_sfc_scan() -> CmdOutput {
    run_cmd("sfc /scannow")
}

pub fn run_dism_repair() -> CmdOutput {
    run_cmd("DISM /Online /Cleanup-Image /RestoreHealth")
}

pub fn run_gpupdate() -> CmdOutput {
    run_cmd("gpupdate /force")
}
