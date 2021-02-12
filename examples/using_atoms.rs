extern crate kdb;

use kdb::KBox;

fn main() {
    let mut k = KBox::new_atom(42u8);
    println!("{}", k);
    assert_eq!(k.value(), 42);

    k.set_value(43);

    assert_eq!(k.value(), 43);
}
