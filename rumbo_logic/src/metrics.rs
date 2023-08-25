pub mod prelude {
    pub use serde::{Deserialize, Serialize};
    pub(super) use sysinfo::{
        CpuExt, CpuRefreshKind, DiskExt, NetworkExt, RefreshKind, System, SystemExt,
    };

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
use std::cmp::Ordering;
use diesel::dsl::*;

use prelude::*;

mod cpu;
mod disk;
mod health;
mod network;
mod ram;

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

fn with_start_date(date: chrono::DateTime<Utc>) -> GtEq<crate::schema::metrics::creating_date, chrono::NaiveDateTime> {
    let date = date.naive_utc();
    crate::schema::metrics::dsl::creating_date.ge(date)
}

fn with_end_date(date: chrono::DateTime<Utc>) -> LtEq<crate::schema::metrics::creating_date, chrono::NaiveDateTime> {
    let date = date.naive_utc();
    crate::schema::metrics::dsl::creating_date.le(date)
}

fn with_instance_id(instance_id: i64) -> Eq<crate::schema::metrics::id, i64> {
    crate::schema::metrics::dsl::id.eq(instance_id)
}

#[derive(Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::metrics)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct MetricSqlRow {
    id: i64,
    instance_id: i64,
    metric_type: String,
    creating_date: chrono::NaiveDateTime,
    metric_value: serde_json::Value
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::metrics)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewMetricSqlRow {
    instance_id: i64,
    metric_type: String,
    creating_date: chrono::NaiveDateTime,
    metric_value: serde_json::Value
}

impl From<MetricSqlRow> for Metric {
    fn from(value: MetricSqlRow) -> Self {
        let metric_value = match value.metric_type.as_str() {
            "DiskUsage" => {
                let val = serde_json::from_value(value.metric_value).unwrap();
                MetricType::DiskUsage(val)
            },
            "DiskSpace" => {
                let val = serde_json::from_value(value.metric_value).unwrap();
                MetricType::DiskSpace(val)
            },
            "RamSpace" => {
                let val = serde_json::from_value(value.metric_value).unwrap();
                MetricType::RamSpace(val)
            },
            "NetworkUsage" => {
                let val = serde_json::from_value(value.metric_value).unwrap();
                MetricType::NetworkUsage(val)
            },
            "CpuUsage" => {
                let val = serde_json::from_value(value.metric_value).unwrap();
                MetricType::CpuUsage(val)
            },
            "HealthCheck" => {
                let val = serde_json::from_value(value.metric_value).unwrap();
                MetricType::HealthCheck(val)
            },
            _ => panic!("Unknown metric type!")
        };

        Metric {
            id: value.id,
            instance_id: value.instance_id,
            creating_date: value.creating_date.and_utc(),
            metric_value: metric_value
        }
    }
}

impl From<Metric> for MetricSqlRow {
    fn from(value: Metric) -> Self {
        let (metric_type, metric_value) = match value.metric_value {
            MetricType::DiskUsage(val) => {
                ("DiskUsage", serde_json::to_value(&val).unwrap())
            },
            MetricType::DiskSpace(val) => {
                ("DiskSpace", serde_json::to_value(&val).unwrap())
            },
            MetricType::RamSpace(val) => {
                ("RamSpace", serde_json::to_value(&val).unwrap())
            },
            MetricType::NetworkUsage(val) => {
                ("NetworkUsage", serde_json::to_value(&val).unwrap())
            },
            MetricType::CpuUsage(val) => {
                ("CpuUsage", serde_json::to_value(&val).unwrap())
            },
            MetricType::HealthCheck(val) => {
                ("HealthCheck", serde_json::to_value(&val).unwrap())
            },
        };

        MetricSqlRow { 
            id: value.id,
            instance_id: value.instance_id,
            creating_date: value.creating_date.naive_utc(),
            metric_type: metric_type.to_string(),
            metric_value: metric_value
        }
    }
}

impl From<MetricSqlRow> for NewMetricSqlRow {
    fn from(value: MetricSqlRow) -> Self {
        NewMetricSqlRow {
            instance_id: value.instance_id,
            creating_date: value.creating_date,
            metric_type: value.metric_type,
            metric_value: value.metric_value
        }
    }
}
