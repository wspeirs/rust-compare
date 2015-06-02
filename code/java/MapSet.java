import java.util.Arrays;
import java.util.Map;
import java.util.HashMap;
import java.util.Set;
import java.util.HashSet;

public class MapSet {
	public static void main(String[] args) {
// BEGIN_CODE
        Map<Character, Integer> m = new HashMap<>();
        Set<Integer> s =
            new HashSet<>(Arrays.asList(9, 8, 7, 6));

        // all initialization is done the same way
        m.put('a', 5);
        m.put('b', 3);
        m.put('c', 1);
        m.put('d', 7);

        // prints the value for d
        System.out.println(m.get('d'));

        // checks to see if 2 is in the set
        System.out.println(s.contains(2));
// END_CODE
	}
}
