#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
// BEGIN_CODE
    #[derive(PartialEq, Eq)]
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

    if e1 == BasicEnum::Option2 {
        e2 = CharEnum::OptionB('b');
    }

    match e2 {
        CharEnum::OptionB(..) => e1 = BasicEnum::Option2,
        _ => {}
    }
// END_CODE
}
