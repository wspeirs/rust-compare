#include <iostream>

#define unused __attribute ((unused))

using namespace std;

int main(unused int argc, unused char **argv) {
// BEGIN_CODE
	const int var = 3;

    if(var < 0) {
        cout << "Var is < 0" << endl;
    } else if(var == 0) {
        cout << "Var is 0" << endl;
    } else {
        cout << "Var is > 0" << endl;
    }

    switch(var) {
    case 1:
        cout << "Var is 1" << endl;
        break;
    case 2:
        cout << "Var is 2" << endl;
        break;
    default:
        cout << "Var is unknown" << endl;
    }

// END_CODE
}

