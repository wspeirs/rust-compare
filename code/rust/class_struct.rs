// BEGIN_CODE
struct MyClass {
	my_field: isize
}

impl MyClass {
	// constructor, by convention
	pub fn new(field: isize) -> MyClass {
		MyClass { my_field: field }
	}

	// destructor
	pub fn drop(&self) {
	}
}
// END_CODE
