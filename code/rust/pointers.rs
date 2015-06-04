use std::rc::Rc;
use std::sync::Arc;

#[allow(unused_variables)]
fn main()
// BEGIN_CODE
{
    // memory freed when ptr_a goes out of scope
    let ptr_a = Box::new('a');

    // reference counted pointer
    let ptr_b = Rc::new('b');

    // pointer that does not increase reference counts
    let ptr_c = ptr_b.downgrade();

    // atomically reference counted pointer
    // can be safely used across threads
    let ptr_e = Arc::new('e');
}
// END_CODE

