#define unused __attribute ((unused))
int main(unused int argc, unused char **argv) {
// BEGIN_CODE
	enum basic_enum {
		option1 = 1,
		option2
	};

	enum class char_enum : char {
		optiona = 'a',
		optionb
	};

	basic_enum e1 = option1;
	char_enum e2 = char_enum::optiona;

	if(e1 == option2)
		e2 = char_enum::optionb;

	if(e2 == char_enum::optionb)
		e1 = option2;

// END_CODE
	return 0;
}
