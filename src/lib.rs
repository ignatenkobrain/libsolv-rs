extern crate libc;
extern crate libsolv_sys;

mod queue;

pub struct Pool {
    pool: libsolv_sys::Pool
}

pub struct Repo {
    repo: libsolv_sys::Repo,
}

pub struct Solver {
    solver: libsolv_sys::Solver,
}

pub struct Transaction {
    t: libsolv_sys::Transaction,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use libsolv_sys::Pool;
        use libsolv_sys::pool_debug;
    }
}
