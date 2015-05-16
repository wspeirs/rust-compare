// BEGIN_CODE
fn safe_div(n: f64, d: f64) -> Result<f64, &'static str> {
    if d == 0.0 {
        return Err("Divide by zero");
    } else {
        return Ok(n/d);
    }
}

fn main() {
    match safe_div(1.0, 0.0) {
        Ok(v) => { println!("{}", v); },
        Err(err) => { println!("{}", err); }
    }
}
// END_CODE

