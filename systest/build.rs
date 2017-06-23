extern crate ctest;

fn main() {
    let mut cfg = ctest::TestGenerator::new();

    cfg.header("solv/queue.h");
    cfg.generate("../libsolv-sys/src/lib.rs", "all.rs");
}
