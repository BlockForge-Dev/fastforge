use crate::models::install::ToolInstallRequest;
use std::{fs, thread, time::Duration};

pub async fn perform_install(req: ToolInstallRequest) -> anyhow::Result<()> {
    println!("\n[Installing] {}@{} on {}", req.name, req.version, req.system.os_name);

    // Simulated install delay
    thread::sleep(Duration::from_secs(2));

    // Write dummy file to /cache
    fs::write(format!("cache/{}_{}.installed", req.name, req.version), "installed")?;

    println!("[Done] {}@{} installed successfully!", req.name, req.version);
    Ok(())
}
