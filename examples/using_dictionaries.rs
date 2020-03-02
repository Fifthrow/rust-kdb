extern crate kdb;

use kdb::{KDict, KSymbol, KSymbolAtom};
use std::convert::TryFrom;

fn main() {
    //Symbols use a lot of try_into. Sad.
    let mut dict = KDict::new();

    dict.insert(1i32, KSymbol::try_from("One").unwrap());
    dict.insert(2i32, KSymbol::try_from("Two").unwrap());
    dict.insert(3i32, KSymbol::try_from("Three").unwrap());

    println!("{:?}", **dict[2i32].try_as_ref::<KSymbolAtom>().unwrap());
}
