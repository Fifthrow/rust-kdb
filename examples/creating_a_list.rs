use kdb::{KBox, List};

fn main() {
    //Create a list containing the numbers from 1 to 10.
    let mut list: KBox<List<u8>> = (1..=10).collect();

    //Create another list by pushing incrementally
    let mut list_2: KBox<List<u8>> = KBox::new_list();
    for i in 11..=20 {
        list_2.push(i);
    }

    //Append the second list to the first
    list.join(list_2);

    // Append from an iterator:
    list.extend(21..=30);

    // write out the contents
    for i in list.iter() {
        println!("{}", i);
    }

    // we can also use it as a slice:
    for i in &list[..5] {
        println!("{}", i)
    }
}
