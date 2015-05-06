#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
// BEGIN_CODE
	enum BasicEnum {
		Option1 = 1,
		Option2
	}

	enum CharEnum {
		OptionA(char),
		OptionB(char)
	}

	let mut e1 = BasicEnum::Option1;
	let mut e2 = CharEnum::OptionA('a');

	match e1 {
		BasicEnum::Option1 => e1 = BasicEnum::Option2,
		_ => {},
	}

	match e2 {
		CharEnum::OptionB(..) => e2 = CharEnum::OptionA('a'),
		_ => {}
	}
// END_CODE
}
