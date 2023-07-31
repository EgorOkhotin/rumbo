pub mod prelude {
    pub(super) use mongodb::{
        bson::{doc, oid::ObjectId},
        Collection,
    };
    pub use serde::{Deserialize, Serialize};

    // Loading the lib.rs prelude
    pub use super::super::prelude::*;

    pub use super::Instance;
    pub use super::InstanceService;
}
use prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Instance {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
}

#[derive(Clone)]
pub struct InstanceService {
    db_adapter: Arc<DbAdapter>,
}

impl InstanceService {
    pub fn new(db_adapter: &Arc<DbAdapter>) -> Self {
        InstanceService {
            db_adapter: db_adapter.clone(),
        }
    }

    pub fn as_arc(self) -> Arc<Self> {
        Arc::from(self)
    }

    pub async fn create(&self, instance: &Instance) -> Result<Instance> {
        let collection = self.get_collection();

        let result = collection.insert_one(instance, None).await?;
        let inserted_id = result.inserted_id.as_object_id().unwrap().to_hex();
        let metric = self.get(&inserted_id).await?.unwrap();
        Ok(metric)
    }

    pub async fn get(&self, id: &str) -> Result<Option<Instance>> {
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

    pub async fn update(&self, instance: &Instance) -> Result<Instance> {
        let collection = self.get_collection();

        let id = instance.id.unwrap();
        let filter = get_id_filter_from_object(&id);

        let result = collection.replace_one(filter, instance, None).await?;

        if result.modified_count > 0 {
            info!("Updated entities count = {}", result.modified_count);

            let metric = self.get(&id.to_hex()).await?.unwrap();
            Ok(metric)
        } else {
            let metric = self.get(&id.to_hex()).await?.unwrap();
            Ok(metric)
        }
    }

    fn get_collection(&self) -> Collection<Instance> {
        const COLLECTION_NAME: &'static str = "instances";
        self.db_adapter.get_collection::<Instance>(COLLECTION_NAME)
    }
}
