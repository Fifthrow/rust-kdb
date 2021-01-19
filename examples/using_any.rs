use kdb::{Any, Atom, KBox};
use std::convert::TryFrom; // <-- this trait is your best friend when you use this crate

fn main() {
    let int = KBox::new_atom(42);

    // convert to an "any" value:
    let any: KBox<Any> = int.into();

    // try to convert back to an i32 atom.
    let int = KBox::<Atom<i32>>::try_from(any).unwrap();
    println!("{:?}", int);

    let any: KBox<Any> = int.into();
    // try to convert to a u8 atom. This will fail!
    if let Err(e) = KBox::<Atom<u8>>::try_from(any) {
        println!("Error: {}", e);
    }
}
