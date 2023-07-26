use std::collections::HashMap;

use rumbo_logic::prelude::JobClosure;
use rumbo_logic::prelude::JobInfo;
use rumbo_logic::prelude::JobScheduler;

pub mod prelude {
    pub use super::super::prelude::*;
}

pub struct TokioJobScheduler {
    map: HashMap<JobInfo, Box<dyn JobClosure>>,
}

impl TokioJobScheduler {
    pub fn new() -> Self {
        TokioJobScheduler {
            map: HashMap::new(),
        }
    }
}

impl JobScheduler for TokioJobScheduler {
    fn add_job(&mut self, info: rumbo_logic::prelude::JobInfo, func: Box<dyn JobClosure>) {
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
