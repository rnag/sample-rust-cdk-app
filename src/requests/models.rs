use std::fmt;
use std::str::FromStr;

use reqwest::Url;
use serde::Serialize;
use serde_json::json;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Serialize)]
pub struct RequestError {
    pub url: String,
    pub host: String,
    pub status: u16,
    pub reason: String,
}

impl From<reqwest::Error> for RequestError {
    fn from(e: reqwest::Error) -> Self {
        let url = match e.url() {
            None => Url::from_str("https://default-url.com").unwrap(),
            Some(u) => u.clone(),
        };
        let status = match e.status() {
            None => 400,
            Some(s) => s.as_u16(),
        };

        Self {
            url: url.to_string(),
            host: url.host_str().unwrap().to_owned(),
            status,
            reason: e.without_url().to_string(),
        }
    }
}

impl std::error::Error for RequestError {
    // this implementation required `Debug` and `Display` traits
}

impl fmt::Display for RequestError {
    /// Display the error struct as a JSON string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_as_json = json!(self).to_string();
        write!(f, "{}", err_as_json)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Environment {
    Prod,
    Staging,
    Sandbox,
    Dev,
}
