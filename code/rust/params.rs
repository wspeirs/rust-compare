#[allow(unused_assignments)]
// BEGIN_CODE
fn multiply(mut arg1: isize, arg2: &mut isize) -> (isize)
{
    let ret = arg1 * *arg2;

    arg1 = ret;
    *arg2 = ret;

    return ret;
}

fn main()
{
    let arg1: isize = 2;
    let mut arg2: isize = 3;

    let ret = multiply(arg1, &mut arg2);

    // prints 2 because it's passed by value
    println!("ARG1: {}", arg1);

    // prints 6 because it's passed by mutable reference
    println!("ARG2: {}", arg2);

    // prints 6
    println!("RET: {}", ret);
}
// END_CODE

