#[macro_use]
extern crate tracing;

pub use aws::*;
pub use constants::*;
pub use context::*;
pub use models::*;
pub use requests::*;
pub use types::*;
pub use utils::*;

mod aws;
mod constants;
mod context;
mod models;
mod requests;
mod types;
mod utils;

/// Prints the type name (*without* the namespace) of a generic or variable.
///
/// See: https://stackoverflow.com/a/73313062/10237506
#[allow(unused)]
macro_rules! ty {
    ($type:ty) => {{
        let result = std::any::type_name::<$type>();
        match result.rsplit_once(':') {
            Some((_, s)) => s,
            None => result,
        }
    }};
}
