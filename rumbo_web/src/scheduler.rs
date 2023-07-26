pub mod prelude {
    pub(super) use rumbo_logic::prelude::JobClosure;
    pub(super) use rumbo_logic::prelude::JobInfo;
    pub(super) use rumbo_logic::prelude::JobScheduler;

    pub use super::super::prelude::*;

    pub use super::ActixJobScheduler;
}
use prelude::*;

pub struct ActixJobScheduler;

impl ActixJobScheduler {
    pub fn new() -> Self {
        ActixJobScheduler
    }
}

impl JobScheduler for ActixJobScheduler {
    fn add_job(&mut self, info: JobInfo, func: Box<dyn JobClosure>) {
        actix_web::rt::spawn(async move {
            let duration = info.get_sleep_time();
            let mut info = info;
            loop {
                actix_web::rt::time::sleep(duration).await;
                func.invoke(&mut info).await;
            }
        });
    }
}
