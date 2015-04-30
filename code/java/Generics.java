public class Generics {
// BEGIN_CODE
	public static class GenericClass<T> {
		private T t;
		
		public GenericClass(T t) {
			this.t = t;
		}
	}

	public static <T> T do_nothing(T t) {
		return t;
	}

	public static void main(String[] args) {
		GenericClass<Integer> gc_int
			= new GenericClass<>(2);
		GenericClass<Float> gc_float
			= new GenericClass<>(2.3f);

		int r1 = do_nothing(2);
		float r2 = do_nothing(2.3f);
	}
// END_CODE
}
