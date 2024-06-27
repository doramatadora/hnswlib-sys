extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    let header_path = "./src/wrapper.h";

    println!("cargo:rerun-if-changed={}", header_path);

    let bindings = bindgen::Builder::default()
        .header(header_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // I want bindings for functions and types to use the hnswlib namespace.
        .allowlist_function("hnswlib::.*")
        .allowlist_type("hnswlib::.*")
        // I want it to use libc crate types. Does this achieve that?
        .use_core()
        .ctypes_prefix("libc")
        .clang_arg("-xc++")
        .clang_arg("-std=c++11")
        // .clang_arg("-stdlib=libc++") 
        .generate_comments(false)
        // There's 300+ formatter warnings about naming conventions.
        // e.g., warning: type __darwin_pthread_mutex_t should have an upper camel case name
        // I want to suppress those. I tried the below, but it leads to weird errors
        // about inner attributes not being permitted in some contexts.
        // .raw_line("#![allow(non_camel_case_types)]")
        // .raw_line("#![allow(non_upper_case_globals)]")
        // .raw_line("#![allow(non_snake_case)]")
        .generate()
        .expect("🚒 failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("💩 failed to write bindings file");
}


// This almost works. I get two errors related to std::priority_queue 
// and 330 formatter warnings which I suppress by running 
// RUSTFLAGS="-Awarnings" cargo build 😭

// error[E0412]: cannot find type `_Container` in this scope
//     --> /Users/dmilitaru/fsly/hnswlib-sys/target/debug/build/hnswlib-sys-cffd6de768b19101/out/bindings.rs:1054:45
//      |
// 1054 |     pub search_results_: std_priority_queue<_Container, _Compare>,
//      |                                             ^^^^^^^^^^ not found in this scope
//      |
// help: you might be missing a type parameter
//      |
// 1048 | pub struct hnswlib_MultiVectorSearchStopCondition<_Container> {
//      |                                                  ++++++++++++

// error[E0412]: cannot find type `_Compare` in this scope
//     --> /Users/dmilitaru/fsly/hnswlib-sys/target/debug/build/hnswlib-sys-cffd6de768b19101/out/bindings.rs:1054:57
//      |
// 1054 |     pub search_results_: std_priority_queue<_Container, _Compare>,
//      |                                                         ^^^^^^^^ not found in this scope
//      |
// help: you might be missing a type parameter
//      |
// 1048 | pub struct hnswlib_MultiVectorSearchStopCondition<_Compare> {
//      |                                                  ++++++++++

// For more information about this error, try `rustc --explain E0412`.
// error: could not compile `hnswlib-sys` (lib) due to 2 previous errors
