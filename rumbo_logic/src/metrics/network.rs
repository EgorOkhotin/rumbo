use super::prelude::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NetworkUsageInfo {
    name: String,
    sending_speed: u64,
    receiving_speed: u64,
}

pub fn get_network_usage() -> Vec<NetworkUsageInfo> {
    let mut system = System::new_all();
    system.refresh_networks();

    let networks_usage = system
        .networks()
        .into_iter()
        .map(|(interface_name, data)| {
            let name = interface_name.clone();
            let sending_speed = (data.total_received() as f64 / 1024.0f64.powi(2)).round() as u64;
            let receiving_speed =
                (data.total_transmitted() as f64 / 1024.0f64.powi(2)).round() as u64;
                NetworkUsageInfo {
                name,
                sending_speed,
                receiving_speed,
            }
        })
        .collect::<Vec<_>>();

    networks_usage
}