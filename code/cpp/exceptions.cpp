#include <iostream>
#include <string>

#define unused __attribute ((unused))

using namespace std;

// BEGIN_CODE
double safe_div(int n, int d) throw(string)
{
    if(d == 0)
        throw string("Divide by zero");

    return n/d;
}

int main(unused int argc, unused char **argv) {
    try {
        cout << safe_div(1, 0) << endl;
    } catch(string &err) {
        cerr << err << endl;
    }

    return 0;
}
// END_CODE

