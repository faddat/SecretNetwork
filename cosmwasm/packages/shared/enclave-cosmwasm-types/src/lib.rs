pub mod coins;
mod consts;
pub mod encoding;
pub mod math;
pub mod query;
pub mod std_error;
pub mod system_error;
pub mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
