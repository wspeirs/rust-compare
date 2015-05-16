public class Branches {

	public static void main(String[] args) {
// BEGIN_CODE
		final int var = 3;

        if(var < 0) {
            System.out.println("Var is < 0");
        } else if(var == 0) {
            System.out.println("Var is 0");
        } else {
            System.out.println("Var is > 0");
        }

        switch(var) {
        case 1:
            System.out.println("Var is 1");
            break;
        case 2:
            System.out.println("Var is 2");
            break;
        case 3:
            System.out.println("Var is 3");
            break;
        default:
            System.out.println("Var is unknown");
        }

// END_CODE
	}
}
