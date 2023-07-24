pub mod prelude {
    pub(super) use mongodb::{
        bson::serde_helpers::bson_datetime_as_rfc3339_string,
        bson::{doc, DateTime},
        Collection,
    };
    pub use serde::{Deserialize, Serialize};
    pub(super) use sysinfo::{DiskExt, NetworkExt, System, SystemExt};

    // Loading the lib.rs prelude
    pub use super::super::prelude::*;

    pub use super::cpu::CpuUsageInfo;
    pub use super::disk::DiskSpaceInfo;
    pub use super::disk::DiskUsageInfo;
    pub use super::health::HealthInfo;
    pub use super::network::NetworkUsageInfo;
    pub use super::ram::RamSpaceInfo;

    pub use super::Metric;
    pub use super::MetricType;
    pub use super::MetricsService;
}
use mongodb::bson::{oid::ObjectId, Document};
use prelude::*;

mod cpu;
mod disk;
mod health;
mod network;
mod ram;

#[derive(Serialize, Deserialize, Debug)]
pub struct Metric {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub instance_name: String,

    #[serde(with = "bson_datetime_as_rfc3339_string")]
    timestamp: DateTime,

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
    pub fn new(app_state: &RumboApp) -> Self {
        MetricsService {
            db_adapter: app_state.db_adapter.clone(),
        }
    }

    pub async fn create(&self, metric: &Metric) -> Result<Metric> {
        let collection = self.get_collection();

        let result = collection.insert_one(metric, None).await?;
        let inserted_id = result.inserted_id.as_object_id().unwrap().to_hex();
        let metric = self.get(&inserted_id).await?.unwrap();
        Ok(metric)
    }

    pub async fn get(&self, id: &str) -> Result<Option<Metric>> {
        let collection = self.get_collection();

        let filter = get_id_filter_from_str(id);
        let result = collection.find_one(filter, None).await?;
        Ok(result)
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        let collection = self.get_collection();
        let filter = get_id_filter_from_str(id);

        let _result = collection.delete_one(filter, None).await?;
        Ok(())
    }

    pub async fn update(&self, metric: &Metric) -> Result<Metric> {
        let collection = self.get_collection();

        let id = metric.id.unwrap();
        let filter = get_id_filter_from_object(&id);

        let result = collection.replace_one(filter, metric, None).await?;

        if result.modified_count > 0 {
            info!("Updated entities count = {}", result.modified_count);

            let metric = self.get(&id.to_hex()).await?.unwrap();
            Ok(metric)
        } else {
            let metric = self.get(&id.to_hex()).await?.unwrap();
            Ok(metric)
        }
    }

    fn get_collection(&self) -> Collection<Metric> {
        const COLLECTION_NAME: &'static str = "rumbo_app";
        self.db_adapter.get_collection::<Metric>(COLLECTION_NAME)
    }
}

fn get_id_filter_from_str(id: &str) -> Document {
    let object_id = ObjectId::parse_str(id).unwrap();
    get_id_filter_from_object(&object_id)
}

fn get_id_filter_from_object(id: &ObjectId) -> Document {
    doc! {"_id": id }
}
