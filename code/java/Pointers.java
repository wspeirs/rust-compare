import java.lang.Character;
import java.lang.ref.WeakReference;
import java.lang.ref.SoftReference;

public class Pointers {
    public static void main(String[] args)
// BEGIN_CODE
    {
        // normal garbage collected reference
        Character ref = new Character('a');

        // reference that does not impact garbage collection
        WeakReference<Character> ptr_c = new WeakReference<>(ref);

        // reference that is stronger than a weak reference
        SoftReference<Character> ref_s = new SoftReference<>(ref);

    }
// END_CODE
}

