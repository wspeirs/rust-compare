#define unused __attribute ((unused))

#include <vector>
#include <algorithm>

using namespace std;

int main(unused int argc, unused char **argv) {
// BEGIN_CODE
    vector<int> vals = {-1, -2, -3, 1, 2, 3, 0};
    int p = 2;

    sort(vals.begin(),
         vals.end(),
         [p](int a, int b) {
            return pow(a, p) < pow(b, p);
            });

// END_CODE

    return 0;
}

