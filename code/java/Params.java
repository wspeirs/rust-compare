
public class Params {
// BEGIN_CODE
    public static int multiply(int arg1, int arg2)
    {
        int ret = arg1 * arg2;

        arg1 = ret;
        arg2 = ret;

        return ret;
    }

	public static void main(String[] args)
    {
        int arg1 = 2;
        int arg2 = 3;

        int ret = multiply(arg1, arg2);

        // prints 2 because it's passed by value
        System.out.println("ARG1: " + arg1);

        // prints 3 because it's passed by value
        System.out.println("ARG2: " + arg2);

        // prints 6
        System.out.println("RET: " + ret);
	}
// END_CODE
}
