// This example requires the following input to succeed:
// { "command": "do something" }
#[macro_use]
extern crate tracing;

// use std::borrow::Borrow;
// use std::cell::{Cell, RefCell};
use std::time::Instant;

// use futures::{stream, StreamExt};
use lambda_runtime::{service_fn, LambdaEvent};

use sample_rust_cdk_app::*;

// Secrets Manager Config
const SECRETS: Secrets = Secrets {
    admin_user: env!("ADMIN_SECRET"),
    creds: env!("CREDS_SECRET"),
};

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging();

    let start = Instant::now();
    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;
    debug!("Call lambda took {:.2?}", start.elapsed());

    Ok(())
}

pub(crate) async fn my_handler(event: LambdaEvent<DummyEvent>) -> Result<SuccessResponse> {
    if let Some(true) = event.payload.test {
        debug!("Received a test event, nothing to do.");
        return Ok(SuccessResponse::default());
    }

    let shared_config = aws_config::load_from_env().await;

    let start = Instant::now();
    let (admin, creds) = SECRETS.fetch(&shared_config).await?;
    info!("Retrieved secrets in {:?}", start.elapsed());

    // optional feature flags as defined in `Cargo.toml`. this is enabled
    // in the CDK code in the `lib/` folder.

    #[cfg(feature = "prod")]
    let message: [&str; 5] = ["this", "is", "a", "production", "env!"];

    #[cfg(not(feature = "prod"))]
    let message: [&str; 4] = ["just", "a", "development", "environment."];

    println!("Message: {}", message.join(" "));

    let client = reqwest_client(admin)?;

    let success = true;

    // let failed = vec![];
    //
    // sample example of making parallel HTTP requests (WIP)

    // let results = stream::iter(something)
    //     .map(|app| {
    //         let client = client.clone();
    //         let user = creds.access_key_id.clone();
    //         let password = creds.password.clone();
    //
    //         tokio::spawn(async move { app.do_something(client, &user, &password).await })
    //     })
    //     .buffer_unordered(2);
    //
    // let failed_cell = RefCell::new(failed);
    // let success_cell = Cell::new(success);
    //
    // results
    //     .for_each(|b| {
    //         let mut failed = failed_cell.borrow_mut();
    //         let success = success_cell.borrow();
    //
    //         async move {
    //             match b {
    //                 Ok(Ok(_)) => {}
    //                 Ok(Err(e)) => {
    //                     success.set(false);
    //                     error!("Got a RequestError: {}", e);
    //                     failed.push(e);
    //                 }
    //                 Err(e) => {
    //                     success.set(false);
    //                     error!("Got a tokio::JoinError: {}", e);
    //                 }
    //             };
    //         }
    //     })
    //     .await;
    //
    // let success = success_cell.get();
    //
    // if success {
    //     // do something here
    // } else {
    //     let failed = failed_cell.take();
    //     warn!(failure_count = failed.len(), "lambda error",);
    //
    //     // send failure notification to admin emails
    //     send_failure_email_to_admins(&shared_config, event.context, SECRETS.creds, failed).await?;
    // }

    // prepare the response
    let resp = SuccessResponse { success };

    // return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}
