extern crate kdb;

use kdb::KByteAtom;

fn main() {
    let k: KByteAtom = 42.into();

    println!("{}", *k);
}
