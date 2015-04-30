// BEGIN_CODE
template<typename T>
class generic_class {
	T t;
public:
	generic_class(const T &t)
		: t(t) { }
};

template<typename T>
T do_nothing(const T &t) {
	return t;
}

int main(int argc, char **argv) {
	generic_class<int> gc_int(2);
	generic_class<float> gc_float(2.3);

	int r1 = do_nothing(2);
	float r2 = do_nothing(2.3);

	return 0;
}
// END_CODE
