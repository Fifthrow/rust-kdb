use kdb::{list, symbol, Connection};

/// Note you'll need a kdb instance on localhost:4200 with the function "upd" defined for this to work...
fn main() {
    let conn = Connection::connect("127.0.0.1", 4200, "", None).unwrap();
    let l = list![i32; 1, 2, 3];
    if let Err(err) = conn.publish("upd", symbol("some_topic"), l) {
        println!("Publishing failed: {}", err);
    }
    println!("Publish succeeded! Probably.");
}
