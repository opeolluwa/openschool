use std::str::FromStr;

use crate::errors::app_error::AppError;

pub fn extract_env<T: FromStr>(env_key: &str) -> Result<T, AppError> {
    openschool_kernel::env::extract_env(env_key).map_err(|err| match err {
        openschool_kernel::env::EnvError::Missing(key) => AppError::EnvError(key),
        openschool_kernel::env::EnvError::Parse(key) => {
            AppError::EnvError(format!("error parsing env: {key}"))
        }
    })
}
