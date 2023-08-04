pub struct DaemonJobScheduler;

impl DaemonJobScheduler {
    pub fn new() -> Self {
        ActixJobScheduler
    }
}   

impl JobScheduler for DaemonJobScheduler {
    fn add_job(&mut self, info: JobInfo, func: Box<dyn JobClosure>) {
        tokio::spawn(async move {
            let duration = info.get_sleep_time();
            let mut info = info;
            loop {
                actix_web::rt::time::sleep(duration).await;
                func.invoke(&mut info).await;
            }
        });
    }
}
