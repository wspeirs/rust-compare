use std::collections::LinkedList;

fn main() {
// BEGIN_CODE
	let v = vec![5, 4, 3, 2, 1];
	let mut l = LinkedList::new();

    l.push_back(10);
    l.push_back(9);
    l.push_back(8);
    l.push_back(7);
    l.push_back(6);

    // sort the two
    v.sort();
    //l.sort();

    // print them out
    for i in v.iter() {
        print!("{},", i);
    }

    for i in l.iter() {
        print!("{},", i);
    }
// END_CODE
}

