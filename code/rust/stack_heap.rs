use std::mem;

#[allow(unused_variables)]
fn main()
// BEGIN_CODE
{
    // single char allocated on the stack
    let a: char = 'a';

    // single char allocated on the heap
    // 
    let b = Box::new('b');

    // array of 23 chars allocated on the heap
    // char pointer allocated on the stack
    let c = Box::new(['c'; 23]);

    // heap allocated memory is freed when the
    // variable goes out of scope
    // however, you can force freeing by using
    // the mem::drop method
    mem::drop(b);
    mem::drop(c);
}
// END_CODE

