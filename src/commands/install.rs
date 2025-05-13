use crate::services::{local_installer, system_check};
use crate::models::install::ToolInstallRequest;
use dialoguer::{Input, Confirm};

pub async fn handle_install() -> anyhow::Result<()> {
    // Ask for tool name
    let tool_name: String = Input::new()
        .with_prompt("Tool to install")
        .interact_text()?;

    // Optional version input
    let version: String = Input::new()
        .with_prompt("Version (leave empty for latest)")
        .default("latest".to_string())
        .interact_text()?;

    // Detect system info
    let sys_info = system_check::detect_system()?;
    println!("[System Check] CPU: {}, RAM: {}MB, OS: {}",
        sys_info.cpu_model, sys_info.total_memory_mb, sys_info.os_name);

    // Confirm install
    let confirm = Confirm::new()
        .with_prompt(format!("Install {} {}?", tool_name, version))
        .interact()?;

    if confirm {
        let req = ToolInstallRequest {
            name: tool_name,
            version,
            system: sys_info,
        };

        local_installer::perform_install(req).await?;
    }

    Ok(())
}
