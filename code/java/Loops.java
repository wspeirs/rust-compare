public class Loops {

	public static void main(String[] args) {
// BEGIN_CODE
		final int items[] = {1, 2, 3};

		for(int i=0; i<items.length; ++i)
			System.out.println(i);

		for(int i:items)
			System.out.println(i);

		int i=0;

		while(true) {
			System.out.println(i);
			++i;

			if(i==items.length)
				break;
		}

		i=0;
		do {
			System.out.println(i);
			++i;
		} while(i<items.length);
// END_CODE
	}
}
