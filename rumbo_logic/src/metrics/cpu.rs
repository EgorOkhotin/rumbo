use super::prelude::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CpuUsageInfo {
    core: u8,
    load_percents: u8,
}

fn get_cpu_usage_info() -> Vec<CpuUsageInfo> {
    let mut system =
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
    system.refresh_cpu();
    let cpu_usage = system
        .cpus()
        .into_iter()
        .enumerate()
        .map(|cpu| {
            let core = cpu.0 as u8;
            let load_percents = cpu.1.cpu_usage() as u8;

            CpuUsageInfo {
                core,
                load_percents,
            }
        })
        .collect::<Vec<_>>();

    cpu_usage
}
