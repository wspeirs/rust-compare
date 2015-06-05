#define unused __attribute ((unused))

#include <iostream>

using namespace std;

// BEGIN_CODE
int multiply(int arg1, int &arg2)
{
    int ret = arg1 * arg2;

    arg1 = ret;
    arg2 = ret;

    return ret;
}

int main(unused int argc, unused char **argv)
{
    int arg1 = 2;
    int arg2 = 3;

    int ret = multiply(arg1, arg2);

    // prints 2 because it's passed by value
    cout << "ARG1: " << arg1 << endl;

    // prints 6 because it's passed by reference
    cout << "ARG2: " << arg2 << endl;

    // prints 6
    cout << "RET: " << ret << endl;

    return 0;
}
// END_CODE

