use super::prelude::*;

pub struct PostgresJobStorageService {
    db_adapter: Arc<DbAdapter>
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
}

#[async_trait]
impl JobStorageService for PostgresJobStorageService {
    async fn save(&self, info: JobInfo) -> Result<Option<JobInfo>> {
        todo!()
    }

    async fn load(&self, name: &str) -> Result<Option<JobInfo>> {
        todo!()
    }
}