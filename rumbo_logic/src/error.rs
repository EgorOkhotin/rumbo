use diesel::r2d2;

use super::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "error_type")]
pub enum RumboError {
    MongoError(String),
    PostgresError(String),
    UserError(String),
    ArgonError(String),
    TokenValidationError(String),
    RedisCacheError(String)
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

impl From<argon2::password_hash::Error> for RumboError {
    fn from(_value: argon2::password_hash::Error) -> Self {
        const ERROR_MESSAGE: &str =
            "Can't create a password hash. Please, check the corectness of input data";
        RumboError::ArgonError(ERROR_MESSAGE.to_string())
    }
}

impl From<jwt_compact::ValidationError> for RumboError {
    fn from(value: jwt_compact::ValidationError) -> Self {
        RumboError::TokenValidationError(format!("{:?}", value))
    }
}

impl From<redis::RedisError> for RumboError {
    fn from(value: redis::RedisError) -> Self {
        RumboError::RedisCacheError(format!("{:?}", value))
    }
}
