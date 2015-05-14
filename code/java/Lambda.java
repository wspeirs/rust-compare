import java.lang.Math;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

public class Lambda {
    public static void main(String[] args) {
// BEGIN_CODE
    List<Integer> vals = Arrays.asList(-1, -2, -3, 0, 1, 2, 3);
    int p = 2;

    Collections.sort(vals, (a, b) -> 
            Integer.compare(Math.pow(a, p), Math.pow(b, p)));

// END_CODE
    }
}
