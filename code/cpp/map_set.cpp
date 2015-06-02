#include <map>
#include <set>
#include <iostream>
#include <algorithm>
#include <iterator>

using namespace std;

#define unused __attribute ((unused))

int main(unused int argc, unused char **argv) {
// BEGIN_CODE
    map<char, int> m {{'a', 5}, {'b', 3}, {'c', 1}};
    set<int> s = {9, 8, 7, 6};

    // inserts a new value for d
    m['d'] = 7;

    // prints the value for d
    cout << m['d'] << endl;

    // checks to see if 2 is in the set
    cout << (s.find(2) == s.end()) << endl;
// END_CODE
    return 0;
}
