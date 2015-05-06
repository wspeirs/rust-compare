public class Enum {
// BEGIN_CODE
	enum BasicEnum {
		Option1,
		Option2
	}

	enum CharEnum {
		OptionA('a'),
		OptionB('b');

		private char v;

		CharEnum(char v) {
			this.v = v;
		}
	}

	public static void main(String[] args) {
		BasicEnum e1 = BasicEnum.Option1;
		CharEnum e2 = CharEnum.OptionA;

		if(e1.equals(BasicEnum.Option2))
			e2 = CharEnum.OptionB;

		if(e2.equals(CharEnum.OptionB))
			e1 = BasicEnum.Option2;
	}
// END_CODE
}
