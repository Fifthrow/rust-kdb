use kdb::{Atom, Connection, ConnectionError, KBox, List, Symbol};
use std::convert::TryFrom;

fn main() -> Result<(), ConnectionError> {
    let conn = Connection::connect("127.0.0.1", 4200, "", None)?;
    let result: KBox<Atom<i64>> = KBox::try_from(conn.eval("2+2").unwrap()).unwrap();
    println!("{}", result.value());

    //Returning a list of symbols
    let result: KBox<List<Symbol>> = KBox::try_from(conn.eval("`a`b`c").unwrap()).unwrap();
    println!("{:?}", &result[0..]);

    Ok(())
}
