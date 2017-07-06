extern crate libc;
extern crate libsolv_sys;

#[cfg(feature = "ext")]
extern crate libsolvext_sys;

pub mod queue;
pub mod pool;
pub mod repo;
pub mod solver;
pub mod transaction;
pub mod chksum;
mod ownership;

#[cfg(feature = "ext")]
pub mod ext;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
