//! Models for internal (local) testing only.
//!
//! # Note
//! These models won't be used in the deployed AWS Lambda functions.
//!
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StageConfig<'a> {
    #[serde(borrow)]
    pub secrets: Secrets<'a>,
}

#[derive(Debug, Deserialize)]
pub struct Secrets<'a> {
    pub admin_user: &'a str,
    pub creds: &'a str,
}
