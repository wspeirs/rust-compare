// BEGIN_CODE
trait Interfaces {
    fn my_method(a: isize) -> isize;
}

#[allow(dead_code)]
struct Impl;

impl Interfaces for Impl {
    fn my_method(a: isize) -> isize {
        return a;
    }
}
// END_CODE

fn main() {
}
