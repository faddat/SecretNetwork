#![cfg_attr(not(feature = "SGX_MODE_HW"), allow(unused))]

pub const LOG_LEVEL_ENV_VAR: &str = "LOG_LEVEL";
pub const SCRT_SGX_STORAGE_ENV_VAR: &str = "SCRT_SGX_STORAGE";

const DEFAULT_SGX_SECRET_PATH: &str = "/opt/secret/.sgx_secrets/";
