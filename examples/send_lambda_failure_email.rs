//! Sends a Lambda Failure email to Admins (developers).
//!
//! First, ensure your $AWS_PROFILE environment is correctly set up:
//!
//! ```shell
//! export AWS_PROFILE='my-profile'
//! ```
//!
//! # Example
//!
//! ```shell
//! cargo run --example send_lambda_failure_email
//! ```
//!
//!
#[macro_use]
extern crate tracing;

use lambda_runtime::Context;

use sample_rust_cdk_app::*;

#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init_timed_short!();

    let config = aws_config::load_from_env().await;

    let secret_name = "my-secret";
    let function_name = "my-lambda-function";
    let function_arn = format!(
        "arn:aws:lambda:us-east-1:123456789:function:{}",
        function_name
    );
    let errors = vec![
        RequestError {
            url: "https://some-sandbox-url.com".to_string(),
            host: "https://some-sandbox-url.com".to_string(),
            status: 404,
            reason: "some reason here!".to_string(),
        },
        RequestError {
            url: "https://some-dev-url.com".to_string(),
            host: "https://some-dev-url.com".to_string(),
            status: 500,
            reason: "some other reason o.O".to_string(),
        },
    ];

    let cfg = lambda_runtime::Config {
        function_name: function_name.to_owned(),
        memory: 128,
        version: "1".to_owned(),
        log_stream: "my-stream".to_owned(),
        log_group: "my-group".to_owned(),
    };

    let mut lambda_context = Context::default().with_config(&cfg);
    lambda_context.invoked_function_arn = function_arn;

    trace!("sending failure email notification");
    send_failure_email_to_admins(&config, lambda_context, secret_name, errors).await?;
    trace!("done!");

    Ok(())
}
