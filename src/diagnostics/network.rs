use serde::Serialize;
use crate::utils::cmd::{run_cmd, run_powershell, CmdOutput};

#[derive(Debug, Serialize)]
pub struct NetworkDiagnosticResult {
    pub ping_gateway: CmdOutput,
    pub ping_google: CmdOutput,
    pub dns_lookup: CmdOutput,
    pub ipconfig_info: CmdOutput,
}

pub fn run_network_check() -> NetworkDiagnosticResult {
    let ping_gateway = run_cmd("ping -n 2 192.168.1.1");
    let ping_google = run_cmd("ping -n 2 8.8.8.8");
    let dns_lookup = run_cmd("nslookup google.com");
    let ipconfig_info = run_cmd("ipconfig /all");

    NetworkDiagnosticResult {
        ping_gateway,
        ping_google,
        dns_lookup,
        ipconfig_info,
    }
}

pub fn fix_network() -> CmdOutput {
    run_powershell("ipconfig /flushdns; Clear-DnsClientCache; ipconfig /release; ipconfig /renew")
}
