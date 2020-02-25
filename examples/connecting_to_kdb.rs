extern crate kdb;
use kdb::{Connection, KItem, KLongAtom, KSymbolList};
use std::convert::TryFrom;

fn main() -> Result<(), kdb::ConnectionError> {
    let conn = Connection::connect("127.0.0.1", 4200, "", None)?;
    let result = KLongAtom::try_from(conn.eval("2+2").unwrap()).unwrap();
    println!("{}", *result);

    //Returning a list of symbols
    let result = KSymbolList::try_from(conn.eval("`a`b`c").unwrap()).unwrap();
    println!("{}", result.k_type());
    println!("{:?}", &result[0..]);

    Ok(())
}
