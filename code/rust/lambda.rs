
#[allow(unused_variables)]
fn main() {
// BEGIN_CODE
    let mut vals = [-1, -2, -3, 1, 2, 3, 0];
    let p = 2;

    vals.sort_by(|a :&isize, b :&isize|
                    a.pow(p).cmp(&b.pow(p)));

// END_CODE
}

