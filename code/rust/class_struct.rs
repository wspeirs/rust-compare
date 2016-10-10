// BEGIN_CODE
struct MyClass {
	my_field: isize
}

impl MyClass {
	// constructor, by convention
	pub fn new(field: isize) -> MyClass {
		MyClass { my_field: field }
	}
}

impl Drop for MyClass {
	// destructor
	fn drop(&mut self) {
	}
}
// END_CODE
