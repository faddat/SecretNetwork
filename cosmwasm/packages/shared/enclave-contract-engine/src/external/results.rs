use enclave_ffi_types::{
    EnclaveError, HandleResult, InitResult, QueryResult, UntrustedVmError, UserSpaceBuffer,
};
use sgx_types::sgx_status_t;

use super::ocalls::ocall_allocate_qe;
use crate::types::{HandleSuccess, InitSuccess, QuerySuccess};

pub fn result_init_success_to_initresult(result: Result<InitSuccess, EnclaveError>) -> InitResult {
    match result {
        Ok(InitSuccess {
            output,
            contract_key,
        }) => {
            let user_buffer = unsafe {
                let mut user_buffer = std::mem::MaybeUninit::<UserSpaceBuffer>::uninit();
                match ocall_allocate_qe(user_buffer.as_mut_ptr(), output.as_ptr(), output.len()) {
                    sgx_status_t::SGX_SUCCESS => { /* continue */ }
                    _ => {
                        return InitResult::Failure {
                            err: EnclaveError::FailedOcall {
                                vm_error: UntrustedVmError::default(),
                            },
                        }
                    }
                }
                user_buffer.assume_init()
            };
            InitResult::Success {
                output: user_buffer,
                contract_key,
            }
        }
        Err(err) => InitResult::Failure { err },
    }
}

pub fn result_handle_success_to_handleresult(
    result: Result<HandleSuccess, EnclaveError>,
) -> HandleResult {
    match result {
        Ok(HandleSuccess { output }) => {
            let user_buffer = unsafe {
                let mut user_buffer = std::mem::MaybeUninit::<UserSpaceBuffer>::uninit();
                match ocall_allocate_qe(user_buffer.as_mut_ptr(), output.as_ptr(), output.len()) {
                    sgx_status_t::SGX_SUCCESS => { /* continue */ }
                    _ => {
                        return HandleResult::Failure {
                            err: EnclaveError::FailedOcall {
                                vm_error: UntrustedVmError::default(),
                            },
                        }
                    }
                }
                user_buffer.assume_init()
            };
            HandleResult::Success {
                output: user_buffer,
            }
        }
        Err(err) => HandleResult::Failure { err },
    }
}

pub fn result_query_success_to_queryresult(
    result: Result<QuerySuccess, EnclaveError>,
) -> QueryResult {
    match result {
        Ok(QuerySuccess { output }) => {
            let user_buffer = unsafe {
                let mut user_buffer = std::mem::MaybeUninit::<UserSpaceBuffer>::uninit();
                match ocall_allocate_qe(user_buffer.as_mut_ptr(), output.as_ptr(), output.len()) {
                    sgx_status_t::SGX_SUCCESS => { /* continue */ }
                    _ => {
                        return QueryResult::Failure {
                            err: EnclaveError::FailedOcall {
                                vm_error: UntrustedVmError::default(),
                            },
                        }
                    }
                }
                user_buffer.assume_init()
            };
            QueryResult::Success {
                output: user_buffer,
            }
        }
        Err(err) => QueryResult::Failure { err },
    }
}
