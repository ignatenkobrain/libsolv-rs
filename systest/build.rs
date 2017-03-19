extern crate ctest;

fn main() {
    let mut cfg = ctest::TestGenerator::new();

    cfg.header("solv/queue.h");
    // No need for `struct X`
    cfg.type_name(|ty, _is_struct| ty.to_string());
    // Those do not really exist
    cfg.skip_fn(|s| s.ends_with("_real"));

    cfg.generate("../libsolv-sys/src/lib.rs", "all.rs");
}
