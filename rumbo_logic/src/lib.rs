pub mod prelude {
    pub(super) use log::info;
    pub use std::sync::Arc;

    pub type Result<T> = std::result::Result<T, RumboError>;
    pub use super::db::*;
    pub use super::error::RumboError;
    pub use super::RumboApp;

    pub use super::metrics::prelude::*;
    pub use super::instances::prelude::*;
}
use instances::InstanceService;
use prelude::*;

mod db;
mod error;
mod metrics;
mod instances;

#[derive(Clone)]
pub struct RumboApp {
    // db_adapter: Arc<DbAdapter>,
    pub metrics_service: Arc<MetricsService>,
    pub instances_service: Arc<InstanceService>
}

impl RumboApp {
    pub async fn new(host: &str, app_name: &str) -> Result<Self> {
        let adapter = DbAdapter::new(host, app_name).await?;
        let db_arc = Arc::from(adapter);

        let instances_service = InstanceService::new(&db_arc).as_arc();
        let metrics_service = MetricsService::new(&db_arc, &instances_service).as_arc();

        let app = RumboApp {
            instances_service: instances_service,
            metrics_service: metrics_service,
            // db_adapter: db_arc,
        };

        Ok(app)
    }
}
