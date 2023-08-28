mod prelude {
    pub use super::super::prelude::*;
    pub(super) use diesel::dsl::*;
}
use prelude::*;

type WithStartDate = GtEq<crate::schema::metrics::creating_date, chrono::NaiveDateTime>;
type WithEndDate = LtEq<crate::schema::metrics::creating_date, chrono::NaiveDateTime>;
type WithInstanceId = Eq<crate::schema::metrics::instance_id, i64>;

pub(super) fn with_start_date(date: chrono::DateTime<Utc>) -> WithStartDate {
    let date = date.naive_utc();
    crate::schema::metrics::dsl::creating_date.ge(date)
}

pub(super) fn with_end_date(date: chrono::DateTime<Utc>) -> WithEndDate {
    let date = date.naive_utc();
    crate::schema::metrics::dsl::creating_date.le(date)
}

pub(super) fn with_instance_id(instance_id: i64) -> WithInstanceId {
    crate::schema::metrics::dsl::instance_id.eq(instance_id)
}

#[derive(Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::metrics)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(super) struct MetricSqlRow {
    pub id: i64,
    instance_id: i64,
    metric_type: String,
    creating_date: chrono::NaiveDateTime,
    metric_value: serde_json::Value,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::metrics)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(super) struct NewMetricSqlRow {
    instance_id: i64,
    metric_type: String,
    creating_date: chrono::NaiveDateTime,
    metric_value: serde_json::Value,
}

impl From<MetricSqlRow> for Metric {
    fn from(value: MetricSqlRow) -> Self {
        let metric_value = match value.metric_type.as_str() {
            "DiskUsage" => {
                let val = serde_json::from_value(value.metric_value).unwrap();
                MetricType::DiskUsage(val)
            }
            "DiskSpace" => {
                let val = serde_json::from_value(value.metric_value).unwrap();
                MetricType::DiskSpace(val)
            }
            "RamSpace" => {
                let val = serde_json::from_value(value.metric_value).unwrap();
                MetricType::RamSpace(val)
            }
            "NetworkUsage" => {
                let val = serde_json::from_value(value.metric_value).unwrap();
                MetricType::NetworkUsage(val)
            }
            "CpuUsage" => {
                let val = serde_json::from_value(value.metric_value).unwrap();
                MetricType::CpuUsage(val)
            }
            "HealthCheck" => {
                let val = serde_json::from_value(value.metric_value).unwrap();
                MetricType::HealthCheck(val)
            }
            _ => panic!("Unknown metric type!"),
        };

        Metric {
            id: value.id,
            instance_id: value.instance_id,
            creating_date: value.creating_date.and_utc(),
            metric_value: metric_value,
        }
    }
}

impl From<Metric> for MetricSqlRow {
    fn from(value: Metric) -> Self {
        let (metric_type, metric_value) = match value.metric_value {
            MetricType::DiskUsage(val) => ("DiskUsage", serde_json::to_value(&val).unwrap()),
            MetricType::DiskSpace(val) => ("DiskSpace", serde_json::to_value(&val).unwrap()),
            MetricType::RamSpace(val) => ("RamSpace", serde_json::to_value(&val).unwrap()),
            MetricType::NetworkUsage(val) => ("NetworkUsage", serde_json::to_value(&val).unwrap()),
            MetricType::CpuUsage(val) => ("CpuUsage", serde_json::to_value(&val).unwrap()),
            MetricType::HealthCheck(val) => ("HealthCheck", serde_json::to_value(&val).unwrap()),
        };

        MetricSqlRow {
            id: value.id,
            instance_id: value.instance_id,
            creating_date: value.creating_date.naive_utc(),
            metric_type: metric_type.to_string(),
            metric_value: metric_value,
        }
    }
}

impl From<MetricSqlRow> for NewMetricSqlRow {
    fn from(value: MetricSqlRow) -> Self {
        NewMetricSqlRow {
            instance_id: value.instance_id,
            creating_date: value.creating_date,
            metric_type: value.metric_type,
            metric_value: value.metric_value,
        }
    }
}
