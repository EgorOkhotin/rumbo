pub use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Metric {
    instance_name: String,
    timestamp: u64,
    metric_value: MetricType
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "metric_type")]
pub enum MetricType {
    DiskUsage(DiskUsageInfo),
    DiskSpace(DiskSpaceInfo),

    RamSpace(RamSpaceInfo),

    NetworkUsage(NetworkUsageInfo),

    CpuUsage(CpuUsageInfo)
}

#[derive(Serialize, Deserialize)]
pub struct DiskUsageInfo {
    name: String,
    load_percents: u64,
    reading_speed: u64,
    writing_speed: u64,
}

#[derive(Serialize, Deserialize)]
pub struct DiskSpaceInfo {
    pub name: String,
    pub total_amount: u64,
    pub free_amount: u64
}

#[derive(Serialize, Deserialize)]
pub struct RamSpaceInfo {
    free_amount: u64,
    total_amount: u64
}

#[derive(Serialize, Deserialize)]
pub struct NetworkUsageInfo {
    sending_speed: u64,
    receiving_speed: u64
}

#[derive(Serialize, Deserialize)]
pub struct CpuUsageInfo {
    core: u8,
    load_percents: u8
}