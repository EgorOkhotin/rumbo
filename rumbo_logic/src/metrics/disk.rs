use super::prelude::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DiskSpaceInfo {
    pub name: String,
    pub total_amount: u64,
    pub free_amount: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DiskUsageInfo {
    name: String,
    load_percents: u64,
    reading_speed: u64,
    writing_speed: u64,
}

pub fn get_disk_spaces() -> Vec<DiskSpaceInfo> {
    let mut system = System::new();
    system.refresh_disks_list();

    let disk_spaces = system
        .disks()
        .into_iter()
        .map(|disk| {
            let total_amount = (disk.total_space() as f64 / 1024.0f64.powi(3)).round() as u64;
            let free_amount = (disk.available_space() as f64 / 1024.0f64.powi(3)).round() as u64;
            let name = disk.mount_point().display().to_string();

            DiskSpaceInfo {
                name,
                free_amount,
                total_amount,
            }
        })
        .collect::<Vec<_>>();

    disk_spaces
}