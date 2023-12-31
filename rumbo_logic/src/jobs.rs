pub mod prelude {
    pub use super::super::prelude::*;
    pub(super) use chrono::DateTime;
    pub(super) use serde::{Deserialize, Serialize};

    pub use async_trait::async_trait;
    pub use std::future::Future;
    pub use std::time::Duration;

    pub use super::postgres::PostgresJobStorageService;
    pub use super::JobClosure;
    pub use super::JobInfo;
    pub use super::JobScheduler;
    pub use super::JobStorageService;
}
use prelude::*;
mod postgres;

#[async_trait]
pub trait JobStorageService: Send + Sync {
    async fn save(&self, info: JobInfo) -> Result<JobInfo>;
    async fn load(&self, name: &str) -> Result<Option<JobInfo>>;
}

pub trait JobScheduler {
    fn add_job(&mut self, info: JobInfo, func: Box<dyn JobClosure>);
}

#[async_trait]
pub trait JobClosure {
    async fn invoke(&self, info: &mut JobInfo) -> ();
}

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq)]
pub struct JobInfo {
    name: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    last_invocation: DateTime<Utc>,
    sleep_time: Duration,
}

impl JobInfo {
    pub fn new(name: &str, sleep_time: Duration) -> Self {
        JobInfo {
            name: name.to_string(),
            last_invocation: DateTime::<Utc>::MIN_UTC,
            sleep_time: sleep_time,
        }
    }

    pub fn get_sleep_time(&self) -> Duration {
        self.sleep_time
    }

    pub fn update_last_invocation_time(&mut self, last_invocation: DateTime<Utc>) {
        self.last_invocation = last_invocation;
    }
}
