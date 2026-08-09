use crate::utils::cmd::{run_powershell, CmdOutput};

pub fn fix_printer_spooler() -> CmdOutput {
    run_powershell("Stop-Service -Name Spooler -Force; Remove-Item -Path '$env:windir\\System32\\spool\\PRINTERS\\*' -Force -Recurse -ErrorAction SilentlyContinue; Start-Service -Name Spooler")
}
