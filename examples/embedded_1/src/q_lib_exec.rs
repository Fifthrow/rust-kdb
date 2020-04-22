use nix::unistd::execvp;
use std::ffi::{CStr, CString};
use std::env::args;

fn main() {
    let capp = CString::new("q").unwrap(); // "q36.64" 
    let mut cargs = [
        CStr::from_bytes_with_nul(b"q\0").unwrap(),
        CStr::from_bytes_with_nul(b"src/load.q\0").unwrap(),
        CStr::from_bytes_with_nul(b"\0").unwrap(),
        CStr::from_bytes_with_nul(b"\0").unwrap(),
    ];

    if let Some(in_args) = args().nth(1) {
        match in_args.as_str() {
          "m32" => cargs[2] = CStr::from_bytes_with_nul(b"m32\0").unwrap(),
          _ => {},
        }
    };
    let _ = match execvp(&capp, &cargs) {
        Err(e) => println!("Can't exec q due: {:?}. Make sure q is on the PATH.", e),
        _ => {} //exec replaces this process, so this cond is unreachable
    };
}
