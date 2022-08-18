//! Library-specific type definitions

use crate::{CustomError, RequestError};
use lambda_runtime::Error;

/// A simple type alias so as to DRY.
pub type Result<T> = std::result::Result<T, Error>;
pub type RequestResult<T> = std::result::Result<T, RequestError>;
pub type CustomResult<T> = std::result::Result<T, CustomError>;
