pub mod prelude {
    pub(super) use mongodb::{
        bson::serde_helpers::{bson_datetime_as_rfc3339_string, serialize_object_id_as_hex_string},
        bson::{doc, DateTime, oid::ObjectId},
        Collection,
    };
    pub use serde::{Deserialize, Serialize};
    pub(super) use sysinfo::{DiskExt, NetworkExt, System, SystemExt};

    // Loading the lib.rs prelude
    pub use super::super::prelude::*;

    pub use super::cpu::CpuUsageInfo;
    pub use super::disk::DiskSpaceInfo;
    pub use super::disk::DiskUsageInfo;
    pub use super::network::NetworkUsageInfo;
    pub use super::ram::RamSpaceInfo;

    pub use super::Metric;
    pub use super::MetricType;
    pub use super::MetricsService;
}
use prelude::*;

mod cpu;
mod disk;
mod network;
mod ram;

#[derive(Serialize, Deserialize, Debug)]
pub struct Metric {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub instance_id: ObjectId,

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
}

pub struct MetricsService {
    db_adapter: Arc<DbAdapter>,
    instances_service: Arc<InstanceService>
}

impl MetricsService {
    pub fn new(db_adapter: &Arc<DbAdapter>, instances_service: &Arc<InstanceService>) -> Self {
        MetricsService {
            db_adapter: db_adapter.clone(),
            instances_service: instances_service.clone()
        }
    }

    pub fn as_arc(self) -> Arc<Self> {
        Arc::from(self)
    }

    pub async fn create(&self, metric: &Metric) -> Result<Metric> {
        let collection = self.get_collection();

        self.get_instance(metric).await?;

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

        // just check that instance exists
        self.get_instance(metric).await?;

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

    async fn get_instance(&self, metric: &Metric) -> Result<Instance> {
        let instance = self.instances_service.get(metric.instance_id.to_hex().as_str()).await?;
        if instance.is_none() {
            todo!("return error that instance doesn't exist");
        }

        Ok(instance.unwrap())
    }

    fn get_collection(&self) -> Collection<Metric> {
        const COLLECTION_NAME: &'static str = "metrics";
        self.db_adapter.get_collection::<Metric>(COLLECTION_NAME)
    }
}