use super::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "error_type")]
pub enum RumboError {
    MongoError(String),
    UserError(String),
    ArgonError(String),
}

impl From<mongodb::error::Error> for RumboError {
    fn from(value: mongodb::error::Error) -> Self {
        RumboError::MongoError(format!("{:?}", value))
    }
}

impl From<argon2::password_hash::Error> for RumboError {
    fn from(value: argon2::password_hash::Error) -> Self {
        const ERROR_MESSAGE: &str =
            "Can't create a password hash. Please, check the corectness of input data";
        RumboError::ArgonError(ERROR_MESSAGE.to_string())
    }
}
