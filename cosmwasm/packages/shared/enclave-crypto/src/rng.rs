use crate::CryptoError;

pub fn rand_slice(rand: &mut [u8]) -> Result<(), CryptoError> {
    use sgx_trts::trts::rsgx_read_rand;
    rsgx_read_rand(rand).map_err(|_e| CryptoError::RandomError {})
}
