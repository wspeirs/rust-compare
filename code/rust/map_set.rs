use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
// BEGIN_CODE
	let mut m = HashMap::new();
	let mut s = HashSet::new();

    // all initialization is done the same way
    m.insert('a', 5);
    m.insert('b', 3);
    m.insert('c', 1);
    m.insert('d', 7);

    s.insert(9);
    s.insert(8);
    s.insert(7);
    s.insert(6);

    // prints the value for d
    println!("{}", m.get(&'d').unwrap());

    // checks to see if 2 is in the set
    println!("{}", s.contains(&2));
// END_CODE
}

