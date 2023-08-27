pub mod prelude {
    pub use serde::{Deserialize, Serialize};
    pub(super) use sysinfo::{
        CpuExt, CpuRefreshKind, DiskExt, NetworkExt, RefreshKind, System, SystemExt,
    };
    pub(super) use super::sql::*;

    // Loading the lib.rs prelude
    pub use super::super::prelude::*;

    pub use super::disk::prelude::*;
    pub use super::network::prelude::*;
    pub use super::ram::prelude::*;

    pub use super::cpu::CpuUsageInfo;
    pub use super::health::HealthInfo;
    pub use super::ram::RamSpaceInfo;

    pub use super::Metric;
    pub use super::MetricType;
    pub use super::MetricsService;
}
use prelude::*;

mod cpu;
mod disk;
mod health;
mod network;
mod ram;
mod sql;

#[derive(Serialize, Deserialize, Debug)]
pub struct Metric {
    pub id: i64,
    pub instance_id: i64,

    #[serde(with = "chrono::serde::ts_milliseconds")]
    creating_date: chrono::DateTime<Utc>,

    metric_value: MetricType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "metric_type")]
pub enum MetricType {
    DiskUsage(DiskUsageInfo),
    DiskSpace(DiskSpaceInfo),
    RamSpace(RamSpaceInfo),
    NetworkUsage(NetworkUsageInfo),
    CpuUsage(CpuUsageInfo),
    HealthCheck(HealthInfo),
}

pub struct MetricsService {
    db_adapter: Arc<DbAdapter>,
}

impl MetricsService {
    pub fn new(db_adapter: &Arc<DbAdapter>) -> Self {
        MetricsService {
            db_adapter: db_adapter.clone(),
        }
    }

    pub fn as_arc(self) -> Arc<Self> {
        Arc::from(self)
    }

    pub async fn create(&self, metric: Metric) -> Result<Metric> {
        use crate::schema::metrics;

        let metric = MetricSqlRow::from(metric);
        let metric = NewMetricSqlRow::from(metric);

        let mut connection = self.db_adapter.get_connection()?;
        let result = diesel::insert_into(metrics::table)
            .values(metric)
            .returning(MetricSqlRow::as_returning())
            .get_result(&mut connection)?;

        let result = Metric::from(result);
        Ok(result)
    }

    pub async fn get(&self, metric_id: i64) -> Result<Option<Metric>> {
        use crate::schema::metrics::dsl::*;

        let mut connection = self.db_adapter.get_connection()?;
        let result: Option<MetricSqlRow> = metrics
            .find(metric_id)
            .first::<MetricSqlRow>(&mut connection)
            .optional()?;

        let result = match result {
            Some(value) => Some(Metric::from(value)),
            None => None
        };

        Ok(result)
    }

    pub async fn for_period(
        &self,
        instance_id: i64,
        start_period: chrono::DateTime<Utc>,
        end_period: chrono::DateTime<Utc>,
        skip: i64,
        top: i64
    ) -> Result<Vec<Metric>> {

        let mut connection = self.db_adapter.get_connection()?;
        let result: Vec<MetricSqlRow> = crate::schema::metrics::dsl::metrics
            .filter(with_start_date(start_period))
            .filter(with_end_date(end_period))
            .filter(with_instance_id(instance_id))
            .order(crate::schema::metrics::dsl::creating_date.asc())
            .offset(skip)
            .limit(top)
            .load::<MetricSqlRow>(&mut connection)?;

        let result = result.into_iter().map(|row| {
            Metric::from(row)
        }).collect();

        Ok(result)
    }

    pub async fn delete(&self, metric_id: i64) -> Result<()> {
        use crate::schema::metrics::dsl::*;

        let mut connection = self.db_adapter.get_connection()?;
        diesel::delete(metrics.find(metric_id)).execute(&mut connection)?;
        Ok(())
    }

    pub async fn update(&self, metric: Metric) -> Result<Metric> {
        use crate::schema::metrics::dsl::*;

        let metric = MetricSqlRow::from(metric);

        let mut connection = self.db_adapter.get_connection()?;
        let result = diesel::update(metrics.find(metric.id))
            .set(metric)
            .returning(MetricSqlRow::as_returning())
            .get_result(&mut connection)?;

        let result = Metric::from(result);
        Ok(result)
    }
}
