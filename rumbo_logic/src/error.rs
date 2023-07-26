use super::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "error_type")]
pub enum RumboError {
    MongoError(String),
}

impl From<mongodb::error::Error> for RumboError {
    fn from(value: mongodb::error::Error) -> Self {
        RumboError::MongoError(format!("{:?}", value))
    }
}
