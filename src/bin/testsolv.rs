extern crate libc;
extern crate libsolv;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use libsolv::ext::testcase;
use libsolv::queue::Queue;
use libsolv::pool::{PoolContext, Pool};
use libsolv::solver::Solver;

fn main() {
    let path = Path::new("../libsolv/test/testcases/choose/default.t");
    let mut job = Queue::new();
    let pool = PoolContext::new();
    if let Ok((mut solver, result, resultflags)) = testcase::read(&pool, path, &mut job) {
        solver.solve(&mut job);
        let myresult = testcase::solverresult(&mut solver, resultflags).unwrap();
        println!("{:?}", myresult);
    }
}