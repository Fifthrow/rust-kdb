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

    // Note that because rust strings are utf-8, and symbols have no encoding requirement,
    // this may not display the same way as you will see it in kdb, especially if the string is
    // not a valid ASCII or utf-8 string.
    println!("{}", sym);
}
