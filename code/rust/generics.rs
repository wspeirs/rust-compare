// BEGIN_CODE
#[allow(dead_code)]
struct GenericStruct<T> {
	t: T
}

fn do_nothing<T>(t: T) -> T {
	return t;
}

#[allow(unused_variables)]
fn main() {
	let gc_int = GenericStruct{t: 2};
	let gc_float = GenericStruct{t: 2.3};

	let r1 = do_nothing(2);
	let r2 = do_nothing(2.3);
}
// END_CODE
