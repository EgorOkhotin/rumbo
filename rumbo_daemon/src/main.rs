mod config;
mod jobs;
use std::collections::BTreeMap;
use std::time::Duration;
use jobs::DaemonJobScheduler;

use crate::config::AppConfig;
use rumbo_logic::prelude::*;
use rumbo_logic::prelude::JobInfo;
use rumbo_logic::prelude::JobScheduler;





#[tokio::main]
async fn main() ->  std::io::Result<()>{

  let mut sheduler : DaemonJobScheduler = DaemonJobScheduler::new();
  


  Ok(())
}

async fn add_jobs_to_schedule<T: JobScheduler>(job_scheduler: &mut T) {
    // load info from DB
    // add jobs to scheduler
    // done

    let job_info = JobInfo::new("TEST_JOB", Duration::from_secs(10));
    let test_job_box = Box::new(TestJob);
    job_scheduler.add_job(job_info, test_job_box);
}

struct TestJob;
#[async_trait]
impl JobClosure for TestJob {
    async fn invoke(&self, _job_info: &mut JobInfo) {
        warn!("It's a test job");
    }
}
