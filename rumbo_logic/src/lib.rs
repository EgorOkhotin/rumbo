pub mod prelude {
    pub(super) use log::{info, warn};
    pub use std::sync::Arc;

    pub type Result<T> = std::result::Result<T, RumboError>;
    pub use super::db::*;
    pub use super::error::RumboError;
    pub use super::jobs::JobClosure;
    pub use super::RumboApp;

    pub use super::instances::prelude::*;
    pub use super::jobs::prelude::*;
    pub use super::metrics::prelude::*;
    pub use super::security::prelude::*;
    pub use super::users::prelude::*;
}
use prelude::*;

mod db;
mod error;
mod instances;
mod jobs;
mod metrics;
mod security;
mod users;

#[derive(Clone)]
pub struct RumboApp {
    // db_adapter: Arc<DbAdapter>,
    pub metrics_service: Arc<MetricsService>,
    pub instances_service: Arc<InstanceService>,
    pub jobs_storage_service: Arc<dyn JobStorageService>,
    pub users_service: Arc<UserService>,
}

impl RumboApp {
    pub async fn new<T: JobScheduler>(
        host: &str,
        app_name: &str,
        job_scheduler: &mut T,
        password_salter: Option<Arc<dyn PasswordSalter>>,
    ) -> Result<Self> {
        let adapter = DbAdapter::new(host, app_name).await?;
        let db_arc = Arc::from(adapter);

        let instances_service = InstanceService::new(&db_arc).as_arc();
        let metrics_service = MetricsService::new(&db_arc, &instances_service).as_arc();
        let jobs_storage_service = MongoJobStorageService::new(&db_arc).as_arc();

        // If password_salter not specified - create the default one.
        let password_salter: Arc<dyn PasswordSalter> = match password_salter {
            None => Arc::from(Argon2PasswordSalter),
            Some(val) => val,
        };

        let users_service = UserService::new(&db_arc, &password_salter).as_arc();

        let app = RumboApp {
            instances_service: instances_service,
            metrics_service: metrics_service,
            jobs_storage_service: jobs_storage_service,
            users_service: users_service,
        };

        add_jobs_to_schedule(job_scheduler).await;

        Ok(app)
    }
}

async fn add_jobs_to_schedule<T: JobScheduler>(_job_scheduler: &mut T) {
    // load info from DB
    // add jobs to scheduler
    // done

    // let job_info = JobInfo::new("TEST_JOB", Duration::from_secs(10));
    // let test_job_box = Box::new(TestJob);
    // job_scheduler.add_job(job_info, test_job_box);
}

struct TestJob;
#[async_trait]
impl JobClosure for TestJob {
    async fn invoke(&self, _job_info: &mut JobInfo) {
        warn!("It's a test job");
    }
}
