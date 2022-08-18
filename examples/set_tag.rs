#[macro_use]
extern crate tracing;

use sample_rust_cdk_app::*;

#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init_timed_short!();

    let shared_config = aws_config::load_from_env().await;

    let secret_name = "my/secret";
    let tag_key = "some-tag";
    let tag_value = "test";

    trace!(
        secret = secret_name,
        key = tag_key,
        value = tag_value,
        "updating tag"
    );

    let _ = secret_name
        .set_tag(&shared_config, tag_key, tag_value)
        .await?;

    trace!(secret = secret_name, "successfully updated the tag");

    Ok(())
}
