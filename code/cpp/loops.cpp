#include <iostream>

#define unused __attribute ((unused))

using namespace std;

int main(unused int argc, unused char **argv) {
// BEGIN_CODE
	const int items_size = 3;
	const int items[items_size] = {1, 2, 3};

	for(int i=0; i<items_size; ++i)
		cout << items[i] << endl;

	for(int i:items)
		cout << i << endl;

	int i=0;

	while(true)
	{
		cout << items[i] << endl;

		if(++i == items_size)
			break;
	}

	i=0;
	do {
		cout << items[i] << endl;
		++i;
	} while(i<items_size);
// END_CODE
}

