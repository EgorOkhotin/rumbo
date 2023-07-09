use std::sync::Arc;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    error::Result,
    bson::serde_helpers::bson_datetime_as_rfc3339_string,
};
pub use serde::{Deserialize, Serialize};
use crate::{RumboApp, db_adapter::DbAdapter};
use log::{info};

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
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DiskUsageInfo {
    name: String,
    load_percents: u64,
    reading_speed: u64,
    writing_speed: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DiskSpaceInfo {
    pub name: String,
    pub total_amount: u64,
    pub free_amount: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RamSpaceInfo {
    free_amount: u64,
    total_amount: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NetworkUsageInfo {
    sending_speed: u64,
    receiving_speed: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CpuUsageInfo {
    core: u8,
    load_percents: u8,
}

pub struct MetricService {
    db_adapter: Arc<DbAdapter>,
}

const COLLECTION_NAME: &'static str = "rumbo_app";

impl MetricService {
    pub fn new(app_state: &RumboApp) -> Self {
        MetricService { db_adapter: app_state.db_adapter.clone() }
    }

    pub async fn create(&self, metric: &Metric) -> Result<Metric> {
        let collection = self
            .db_adapter
            .get_collection::<Metric>(COLLECTION_NAME);

        let result = collection.insert_one(metric, None).await?;
        let inserted_id = result.inserted_id.as_object_id().unwrap();
        let metric = self.get(&inserted_id).await?.unwrap();
        Ok(metric)
    }

    pub async fn get(&self, id: &ObjectId) -> Result<Option<Metric>> {
        let collection = self
            .db_adapter
            .get_collection::<Metric>(COLLECTION_NAME);

        let filter = doc! {"_id": id};
        let result = collection.find_one(filter, None).await?;
        Ok(result)
    }

    pub async fn delete(&self, id: &ObjectId) -> Result<()> {
        let collection = self
            .db_adapter
            .get_collection::<Metric>(COLLECTION_NAME);

        let _result = collection.delete_one(doc! {"_id": id}, None).await?;
        Ok(())
    }

    pub async fn update(&self, metric: &Metric) -> Result<Metric> {
        let collection = self
            .db_adapter
            .get_collection::<Metric>(COLLECTION_NAME);

        let filter = doc! {"_id": metric.id.unwrap() };
        let result = collection.replace_one(filter, metric, None).await?;

        if result.modified_count > 0 {
            info!("Updated entities count = {}", result.modified_count);

            let metric = self.get(&metric.id.unwrap()).await?.unwrap();
            Ok(metric)
        } else {
            let metric = self.get(&metric.id.unwrap()).await?.unwrap();
            Ok(metric)
        }
    }
}
