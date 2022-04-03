fn main() {
    let motion_lib_path = "extern/MOTION/build/lib";
    println!("cargo:rustc-env=DYLD_LIBRARY_PATH={}", motion_lib_path);
    println!("cargo:rustc-link-search={}", motion_lib_path);
    println!("cargo:rustc-link-lib=motionffi");
}
