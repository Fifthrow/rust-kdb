extern crate kdb;

use kdb::{KDict, KSymbolAtom, Symbol};
use std::convert::TryFrom;

fn main() {
    //Symbols use a lot of try_into. Sad.
    let mut dict = KDict::new();

    dict.insert(1i32, Symbol::try_from("One").unwrap());
    dict.insert(2i32, Symbol::try_from("Two").unwrap());
    dict.insert(3i32, Symbol::try_from("Three").unwrap());

    println!("{:?}", <&KSymbolAtom>::try_from(&dict[2i32]).unwrap());
}
