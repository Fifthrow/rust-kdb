use kdb::{symbol, KBox, Symbol};

fn main() {
    //Create two identical symbols in different ways, and check that they are equal.
    let sym = symbol("Hello, World");
    // Note: converting a string into a symbol is not an infallible operation
    // rust strings can contain embedded nuls, whereas symbols cannot.
    let sym_2 = Symbol::new(String::from("Hello") + ", World").unwrap();
    assert_eq!(sym, sym_2);

    // As an atom:
    let atom = KBox::new_atom(sym);
    let atom_2 = KBox::new_atom(Symbol::new(String::from("Hello") + ", World").unwrap());

    assert_eq!(atom.value(), atom_2.value());

    // We cannot directly display a KDB symbol as, unlike like rust strings, they do not have to be
    // Utf8 encoded.
    println!("{}", sym.try_as_str().unwrap());

    //As a compromise we can use debug, which will attempt to display it or display
    // <invalid rust string> if it is invalid.
    println!("{:?}", sym);
}
