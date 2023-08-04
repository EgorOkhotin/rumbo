pub use super::prelude::*;
use mongodb::Collection;

pub struct MongoJobStorageService {
    db_adapter: Arc<DbAdapter>,
}

impl MongoJobStorageService {
    pub fn new(db: &Arc<DbAdapter>) -> Self {
        MongoJobStorageService {
            db_adapter: db.clone(),
        }
    }

    pub fn as_arc(self) -> Arc<Self> {
        Arc::from(self)
    }

    fn get_collection(&self) -> Collection<JobInfo> {
        const COLLECTION_NAME: &str = "JobInfo";
        self.db_adapter.get_collection(COLLECTION_NAME)
    }
}

#[async_trait]
impl JobStorageService for MongoJobStorageService {
    async fn save(&self, info: JobInfo) -> Result<Option<JobInfo>> {
        let collection = self.get_collection();

        let id = &info.name.clone();

        let some = collection
            .replace_one(get_id_filter(&info), info, None)
            .await?;

        if some.modified_count == 0 {
            const ZERO_MODIFIED_ENTITIES_ERROR_MESSAGE: &str = "Updated count = 0";
            return Err(RumboError::MongoError(
                ZERO_MODIFIED_ENTITIES_ERROR_MESSAGE.to_string(),
            ));
        }

        info!("Updated {} jobs in DB", some.modified_count);
        let result = self.load(id).await?;

        Ok(result)
    }

    async fn load(&self, name: &str) -> Result<Option<JobInfo>> {
        let collection = self.get_collection();
        let result = collection
            .find_one(get_id_filter_from_str(name), None)
            .await?;
        Ok(result)
    }
}
fn get_id_filter(info: &JobInfo) -> Document {
    doc! {ID_FIELD_NAME: &info.name }
}

fn get_id_filter_from_str(name: &str) -> Document {
    doc! {ID_FIELD_NAME: name }
}
