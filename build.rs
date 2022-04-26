// `rustc` book: https://doc.rust-lang.org/rustc/command-line-arguments.html
// `cargo` build.rs: https://doc.rust-lang.org/cargo/reference/build-scripts.html
// Opaque wrappers: https://anssi-fr.github.io/rust-guide/07_ffi.html#memory-and-resource-management

use std::env;

fn main() {
    let rpath = "/Users/ian/Projects/symphony-lang/extern/symphony-runtime/target/debug";
    println!("cargo:rustc-cdylib-link-arg=-Wl,-rpath,{}", rpath);

    let mut motion_lib_path = env::current_dir().unwrap();
    motion_lib_path.push("extern/MOTION/build/lib");
    let motion_lib_path_str = motion_lib_path.into_os_string().into_string().unwrap();
    println!("cargo:rustc-link-search=native={}", motion_lib_path_str);
    println!("cargo:rustc-link-lib=motionffi");
    println!(
        "cargo:rustc-cdylib-link-arg=-Wl,-rpath,{}",
        motion_lib_path_str
    );
}
