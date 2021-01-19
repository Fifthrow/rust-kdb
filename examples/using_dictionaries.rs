extern crate kdb;

use kdb::{symbol, Atom, KBox, Symbol};
use std::convert::TryFrom;

fn main() {
    //Symbols use a lot of try_into. Sad.
    let mut dict = KBox::new_dict();

    dict.insert(1i32, symbol("One"));
    dict.insert(2i32, symbol("Two"));
    dict.insert(3i32, symbol("Three"));

    println!("{:?}", <&Atom<Symbol>>::try_from(&dict[2i32]).unwrap());
}
