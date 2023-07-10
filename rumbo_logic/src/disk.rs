use sysinfo::{DiskExt, System, SystemExt};

pub struct DiskSpace {
    pub name: String,
    pub free_amount: u64,
    pub total_amount: u64,
}

pub fn get_disk_spaces() -> Vec<DiskSpace> {
    let mut system = System::new();
    system.refresh_disks_list();

    let disk_spaces = system
        .disks()
        .into_iter()
        .map(|disk| {
            let total_amount = (disk.total_space() as f64 / 1024.0f64.powi(3)).round() as u64;
            let free_amount = (disk.available_space() as f64 / 1024.0f64.powi(3)).round() as u64;
            let name = disk.mount_point().display().to_string();

            DiskSpace {
                name,
                free_amount,
                total_amount,
            }
        })
        .collect::<Vec<_>>();

    disk_spaces
}
