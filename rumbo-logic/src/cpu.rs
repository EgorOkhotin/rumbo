use sysinfo::{CpuExt, CpuRefreshKind, RefreshKind, System, SystemExt};

pub struct CpuUsage {
    pub core: u8,
    pub load_percentage: u8,
}

fn get_cpu_usage() -> Vec<CpuUsage> {
    let mut system =
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL * 5);
    system.refresh_cpu();
    let mut i: u8 = 0;
    let cpu_usage = system
        .cpus()
        .into_iter()
        .map(|cpu| {
            let core = i;
            let load_percentage = cpu.cpu_usage() as u8;
            i += 1;
            CpuUsage {
                core,
                load_percentage,
            }
        })
        .collect::<Vec<_>>();

    cpu_usage
}