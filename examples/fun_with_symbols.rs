extern crate kdb;

use kdb::{KSymbol, KSymbolAtom};
use std::convert::TryFrom;

fn main() {
    //Create two identical symbols in different ways, and check that they are equal.
    let sym = KSymbol::try_from("Hello, World").unwrap();
    //Note: converting a string into a symbol is not an infallible operation:
    let sym_2 = KSymbol::try_from(String::from("Hello") + ", World").unwrap();
    assert_eq!(sym, sym_2);

    // As an atom:
    let atom: KSymbolAtom = sym.into();
    let atom_2: KSymbolAtom = KSymbolAtom::try_from(String::from("Hello") + ", World").unwrap();

    assert_eq!(*atom, *atom_2);

    println!("{}", sym);
    //Symbols can be converted into strings
}
