// BEGIN_CODE
class Interfaces {
public:
	virtual int my_method(int a) = 0;
};

class Impl : public Interfaces {
public:
	virtual int my_method(int a) {
		return a;
	}
};
// END_CODE
