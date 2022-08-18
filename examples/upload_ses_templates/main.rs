#[macro_use]
extern crate tracing;

use sample_rust_cdk_app::*;

use crate::helpers::SESHelper;

mod helpers;

#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init_timed_short!();

    let name = "test-failure";
    let contents = include_str!("templates/test_failure.html");

    let ses = SESHelper::new().await;

    ses.update_template(name, None, contents).await?;

    Ok(())
}
