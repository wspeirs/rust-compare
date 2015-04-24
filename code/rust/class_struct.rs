// BEGIN_CODE
struct MyClass {
	my_field: int
}

impl MyClass {
	// constructor, by convention
	pub fn new(field: int) -> MyClass {
		MyClass { my_field: field }
	}

	// destructor
	pub fn drop(&self) {
	}
}
// END_CODE
