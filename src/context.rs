//! Error context (and error handling) utilities
//!
use crate::{CustomError, CustomResult};
use std::fmt::Display;

const ERR_MESSAGE: &str = "The lambda encountered an error and your request was not processed";

pub trait DefaultResponse {
    fn new_response() -> Self;
}

impl DefaultResponse for CustomError {
    fn new_response() -> Self {
        Self {
            msg: ERR_MESSAGE.to_owned(),
        }
    }
}

pub trait AwsErrorContext<T, E: std::fmt::Debug + Send + Sync + 'static> {
    fn map_context<O: FnOnce(E)>(self, op: O) -> CustomResult<T>;
    fn context<D: Display>(self, message: D) -> CustomResult<T>;
}

impl<T, E: std::fmt::Debug + Send + Sync + 'static> AwsErrorContext<T, E> for Result<T, E> {
    fn map_context<O: FnOnce(E)>(self, op: O) -> CustomResult<T> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                op(e);
                // The sender of the request receives this message in response.
                Err(CustomError::new_response())
            }
        }
    }

    fn context<D: Display>(self, message: D) -> CustomResult<T> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                error!("{}: {:?}", message, e);
                // The sender of the request receives this message in response.
                Err(CustomError::new_response())
            }
        }
    }
}
