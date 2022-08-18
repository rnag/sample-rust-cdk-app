//! Project-specific model definitions
//!
//! Generated in part from [aws-lambda-events].
//!
//! [aws-lambda-events]: https://github.com/LegNeato/aws-lambda-events/blob/master/aws_lambda_events/src/generated/s3.rs
//!
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{is_default, RequestError};

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
}

impl Default for SuccessResponse {
    fn default() -> Self {
        Self { success: true }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DummyEvent {
    pub test: Option<bool>,
}

// note: commenting out since the Lambda won't be triggered by a CW Event rule

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "kebab-case")]
// pub struct CloudWatchEvent {
//     pub account: String,
//     pub detail: Detail,
//     pub detail_type: String,
//     pub id: String,
//     pub region: String,
//     pub resources: Vec<String>,
//     pub source: String,
//     pub time: String,
//     #[serde(default)]
//     pub version: String,
// }
//
// #[derive(Debug, Deserialize, Serialize)]
// pub struct Detail {}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomError {
    pub msg: String,
}

impl std::error::Error for CustomError {
    // this implementation required `Debug` and `Display` traits
}

impl std::fmt::Display for CustomError {
    /// Display the error struct as a JSON string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_as_json = json!(self).to_string();
        write!(f, "{}", err_as_json)
    }
}

/// Template data for the `send-to-outlook` SES template
#[derive(Serialize, Default)]
pub struct TemplateData<'a> {
    /// The subject part of the email
    pub subject: &'a str,
    /// Test secret name
    pub secret_name: &'a str,
    /// The name of the AWS Lambda Function
    pub function_name: &'a str,
    /// AWS Account info
    #[serde(skip_serializing_if = "is_default")]
    pub account: AccountInfo<'a>,
    /// Helpful links
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<RequestError>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link<'a>>,
}

#[derive(Serialize, Default, PartialEq)]
pub struct AccountInfo<'a> {
    pub name: &'a str,
    pub id: &'a str,
}

#[derive(Serialize, Default)]
pub struct Link<'a> {
    pub location: String,
    pub text: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    use lambda_runtime::Error;
    use serde_json::{from_value, json, to_string_pretty};

    #[test]
    fn test_deserialize_dummy_event() -> Result<(), Error> {
        sensible_env_logger::safe_init!();

        let input_data = json!({"test": true});
        let request: DummyEvent = from_value(input_data)?;
        trace!("Request data: {}", to_string_pretty(&request)?);

        let input_data = json!({});
        let request: DummyEvent = from_value(input_data)?;
        trace!("Request data: {}", to_string_pretty(&request)?);

        Ok(())
    }

    // #[test]
    // fn test_deserialize_cw_event() -> Result<(), Error> {
    //     sensible_env_logger::safe_init!();
    //
    //     let input_data = json!({
    //       "id": "cdc73f9d-aea9-11e3-9d5a-835b769c0d9c",
    //       "detail-type": "Scheduled Event",
    //       "source": "aws.events",
    //       "account": "123456789012",
    //       "time": "1970-01-01T00:00:00Z",
    //       "region": "us-east-1",
    //       "resources": [
    //         "arn:aws:events:us-east-1:123456789012:rule/ExampleRule"
    //       ],
    //       "detail": {}
    //     });
    //
    //     let request: CloudWatchEvent = from_value(input_data)?;
    //
    //     trace!("Request data: {}", to_string_pretty(&request)?);
    //
    //     Ok(())
    // }

    #[test]
    fn test_serialize_account_info() {
        sensible_env_logger::init!();

        let account = AccountInfo {
            name: "aws-apdaily-prod",
            id: "1234567",
        };

        let res = to_string_pretty(&account).unwrap();
        trace!("Result: {res}");
    }
}
