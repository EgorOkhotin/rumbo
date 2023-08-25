use diesel::r2d2;

use super::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "error_type")]
pub enum RumboError {
    MongoError(String),
    PostgresError(String),
}

impl From<r2d2::Error> for RumboError {
    fn from(value: r2d2::Error) -> Self {
        RumboError::PostgresError(format!("{:?}", value))
    }
}

impl From<diesel::result::Error> for RumboError {
    fn from(value: diesel::result::Error) -> Self {
        RumboError::PostgresError(format!("{:?}", value))
    }
}