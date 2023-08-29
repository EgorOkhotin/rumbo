pub mod prelude {
    pub use chrono::Utc;
    pub(super) use diesel::prelude::*;
    pub(super) use log::{info, warn};
    pub use std::sync::Arc;

    pub type Result<T> = std::result::Result<T, RumboError>;
    pub use super::db::prelude::*;
    pub use super::error::RumboError;
    pub use super::jobs::JobClosure;
    pub use super::RumboApp;

    pub(super) use super::instances::InstanceService;

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

mod schema;

#[derive(Clone)]
pub struct RumboApp {
    pub metrics_service: Arc<MetricsService>,
    pub instances_service: Arc<InstanceService>,
    pub jobs_storage_service: Arc<dyn JobStorageService>,
    pub users_service: Arc<UserService>,
}

impl RumboApp {
    pub async fn new<T: JobScheduler>(
        db_conection_string: &str,
        job_scheduler: &mut T,
        password_salter: Option<Arc<dyn PasswordSalter>>,
    ) -> Result<Self> {
        info!(
            "Trying to connect to db with connection string: {}",
            db_conection_string
        );
        let adapter = DbAdapter::new(db_conection_string).await?;
        info!("DB connection established");

        let db_arc = Arc::from(adapter);

        let instances_service = InstanceService::new(&db_arc).as_arc();
        info!("Instances service created");

        let metrics_service = MetricsService::new(&db_arc).as_arc();
        info!("Metrics service created");

        let jobs_storage_service = PostgresJobStorageService::new(&db_arc).as_arc();
        info!("Jobs Service created");

        // If password_salter not specified - create the default one.
        let password_salter: Arc<dyn PasswordSalter> = match password_salter {
            None => Arc::from(Argon2PasswordSalter),
            Some(val) => val,
        };
        info!("Password salter created");

        let users_service = UserService::new(&db_arc, &password_salter).as_arc();
        info!("Users service created");

        let app = RumboApp {
            instances_service: instances_service,
            metrics_service: metrics_service,
            jobs_storage_service: jobs_storage_service,
            users_service: users_service,
        };
        info!("App state created");

        add_jobs_to_schedule(job_scheduler).await;
        info!("Jobs added to schedule");

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
