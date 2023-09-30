use diesel::data_types::PgInterval;

use super::prelude::*;

pub struct PostgresJobStorageService {
    db_adapter: Arc<DbAdapter>,
}

impl PostgresJobStorageService {
    pub fn new(db: &Arc<DbAdapter>) -> Self {
        PostgresJobStorageService {
            db_adapter: db.clone(),
        }
    }

    pub fn as_arc(self) -> Arc<Self> {
        Arc::from(self)
    }

    async fn update_job(&self, info: JobInfo) -> Result<JobInfo> {
        use crate::schema::jobs::dsl::*;

        let value = JobSqlRow::from(info);
        let job_name = value.name.clone();

        let mut connection = self.db_adapter.get_connection()?;
        let result = diesel::update(jobs.find(job_name))
            .set(value)
            .returning(JobSqlRow::as_returning())
            .get_result(&mut connection)?;

        Ok(JobInfo::from(result))
    }

    async fn add_job(&self, info: JobInfo) -> Result<JobInfo> {
        use crate::schema::jobs;

        let job = JobSqlRow::from(info);

        let mut connection = self.db_adapter.get_connection()?;
        let result = diesel::insert_into(jobs::table)
            .values(job)
            .returning(JobSqlRow::as_returning())
            .get_result(&mut connection)?;

        let result = JobInfo::from(result);
        Ok(result)
    }
}

#[async_trait]
impl JobStorageService for PostgresJobStorageService {
    async fn save(&self, info: JobInfo) -> Result<JobInfo> {
        let stored_job = self.load(&info.name).await?;

        let job_info = match stored_job {
            None => self.add_job(info).await?,
            Some(_) => self.update_job(info).await?,
        };

        Ok(job_info)
    }

    async fn load(&self, job_name: &str) -> Result<Option<JobInfo>> {
        use crate::schema::jobs::dsl::*;

        let mut connection = self.db_adapter.get_connection()?;
        let result: Option<JobSqlRow> = jobs
            .find(job_name)
            .first::<JobSqlRow>(&mut connection)
            .optional()?;

        let result = match result {
            Some(value) => Some(JobInfo::from(value)),
            None => None,
        };

        Ok(result)
    }
}

#[derive(Insertable, Queryable, Selectable, AsChangeset)]
#[diesel(primary_key(name))]
#[diesel(table_name = crate::schema::jobs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct JobSqlRow {
    name: String,
    last_invocation: chrono::NaiveDateTime,
    sleep_time: PgInterval,
}

impl From<JobInfo> for JobSqlRow {
    fn from(value: JobInfo) -> Self {
        warn!("Make safe conversion to the PgInterval");
        JobSqlRow {
            name: value.name,
            last_invocation: value.last_invocation.naive_utc(),
            sleep_time: PgInterval::from_microseconds(value.sleep_time.num_microseconds().unwrap()),
        }
    }
}

impl From<JobSqlRow> for JobInfo {
    fn from(value: JobSqlRow) -> Self {
        warn!("Make safe conversion to the Duration");
        JobInfo {
            name: value.name,
            last_invocation: value.last_invocation.and_utc(),
            sleep_time: Duration::microseconds(value.sleep_time.microseconds),
        }
    }
}
