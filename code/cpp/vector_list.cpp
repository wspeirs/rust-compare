#include <vector>
#include <list>
#include <iostream>
#include <algorithm>
#include <iterator>

using namespace std;

#define unused __attribute ((unused))

int main(unused int argc, unused char **argv) {
    vector<int> v = {5, 4, 3, 2, 1};
    list<int> l = {10, 9, 8, 7, 6};

    // sort the two
    sort(v.begin(), v.end());
    l.sort(); // use container specific sort

    // print them out
    copy(v.begin(), v.end(), ostream_iterator<int>(cout, "\n"));
    copy(l.begin(), l.end(), ostream_iterator<int>(cout, "\n"));

    return 0;
}
