use sysinfo::{System, SystemExt};

pub struct Ram {
    pub free_amount: u64,
    pub total_amount: u64,
}

pub fn get_ram() -> Ram {
    let mut system = System::new();
    system.refresh_memory();

    let free_amount = (system.free_memory() as f64 / 1024.0f64.powi(3)).round() as u64;
    let total_amount = (system.total_memory() as f64 / 1024.0f64.powi(3)).round() as u64;

    Ram {
        free_amount,
        total_amount,
    }
}
