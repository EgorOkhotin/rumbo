use super::prelude::*;
use diesel::r2d2;
use lettre::address::AddressError as LettreAddressError;
use lettre::error::Error as LettreError;
use lettre::transport::smtp::Error as LettreSmtpError;
use std::env::VarError;
use ureq::Error as UreqError;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "error_type")]
pub enum RumboError {
    MongoError(String),
    PostgresError(String),
    VarError(String),
    UreqError(String),
    LettreError(String),
    LettreSmtpError(String),
    LettreAddressError(String),
    GenericError(String),
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

impl From<LettreAddressError> for RumboError {
    fn from(value: LettreAddressError) -> Self {
        RumboError::LettreAddressError(format!("{:?}", value))
    }
}

impl From<LettreError> for RumboError {
    fn from(value: LettreError) -> Self {
        RumboError::LettreError(format!("{:?}", value))
    }
}

impl From<LettreSmtpError> for RumboError {
    fn from(value: LettreSmtpError) -> Self {
        RumboError::LettreSmtpError(format!("{:?}", value))
    }
}

impl From<VarError> for RumboError {
    fn from(value: VarError) -> Self {
        RumboError::VarError(format!("{:?}", value))
    }
}

impl From<UreqError> for RumboError {
    fn from(value: UreqError) -> Self {
        RumboError::UreqError(format!("{:?}", value))
    }
}
