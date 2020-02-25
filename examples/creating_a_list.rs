extern crate kdb;

use kdb::KIntList;

fn main() {
    //Create a list containing the numbers from 1 to 10.
    let mut list: KIntList = (1..=10i32).into_iter().collect();

    //Create another list by pushing incrementally
    let mut list_2 = KIntList::new();
    for i in 11..=20 {
        list_2.push(i);
    }

    //Append the second list to the first
    list.extend(list_2);

    // write out the contents
    for i in list.iter() {
        println!("{}", i);
    }

    // we can also use it as a slice:
    for i in &list[5..6] {
        println!("{}", i)
    }
}
