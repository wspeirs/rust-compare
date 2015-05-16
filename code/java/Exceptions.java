public class Exceptions {
// BEGIN_CODE
    public static double safe_div(int n, int d)
        throws IllegalArgumentException {

        if(d == 0)
            throw new IllegalArgumentException("Divide by zero");

        return n/d;
    }

	public static void main(String[] args) {
        try {
            System.out.println(safe_div(1, 0));
        } catch(IllegalArgumentException err) {
            System.err.println(err.getMessage());
        }
	}
// END_CODE
}
