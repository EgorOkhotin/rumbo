use super::prelude::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CpuUsageInfo {
    core: u8,
    load_percents: u8,
}