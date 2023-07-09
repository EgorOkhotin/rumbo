use std::sync::Arc;
use db_adapter::DbAdapter;
pub use metrics::MetricService;
pub use mongodb::error::Result;
pub use mongodb::bson::oid::ObjectId;

pub mod metrics;
mod db_adapter;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Clone)]
pub struct RumboApp {
    db_adapter: Arc<DbAdapter>,
}

impl RumboApp {
    pub async fn new(host: &str, app_name: &str) -> Result<Self> {
        let adapter = DbAdapter::new(host, app_name).await?;
        let app = RumboApp {
            db_adapter: Arc::from(adapter)
        };

        Ok(app)
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
