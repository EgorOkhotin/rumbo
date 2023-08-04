pub struct DaemonJobScheduler;
use rumbo_logic::prelude::JobClosure;
use rumbo_logic::prelude::JobInfo;
use rumbo_logic::prelude::JobScheduler;

impl DaemonJobScheduler {
    pub fn new() -> Self {
        Self
    }
}   

impl JobScheduler for DaemonJobScheduler {
    fn add_job(&mut self, info: JobInfo, func: Box<dyn JobClosure>) {
        // tokio::spawn(async move {
        //     let duration = info.get_sleep_time();
        //     let mut info = info;
        //     loop {
        //         tokio::time::sleep(duration).await;
        //         func.invoke(&mut info).await;
        //     }
        // });
    }
}
