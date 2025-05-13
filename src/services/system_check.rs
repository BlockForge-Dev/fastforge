use crate::models::install::SystemInfo;
use sysinfo::{System};

pub fn detect_system() -> anyhow::Result<SystemInfo> {
    let sys = System::new();

    let cpu_model = sys
        .cpus()
        .first()
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let total_memory_mb = sys.total_memory();

    let os_name = System::distribution_id(); // ← fixed here

    Ok(SystemInfo {
        cpu_model,
        total_memory_mb,
        os_name,
    })
}
