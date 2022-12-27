use std::{env::VarError, num::ParseIntError};

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Errors>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Errors {
    #[error("BOT_TOKEN or CHAT_ID are missings!")]
    EnvVarErrors(#[from] VarError),

    #[error("Error when parsing CHAT_ID")]
    ParseChatIdError(#[from] ParseIntError),

    #[error("Ureq error: {0}")]
    UreqError(String),

    #[error(transparent)]
    SOSMedecinsApiError(#[from] SOSMedecinsApiErrors),
}

impl From<ureq::Error> for Errors {
    fn from(e: ureq::Error) -> Self {
        Self::UreqError(e.into_response().unwrap().into_string().unwrap())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SOSMedecinsApiErrors {
    #[error("SOS Medecins call failed")]
    NotSuccess,
}
