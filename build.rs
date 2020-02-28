fn main() {
    let base = std::env::current_dir().unwrap();
    if let Ok(lib_path) = std::env::var("LKDB_LIB_DIR") {
        println!("cargo:rustc-link-search={}", lib_path);
    } else {
        println!("cargo:rustc-link-search={}", base.to_str().unwrap());
        if let Ok(lib_path) = std::env::var("LIBRARY_PATH") {
            println!("cargo:rustc-link-search={}", lib_path);
        }
    }
}
