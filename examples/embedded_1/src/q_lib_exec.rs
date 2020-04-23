use std::env::args;
use std::process::Command;
fn main() {
    let mut cmd = Command::new("q");
    cmd.arg("src/load.q");

    if let Some("m32") = args().nth(1).as_ref().map(String::as_str) {
        cmd.arg("m32");
    };
    cmd.status().unwrap();
}
