extern crate libc;
extern crate libsolv_sys;

mod queue;
mod pool;
mod repo;
mod solver;
mod transaction;
mod ext;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
