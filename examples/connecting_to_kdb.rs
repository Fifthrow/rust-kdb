use kdb::{cast, Atom, Connection, ConnectionError, List, Symbol};

fn main() -> Result<(), ConnectionError> {
    let conn = Connection::connect("127.0.0.1", 4200, "", None)?;
    let result = cast!(conn.eval("2+2").unwrap(); Atom<i64>);
    println!("{}", result.value());

    //Returning a list of symbols
    let result = cast!(conn.eval("`a`b`c").unwrap(); List<Symbol>);
    println!("{:?}", &result[0..]);

    Ok(())
}
