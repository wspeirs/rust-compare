fn main() {
// BEGIN_CODE
	let items = [1, 2, 3];

	for i in 0..items.len() {
		println(items[i]);
	}

	for i in items {
		println(i);
	}

	let i = 0;

	loop {
		println(items[i]);

		if(++i==items.len())
			break;
	}

	// Rust does not have do-while loops
// END_CODE
}

