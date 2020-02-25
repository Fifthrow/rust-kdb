const KLIB: &str = ".";

fn main() {
    let base = std::env::current_dir().unwrap();
    match std::env::var("LKDB_LIB_DIR") {
        Ok(ld) => {
            println!("cargo:rustc-link-search={}", ld);
        }
        _ => {
            println!("cargo:rustc-link-search={}/{}", base.to_str().unwrap(), KLIB);
        }
    };
}
