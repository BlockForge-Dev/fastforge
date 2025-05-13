use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub cpu_model: String,
    pub total_memory_mb: u64,
    pub os_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolInstallRequest {
    pub name: String,
    pub version: String,
    pub system: SystemInfo,
}
