extern crate kdb;

use kdb::{cast, symbol, Atom, KBox, Symbol};
fn main() {
    //Symbols use a lot of try_into. Sad.
    let mut dict = KBox::new_dict();

    dict.insert(symbol("One"), 1i32);
    dict.insert(symbol("Two"), 2i32);
    dict.insert(symbol("Three"), 3i32);

    println!("{:?}", cast!(&dict[symbol("Two")]; Atom<Symbol>));
}
