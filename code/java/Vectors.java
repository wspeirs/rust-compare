import java.util.Arrays;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.LinkedList;

public class Vectors {
	public static void main(String[] args) {
// BEGIN_CODE
        List<Integer> v =
            new ArrayList<>(Arrays.asList(5, 4, 3, 2, 1));
        List<Integer> l =
            new LinkedList<>(Arrays.asList(10, 9, 8, 7, 6));

        // sort the two
        Collections.sort(v);
        Collections.sort(l);

        // print them out
        for(Integer i : v) {
            System.out.print(i);
            System.out.print(',');
        }

        for(Integer i : l) {
            System.out.print(i);
            System.out.print(',');
        }
// END_CODE
	}
}
