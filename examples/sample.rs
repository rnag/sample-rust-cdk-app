#[macro_use]
extern crate tracing;

mod utils;
use utils::StageExt;

use sample_rust_cdk_app::*;

#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init_timed_short!();

    let env = Environment::Dev;

    let config = env.stage_config()?;

    // or, simply:
    //   trace!(?config);
    trace!(stage_config = ?config);

    Ok(())
}
