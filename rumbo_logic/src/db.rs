pub mod prelude {
    pub(super) use diesel::{
        r2d2::{ConnectionManager, Pool},
        PgConnection,
    };

    // use from lib.rs
    pub use super::super::prelude::*;

    pub use super::DbAdapter;
}
use diesel::r2d2::PooledConnection;
use prelude::*;

#[derive(Clone)]
pub struct DbAdapter {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DbAdapter {
    pub async fn new(connection_string: &str) -> Result<Self> {
        let manager = ConnectionManager::<PgConnection>::new(connection_string);
        info!("Created connection manager");

        let pool = Pool::builder()
            .build(manager)
            .expect("Could not build connection pool");

        let result = DbAdapter { pool };
        Ok(result)
    }

    pub fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>> {
        let result = self.pool.get();
        match result {
            Ok(val) => Ok(val),
            Err(error) => Err(RumboError::PostgresError(error.to_string()))
        }
    }
}
