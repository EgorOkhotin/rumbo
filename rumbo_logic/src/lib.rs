pub mod prelude {
    pub(super) use log::info;
    pub use std::sync::Arc;

    pub type Result<T> = std::result::Result<T, RumboError>;
    pub use super::db::DbAdapter;
    pub use super::error::RumboError;
    pub use super::RumboApp;

    pub use super::metrics::prelude::*;
}
use prelude::*;

mod db;
mod error;
mod metrics;

#[derive(Clone)]
pub struct RumboApp {
    db_adapter: Arc<DbAdapter>,
}

impl RumboApp {
    pub async fn new(host: &str, app_name: &str) -> Result<Self> {
        let adapter = DbAdapter::new(host, app_name).await?;
        let app = RumboApp {
            db_adapter: Arc::from(adapter),
        };

        Ok(app)
    }
}
