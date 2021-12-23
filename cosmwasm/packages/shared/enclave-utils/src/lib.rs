#![feature(try_reserve)]
#[cfg(not(target_env = "sgx"))]
extern crate sgx_tstd as std;

pub mod logger;
pub mod macros;
pub mod oom_handler;
pub mod pointers;
pub mod recursion_depth;
pub mod storage;
mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
