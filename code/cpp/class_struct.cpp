// BEGIN_CODE
class MyClass {
private:
	const int my_field;

public:
	// constructor
	MyClass(const int &field) : my_field(filed) { }

	// destructor
	~MyClass() { }

	// friend functions
	ostream& operator<<(ostream &out, const MyClass &my_class) {
		out << my_field;
	}
}
// END_CODE

