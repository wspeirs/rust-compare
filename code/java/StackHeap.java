public class StackHeap {
	public static void main(String[] args)
// BEGIN_CODE
    {
        // single char allocated on the stack
        char a = 'a';

        // Character object allocated on the heap
        // Character reference allocated on the stack
        Character b = new Character('b');

        // array of 23 chars allocated on the heap
        // char reference allocated on the stack
        char[] c = new char[23];

        // no way to explicitly free memory in Java
        // best we can do is set to null and wait for
        // the garbage collector
        b = null;
        c = null;
	}
// END_CODE
}
