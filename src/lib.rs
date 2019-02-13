// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

extern crate libc;
extern crate libsolv_sys;

#[cfg(feature = "ext")]
extern crate libsolvext_sys;

pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {
        foreign_links {
            Nul(::std::ffi::NulError);
        }
    }
}

pub mod chksum;
pub mod pool;
pub mod queue;
pub mod repo;
pub mod solver;
pub mod sys;
pub mod transaction;

mod ownership;

pub use libsolv_sys::{solv_knownid, Id};

#[cfg(feature = "ext")]
pub mod ext;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
