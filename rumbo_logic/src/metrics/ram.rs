pub mod prelude {
    pub use super::get_ram;
    pub use super::RamSpaceInfo;

    pub(super) use super::super::prelude::*;
}
use prelude::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RamSpaceInfo {
    free_amount: u64,
    total_amount: u64,
}

pub fn get_ram() -> RamSpaceInfo {
    let mut system = System::new();
    system.refresh_memory();

    let free_amount = (system.free_memory() as f64 / 1024.0f64.powi(3)).round() as u64;
    let total_amount = (system.total_memory() as f64 / 1024.0f64.powi(3)).round() as u64;

    RamSpaceInfo {
        free_amount,
        total_amount,
    }
}
