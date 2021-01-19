extern crate kdb;

use kdb::KBox;

fn main() {
    let k = KBox::new_atom(42u8);
    println!("{}", k);
    assert_eq!(k.value(), 42);
}
