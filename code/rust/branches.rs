#[allow(unused_variables)]
fn main() {
// BEGIN_CODE
    let var = 3;

    if var < 0 {
        println!("Var is < 0");
    } else if var == 0 {
        println!("Var is 0");
    } else {
        println!("Var is > 0");
    }

    let gt_0 = if var > 0 { true } else { false };

    match var {
        1 => println!("Var is 1"),
        2 => println!("Var is 2"),
        3 => println!("Var is 3"),
        _ => println!("Var is unknown"),
    }

// END_CODE
}

